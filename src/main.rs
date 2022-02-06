use dotenv::dotenv;
use reqwest::Error;
use std::io::Read;
use tokio;
mod api;

#[tokio::main]
async fn main() {
    println!("LOADING ENV...");
    dotenv().ok();
    let file = std::fs::File::open("./template.docx").unwrap();
    let bytes = std::io::BufReader::new(file)
        .bytes()
        .map(|b| b.unwrap())
        .collect::<Vec<u8>>();

    let result = test_fetch().await.unwrap();
    assert_eq!(result, bytes);
    // test_put().await;
}

async fn test_fetch() -> Result<Vec<u8>, Error> {
    api::fetch::fetch_file("test").await
}
async fn test_put() {
    let file = std::fs::File::open("./template.docx").unwrap();
    let bytes = std::io::BufReader::new(file)
        .bytes()
        .map(|b| b.unwrap())
        .collect::<Vec<u8>>();
    api::put::put("test", bytes).await;
}
