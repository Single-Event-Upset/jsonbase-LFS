use cached::proc_macro::cached;

const URL: &str = "https://jsonbase.com";

pub fn get_url(key: &str) -> String {
    // get env variable KEY
    format!("{URL}/{key}", URL = get_base_url(), key = key)
}

#[cached]
pub fn get_base_url() -> String {
    let key = std::env::var("KEY").expect("KEY not found");
    format!("{URL}/{key}", URL = URL.to_string(), key = key)
}