use crate::conf::get_current_config_dir_path;
use crate::const_value::SETTINGS;
use crate::data_result::AppResult;
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::ServerConfig;

pub fn get_tls_config() -> AppResult<ServerConfig> {
    let cert_file = get_current_config_dir_path(&SETTINGS.open_ssl.certificate);
    let private_file = get_current_config_dir_path(&SETTINGS.open_ssl.private_key);


    let cert = CertificateDer::pem_file_iter(cert_file).expect("failed to read certificate")
        .map(|e| e.expect("failed to read certificate"))
        .collect();

    let private_key = PrivateKeyDer::from_pem_file(private_file).expect("failed to read private key");

    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert, private_key)?;
    Ok(config)
}