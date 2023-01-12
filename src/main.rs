mod config;
mod structs;

use clap::Parser;
use reqwest::Client as HttpClient;
use std::{net::Ipv4Addr, process::exit};
use structs::{Args, Ipify};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let config_path = match args.config {
        Some(x) => x,
        None => match config::path() {
            Ok(x) => x,
            Err(e) => {
                println!("An unexpected error occurred while trying to get the path of the configuration file.\n\n{}", e);
                exit(1)
            }
        },
    };

    if args.configuration {
        println!("{}", config_path.display());
        return;
    }

    let config = match config::get(config_path) {
        Ok(x) => x,
        Err(_) => {
            println!("An error occurred while parsing the configuration.\nPlease consult the readme for an example configuration.");
            exit(1)
        }
    };
    println!("api token: \t\"{}\"", config.api_token);
    println!("zone name: \t\"{}\"", config.zones[0].name);
    println!("zone record: \t\"{}\"", config.zones[0].records[1]);

    let http: HttpClient = HttpClient::new();

    let ipv4: Ipv4Addr = determine_ipv4(http).await.unwrap();
    println!("ip: {}", ipv4);
}

async fn obtain_zone_id(http: HttpClient, zone_name: String) -> Result<String, reqwest::Error> {
    todo!()
}

async fn obtain_record_id(
    http: HttpClient,
    zone_id: String,
    zone_name: String,
    record_name: String,
) -> Result<String, reqwest::Error> {
    todo!()
}

async fn update_ip(http: HttpClient) {
    todo!()
}

async fn determine_ipv4(http: HttpClient) -> Result<Ipv4Addr, reqwest::Error> {
    let response = http
        .get("https://api.ipify.org?format=json")
        .send()
        .await?
        .json::<Ipify>()
        .await?;

    Ok(response.ip)
}

async fn determine_ipv6() {
    todo!()
}
