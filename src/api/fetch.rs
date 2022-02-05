use std::str::FromStr;
use serde::{Deserialize, Serialize};
use serde_json::json;
use reqwest::{Client, Url, Error};
use crate::api::get_url;

pub async fn fetch(key: &str) -> Result<String, Error> {
    let client = Client::new();
    let mut res = client.get(Url::from_str(&*get_url(key)).unwrap()).send().await?;
    let body = res.text().await?;
    print!("{}", body);
    Ok(body)
}