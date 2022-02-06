use base64::{decode, encode};
use cached::proc_macro::cached;
use serde::{Deserialize, Serialize};
const URL: &str = "https://jsonbase.com";

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub(crate) data: String,
}

#[derive(Deserialize, Serialize)]
pub struct Header {
    key: String,
    pub(crate) length: i32,
}

impl Header {
    pub(crate) fn new(key: &str, length: &i32) -> Self {
        Header {
            key: key.to_string(),
            length: length.to_owned(),
        }
    }

    pub(crate) fn json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Data {
    pub(crate) fn new(data: &[u8]) -> Self {
        Data {
            data: encode(data.to_vec()),
        }
    }

    pub(crate) fn get_data(&self) -> Vec<u8> {
        decode(&self.data).unwrap()
    }

    pub(crate) fn json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
pub fn get_url(key: &str) -> String {
    // get env variable KEY
    format!("{URL}/{key}", URL = get_base_url(), key = key)
}

#[cached]
pub fn get_base_url() -> String {
    let key = std::env::var("KEY").expect("KEY not found");
    format!("{URL}/{key}", URL = URL.to_string(), key = key)
}

pub async fn get_json(url: &str) -> Result<Data, reqwest::Error> {
    let body = get(url).await?;
    let data: Data = serde_json::from_str(&body).unwrap();
    Ok(data)
}

pub async fn get_header(url: &str) -> Result<Header, reqwest::Error> {
    let body = get(url).await?;
    let data: Header = serde_json::from_str(&body).unwrap();
    Ok(data)
}

pub async fn put_json(url: &str, data: Data) -> Result<String, reqwest::Error> {
    let body = serde_json::to_string(&data).unwrap();
    put(url, body).await
}
pub async fn put_header(url: &str, data: Header) -> Result<String, reqwest::Error> {
    let body = serde_json::to_string(&data).unwrap();
    put(url, body).await
}

async fn put(url: &str, body: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut res = client
        .put(url)
        .body(body)
        .header("content-type", "application/json")
        .send()
        .await?;
    let body = res.text().await?;
    Ok(body)
}

async fn get(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;
    let body = res.text().await?;
    Ok(body)
}
