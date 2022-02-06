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
    let mut requests = Vec::with_capacity(header.length as usize);
    for _ in 0..header.length {
        let url = get_url(&format!("{}_{}", key, i));
        i += 1;
        requests.push(async move {
            let res = get_json(&url).await.unwrap();
            (i, res.get_data())
        })
    }
    let stream = stream::iter(requests).buffer_unordered(CONCURRENT_REQUESTS);
    let mut responses: Vec<(usize, Vec<u8>)> = stream.collect::<Vec<_>>().await;
    responses.sort_by_key(|(i, _)| *i);

    let data_vec: Vec<u8> = responses
        .iter()
        .map(|(_, data)| data.to_owned())
        .flatten()
        .collect();

    Ok(data_vec)
}
