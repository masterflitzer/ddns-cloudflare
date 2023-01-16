pub(crate) mod config;
pub(crate) mod structs;

use clap::Parser;
use reqwest::{Client as HttpClient, Response, Url};
use serde::de::DeserializeOwned;
use std::{
    error::Error, io::Error as IOError, io::ErrorKind, net::Ipv4Addr, process::exit,
    result::Result as StdResult, vec,
};
use structs::{
    cloudflare::response::{ListDnsRecords, ListZone},
    cloudflare::{Cloudflare, CloudflareResultHashMap, CloudflareResultVector, RecordType},
    Args, Ipify, RecordIds,
};

type Result<T> = StdResult<T, Box<dyn Error>>;

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
        Err(e) => {
            println!("An error occurred while parsing the configuration.\nPlease consult the readme for an example configuration.\n\n{}", e);
            exit(1)
        }
    };

    let http: HttpClient = HttpClient::new();
    let api_base: Url = Url::parse("https://api.cloudflare.com/client/v4/").unwrap();

    let ipv4: Ipv4Addr = determine_ipv4(&http).await.unwrap();

    let api_zones = api_base.join("zones").unwrap();
    for zone in config.zones {
        let response = api_get(&http, &api_zones, &config.api_token).await.unwrap();

        let data = match deserialize_response::<CloudflareResultVector<ListZone>>(
            response, &zone.name, "zone",
        )
        .await
        {
            Some(x) => x,
            None => continue,
        };

        let zone_id = match obtain_zone_id(data, &zone.name).await {
            Some(x) => x,
            None => continue,
        };

        let api_records = api_base
            .join(format!("zones/{}/dns_records", zone_id).as_str())
            .unwrap();

        for record in zone.records {
            let response = api_get(&http, &api_records, &config.api_token)
                .await
                .unwrap();

            let data = match deserialize_response::<CloudflareResultVector<ListDnsRecords>>(
                response, &record, "record",
            )
            .await
            {
                Some(x) => x,
                None => continue,
            };

            let record_id =
                match obtain_record_ids(data, format!("{}.{}", record, zone.name).as_str()).await {
                    Some(x) => x,
                    None => continue,
                };
        }
    }

    println!("ip: {}", ipv4);
}

async fn determine_ipv4(http: &HttpClient) -> Result<Ipv4Addr> {
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

async fn api_get(http: &HttpClient, url: &Url, api_token: &String) -> Result<Response> {
    let response = http
        .get(url.to_owned())
        .bearer_auth(api_token)
        .send()
        .await?;
    Ok(response)
}

async fn deserialize_response<T>(response: Response, name: &str, type_: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    if !is_http_success(&response) {
        println!(
            "{}: Skipping {} because HTTP status code was not between 200-299",
            name, type_
        );
        return None;
    }

    let data = response.json::<Cloudflare>().await.ok()?;

    if !data.success {
        println!(
            "{}: Skipping {} because JSON payload did not contain {{ \"success\": true }}",
            name, type_
        );
        return None;
    }

    let result: T = serde_json::from_value(data.result).ok()?;

    Some(result)
}

async fn obtain_zone_id(data: CloudflareResultVector<ListZone>, zone_name: &str) -> Option<String> {
    let zone = data.result.into_iter().find(|x| x.name == zone_name)?;
    Some(zone.id)
}

async fn obtain_record_ids(
    data: CloudflareResultVector<ListDnsRecords>,
    record_name: &str,
) -> Option<RecordIds> {
    let records_filter = data.result.into_iter().filter(|x| x.name == record_name);
    let records_filter_v4 = records_filter.clone();
    let records_filter_v6 = records_filter;

    let record_ids_v4 = records_filter_v4
        .filter(|x| matches!(x.type_, RecordType::A))
        .map(|x| x.id)
        .collect::<Vec<_>>();

    let record_ids_v6 = records_filter_v6
        .filter(|x| matches!(x.type_, RecordType::Aaaa))
        .map(|x| x.id)
        .collect::<Vec<_>>();

    let record_ids = RecordIds {
        v4: record_ids_v4,
        v6: record_ids_v6,
    };

    Some(record_ids)
}

async fn update_ip(response: Response, zone_id: &str, record_id: &str) {
    todo!()
}

fn is_http_success(response: &Response) -> bool {
    response.status().is_success()
}

// fn create_generic_error() -> IOError {
//     IOError::from(ErrorKind::Other)
// }
