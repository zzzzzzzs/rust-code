extern crate reqwest;
use reqwest::Client;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.get("http://example.com")
        .send()?
        .text()?;
    println!("{}", res);

    Ok(())
}