mod structs;

use std;
// use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use reqwest::Client;
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() {
    let config: structs::Config = confy::load("ddns-cloudflare", None).unwrap();
    println!("{:#?}", config);
    println!(
        "{}",
        confy::get_configuration_file_path("ddns-cloudflare", None)
            .unwrap()
            .display()
    );

    let http: Client = reqwest::Client::new();
    let ipv4: Ipv4Addr = get_ipv4(http).await.unwrap();
    println!("ip: {}", ipv4);
}

async fn get_ipv4(http: Client) -> Result<Ipv4Addr, reqwest::Error> {
    let response = http
        .get("https://api.ipify.org?format=json")
        .send()
        .await?
        .json::<structs::Ipify>()
        .await?;
    Ok(response.ip)
}
