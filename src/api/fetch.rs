use crate::api::api::{get_header, get_json};
use crate::api::get_url;
use futures::stream::FuturesUnordered;
use futures::{stream, StreamExt};
use reqwest::{Client, Error, Url};
use std::str::FromStr;

const CONCURRENT_REQUESTS: usize = 32;
const BUFFER_SIZE: usize = 1024;

pub async fn fetch(key: &str) -> Result<String, Error> {
    let client = Client::new();
    let res = client
        .get(Url::from_str(&*get_url(key)).unwrap())
        .send()
        .await?;
    let body = res.text().await?;
    print!("{}", body);
    Ok(body)
}

pub async fn fetch_file(key: &str) -> Result<Vec<u8>, Error> {
    let header = get_header(&get_url(key)).await.unwrap();
    let mut data: Vec<Vec<u8>> = Vec::with_capacity(header.length as usize);
    let mut i = 1;
    let stream = stream::iter(0..header.length)
        .map(|_| {
            let url = get_url(&format!("{}_{}", key, i));
            i += 1;
            async move {
                let res = get_json(&url).await.unwrap();
                println!("{}", res.data);
                (i, res.get_data())
            }
        })
        .buffer_unordered(CONCURRENT_REQUESTS);
    let mut_data: Vec<Vec<u8>> = stream
        .map(|(i, x)| async move { x })
        .collect::<FuturesUnordered<_>>()
        .collect()
        .await;
    let data_vec: Vec<u8> = mut_data.into_iter().flatten().collect();
    Ok(data_vec)
}
