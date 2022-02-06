use dotenv::dotenv;
use std::io::Read;
use tokio;
mod api;

#[tokio::main]
async fn main() {
    println!("LOADING ENV...");
    dotenv().ok();
    test_fetch().await;
    // test_put().await;
}

async fn test_fetch() {
    api::fetch::fetch_file("test").await;
}
async fn test_put() {
    let file = std::fs::File::open("./template.docx").unwrap();
    let bytes = std::io::BufReader::new(file)
        .bytes()
        .map(|b| b.unwrap())
        .collect::<Vec<u8>>();
    api::put::put("test", bytes).await;
}
