pub mod test {
    use crate::const_value::SETTINGS;
    use crate::tools::time::now_utc_date_str;
    use base64::engine::general_purpose;
    use base64::{alphabet, engine, Engine};
    use derive_more::Display;
    use hmac::digest::Digest;
    use hmac::{Hmac, Mac};
    use log::debug;
    use serde::Serialize;
    use sha2::Sha256;
    use sha256::Sha256Digest;
    use uuid::{Timestamp, Uuid};


    pub fn build_entry(param: Vec<(&str, &str)>, append_token: &str) -> String {
        let mut entry_vec = param.into_iter().map(|(k, v)| {
            format!("{}{}{}", k, append_token, v)
        }).collect::<Vec<String>>();
        entry_vec.sort();
        entry_vec.into_iter().fold(String::new(), |mut a, b| {
            a.push_str(&b);
            a
        })
    }

    ///
    /// ACS3-HMAC-SHA256 Credential=LTAI5tLypk7Hwq2bL4tpDHTc,SignedHeaders=host;x-acs-action;x-acs-content-sha256;x-acs-date;x-acs-signature-nonce;x-acs-version,Signature=9e002c730a97c26f19c20d665b60be91a6e8038ee70b583ee35987f5a4939977
    ///
    pub fn build_auth_head<T: Serialize + CanHashBase64>(http_request_method: &str, canonical_uri: &str, mut query_params: &mut Vec<(String, String)>, mut common_headers: &mut Vec<(String, String)>, sts_param: &Option<T>) -> String {
        println!( "http_request_method = {http_request_method} canonical_uri = {canonical_uri} canonical_query_string = {:#?} canonical_headers = {:#?} "
            ,query_params
            ,common_headers
            );

        let config = &SETTINGS.aliyun_oss_config;
        query_params.sort();
        let canonical_query_string = query_params.to_append_str_with_encode("=", "&");
        common_headers.sort();
        let mut canonical_header_string = common_headers.to_append_lowercase_str(":", r"\n");
        canonical_header_string.push_str(r"\n");
        let canonical_headers = common_headers.just_headers().join(";");
        let hashed_request_payload = sts_param.to_hash_base16_lower();

        let canonical_request = format!(r"{http_request_method}\n{canonical_uri}\n{canonical_query_string}\n{canonical_header_string}\n{canonical_headers}\n{hashed_request_payload}");
        println!("canonical_request = {}", canonical_request);
        let string_to_sign = format!(r"{ALGORITHM}\n{}", canonical_request.to_hash_base16_lower());
        println!("string_to_sign = {}", string_to_sign);

        let signed_string = signature_method(string_to_sign);
        println!("signed_string = {}",signed_string);
        let credential = &config.access_key_id;
        let authorization = format!(r"{ALGORITHM} Credential={credential},SignedHeaders={canonical_headers},Signature={}", signed_string);
        println!("authorization = {}",authorization);
        authorization

    }

    trait  UrlEncoder {
        fn encode(&self)->String;
    }
    impl UrlEncoder for &str{
        fn encode(&self)->String{
            let mut result = String::new();
            for c in self.chars(){
                if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~'{
                    result.push(c);
                }else{
                    result.push_str(&format!("%{:02X}",c as u8));
                }
            }
            result
        }
    }

    pub trait SignatureFun{
        fn to_append_lowercase_str(self, append_token: &str, join_token: &str) -> String;
        fn to_append_str_with_encode(self, append_token: &str, join_token: &str) -> String;

        fn sort(self);
        fn just_headers(self) -> Vec<String>;
    }
    impl SignatureFun for &mut Vec<(String, String)> {
        fn to_append_lowercase_str(self, append_token: &str, join_token: &str) -> String {
            self.iter().map(|(k, v)| {
                format!("{}{}{}", k.clone().to_lowercase(), append_token, v)
            }).collect::<Vec<String>>().join(join_token)
        }

        fn to_append_str_with_encode(self, append_token: &str, join_token: &str) -> String {
            self.iter().map(|(k, v)| {
                format!("{}{}{}", k, append_token, v.as_str().encode())
            }).collect::<Vec<String>>().join(join_token)
        }

        fn sort(self) {
            self.sort_by(|a, b| {
                a.0.cmp(&b.0)
            })
        }


        fn just_headers(self) -> Vec<String> {
            self.iter().map(|(k, _)| {
                k.clone()
            }).collect()
        }
    }

    type HmacSha256 = hmac::Hmac<Sha256>;
    pub fn signature_method(string_to_sign: String) -> String {
        let key = &SETTINGS.aliyun_oss_config.access_key_secret;
        let mut mac = HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC can take key of any size");
        mac.update(string_to_sign.as_bytes());
        let bytes = mac.finalize().into_bytes();
        base16::encode_lower(&bytes)
    }

    pub fn build_common_heads<T: CanHashBase64>(body: &Option<T>) -> Vec<(&'static str, String)> {
        let config = &SETTINGS.aliyun_oss_config;
        let headers = vec![
            ("x-acs-action", "AssumeRole".to_string()),
            ("x-acs-version", "2015-04-01".to_string()),
            ("x-acs-signature-nonce", Uuid::new_v4().to_string()),
            ("x-acs-date", now_utc_date_str("%Y-%m-%dT%H:%M:%SZ")),
            ("host", config.sts_host.to_string()),
            ("x-acs-content-sha256", body.to_hash_base16_lower()),
            ("Content-Type", "application/json".to_string()),
        ];
        debug!("headers = {:#?}", headers);
        headers
    }


    pub fn build_auth_sts_body() -> StsParam<'static> {
        let config = &SETTINGS.aliyun_oss_config;
        StsParam {
            role_arn: &config.sts_role_arn,
            role_session_name: "session-name",
        }
    }

    const ALGORITHM: &str = "ACS3-HMAC-SHA256";

    trait Baase16Encoder {
        fn to_hash_base16_lower(&self)->String;
    }

    impl<T: CanHashBase64> Baase16Encoder for &Option<T> {
        fn to_hash_base16_lower(&self) -> String {
            let bytes = match self {
                None => { "".to_string() }
                Some(value) => { value.to_json_str() }
            };
            let digest_json = sha2::Sha256::digest(bytes.as_bytes());
            base16::encode_lower(&digest_json[..])
        }
    }

    impl Baase16Encoder for String{
        fn to_hash_base16_lower(&self) -> String {
            let digest_json = sha2::Sha256::digest(self.as_bytes());
            base16::encode_lower(&digest_json[..])
        }
    }
    trait CanHashBase64: Baase16Encoder {
        fn to_json_str(&self) -> String;
    }
    impl Baase16Encoder for StsParam<'_> {
        fn to_hash_base16_lower(&self)->String{
            let json = self.to_json_str();
            //hash我的json
            let shaed_bytes = sha2::Sha256::digest(json.as_bytes());
            base16::encode_lower(&shaed_bytes[..])
        }
    }

    impl CanHashBase64 for StsParam<'_> {
        fn to_json_str(&self) -> String {
            let json = serde_json::to_string(self).unwrap();
            debug!("json = {}", json);
            json
        }
    }

    #[derive(Debug,Serialize)]
    #[serde(rename_all = "PascalCase")]
pub struct StsParam<'a> {
        role_arn: &'a str,
        role_session_name: &'a str,
    }
}