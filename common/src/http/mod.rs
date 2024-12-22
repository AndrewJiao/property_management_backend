use crate::data_result::AppResult;
use crate::error::APP_ERROR;
use reqwest::{Client, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;
use reqwest::header::HeaderMap;

#[cfg(feature = "oss")]
pub mod oss;

#[allow(unused)]
enum Method {
    GET,
    POST,
    PUT,
    //没有自己再加
}

pub trait GotData{}
pub struct Empty;
pub struct BeforeReq;
pub struct No;
pub struct AfterReq;

impl Operation for BeforeReq {}
impl Operation for AfterReq {}
impl Operation for No {}
impl GotData for Empty {}
impl GotData for Response{}
impl GotData for RequestBuilder{}

pub trait Operation {}
pub struct AppHttpClient<OPERATION = No, REQUEST = Empty, RESULT = Empty>
where
    OPERATION: Operation,
    RESULT: GotData,
    REQUEST: GotData,
{
    driver: REQUEST,
    result: RESULT,
    _phantom: PhantomData<OPERATION>
}
impl AppHttpClient {
    #[allow(unused)]
    pub fn get(url: &str) -> AppHttpClient<BeforeReq, reqwest::RequestBuilder>
    {
        AppHttpClient {
            driver: Client::new().request(reqwest::Method::GET, url),
            result: Empty,
            _phantom: PhantomData,
        }
    }
    #[allow(unused)]
    pub fn post(url: &str) -> AppHttpClient<BeforeReq, reqwest::RequestBuilder>
    {
        AppHttpClient {
            driver: Client::new().request(reqwest::Method::POST, url),
            result: Empty,
            _phantom: PhantomData,
        }
    }
    #[allow(unused)]
    pub fn put(url: &str) -> AppHttpClient<BeforeReq, reqwest::RequestBuilder>
    {
        AppHttpClient {
            driver: Client::new().request(reqwest::Method::PUT, url),
            result: Empty,
            _phantom: PhantomData,
        }
    }
}
type BeforeSendClient = AppHttpClient<BeforeReq, RequestBuilder>;
impl AppHttpClient<BeforeReq, RequestBuilder> {
    #[allow(unused)]
    pub fn header(self, key: &str, value: &str) -> BeforeSendClient
    {
        AppHttpClient {
            driver: self.driver.header(key, value),
            result: self.result,
            _phantom: PhantomData,
        }
    }
    #[allow(unused)]
    pub fn headers(self, headers: &Vec<(&'static str, String)>) -> AppHttpClient<BeforeReq, RequestBuilder>
    {
        let headers = headers.iter().fold(HeaderMap::new(), |mut a, b| {
            a.append(b.0, b.1.as_str().parse().expect("parse header error"));
            a
        });

        AppHttpClient {
            driver: self.driver.headers(headers),
            result: self.result,
            _phantom: PhantomData,
        }
    }
    #[allow(unused)]
    pub fn query<T>(self, query: &T) -> AppHttpClient<BeforeReq, RequestBuilder>
    where
        T: Serialize + ?Sized,
    {
        AppHttpClient {
            driver: self.driver.query(query),
            result: self.result,
            _phantom: PhantomData,
        }
    }

    #[allow(unused)]
    pub fn body(self, body: impl Into<reqwest::Body>) -> AppHttpClient<BeforeReq, RequestBuilder>
    {
        AppHttpClient {
            driver: self.driver.body(body),
            result: self.result,
            _phantom: PhantomData,
        }
    }

    #[allow(unused)]
    pub fn json<T: Serialize>(self, json: &T) -> BeforeSendClient {
        AppHttpClient {
            driver: self.driver.json(json),
            result: self.result,
            _phantom: PhantomData,
        }
    }

    #[allow(unused)]
    pub async fn send(self) -> AppResult<AfterResponse>
    {
        let response = self.driver.send().await?;
        println!("url = {}",response.url().to_string());
        Ok(AppHttpClient {
            driver: Empty,
            result: response,
            _phantom: PhantomData,
        })
    }
}
type AfterResponse = AppHttpClient<AfterReq, Empty, Response>;

impl AppHttpClient<AfterReq, Empty, Response> {
    #[allow(unused)]
    pub async fn json<T: DeserializeOwned>(self) -> AppResult<T> {
        self.result.json().await.map_err(|e| APP_ERROR(&*e.to_string()))
    }
    #[allow(unused)]
    pub async fn text(self) -> AppResult<String> {
        self.result.text().await.map_err(|e| APP_ERROR(&*e.to_string()))
    }
    #[allow(unused)]
    pub async fn bytes(self) -> AppResult<Vec<u8>> {
        self.result.bytes().await
            .map(|e| e.to_vec())
            .map_err(|e| APP_ERROR(&*e.to_string()))
    }
}