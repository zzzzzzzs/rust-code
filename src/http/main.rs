use std::collections::HashMap;
use reqwest::{Client, header};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use reqwest::header::USER_AGENT;

    let client = Client::new();
    let res = client.get("https://www.rust-lang.org")
        .header(USER_AGENT, "foo")
        .send()?;
}
