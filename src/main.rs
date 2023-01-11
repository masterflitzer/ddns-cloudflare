mod config;
mod structs;

use reqwest::Client as HttpClient;
use std::{error::Error, net::Ipv4Addr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = config::get_config().expect("An error occurred while parsing the configuration. Please consult the readme for an example configuration.");
    println!("api token: \t\"{}\"", config.api_token);
    println!("1st zone name: \t\"{}\"", config.zones[0].name);
    println!(
        "1st zone - 1st record: \t\"{}\"",
        config.zones[0].records[0]
    );

    let ipv4: Ipv4Addr = get_ipv4().await?;
    println!("ip: {}", ipv4);
    Ok(())
}

async fn get_ipv4() -> Result<Ipv4Addr, reqwest::Error> {
    let http: HttpClient = HttpClient::new();
    let response = http
        .get("https://api.ipify.org?format=json")
        .send()
        .await?
        .json::<structs::Ipify>()
        .await?;

    Ok(response.ip)
}
