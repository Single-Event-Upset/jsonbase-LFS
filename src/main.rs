use tokio;
use dotenv::dotenv;
mod api;


#[tokio::main]
async fn main() {
    println!("LOADING ENV...");
    dotenv().ok();
    test_fetch().await;
}

async fn test_fetch() {
    api::fetch::fetch("test").await;
}