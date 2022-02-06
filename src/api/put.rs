use crate::api::api::{put_header, Data, Header};
use crate::api::get_url;
use futures::{stream, StreamExt};
use reqwest::Client;
use std::io::Error;

const CONCURRENT_REQUESTS: usize = 32;
const BUFFER_SIZE: usize = 1024;

pub async fn put(key: &str, data: Vec<u8>) -> Result<(), Error> {
    let client = Client::new();
    let mut i = 0;

    let stream = stream::iter(data.chunks(BUFFER_SIZE)).map(|chunk| {
        let client = &client;
        i += 1;
        let url = get_url(&format!("{}_{}", key, i));
        println!("{}", url);
        async move {
            let body = Data::new(chunk);
            println!("{}", body.json());
            let res = client
                .put(url)
                .body(body.json())
                .header("content-type", "application/json")
                .send()
                .await
                .unwrap();
            res.text().await.unwrap()
        }
    });

    stream
        .buffer_unordered(CONCURRENT_REQUESTS)
        .for_each(|d| async move {
            println!("{}", d);
        })
        .await;
    let length = &i;
    {
        let client = &client;
        let _header = async move {
            let body = Header::new(key, length);
            put_header(&get_url(key), body).await
        }
        .await;
    }
    Ok(())
}
