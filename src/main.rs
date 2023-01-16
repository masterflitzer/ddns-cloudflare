pub(crate) mod config;
pub(crate) mod errors;
pub(crate) mod structs;

use clap::Parser;
use errors::{handle_errors, ErrorKind};
use reqwest::{Client as HttpClient, Response, Url};
use serde::de::DeserializeOwned;
use std::{net::Ipv4Addr, process::exit};
use structs::{
    cloudflare::response::{ListDnsRecords, ListZone},
    cloudflare::Cloudflare,
    Args, Ipify, RecordIds,
};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let config_path = match args.config {
        Some(x) => x,
        None => match config::path() {
            Ok(x) => x,
            Err(e) => {
                handle_errors(&ErrorKind::ConfigPath(e));
                exit(1);
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
            handle_errors(&ErrorKind::Config(e));
            exit(1)
        }
    };

    let http: HttpClient = HttpClient::new();
    let api_base: Url = match Url::parse("https://api.cloudflare.com/client/v4/") {
        Ok(x) => x,
        Err(e) => {
            handle_errors(&ErrorKind::Unknown(Box::new(e)));
            exit(1)
        }
    };

    let ipv4: Option<Ipv4Addr> = determine_ipv4(&http).await.ok();
    if let None = ipv4 {
        handle_errors(&ErrorKind::IPv4);
    }

    let api_zones = match api_base.join("zones") {
        Ok(x) => x,
        Err(e) => {
            handle_errors(&ErrorKind::Unknown(Box::new(e)));
            exit(1)
        }
    };

    for zone in config.zones {
        let response = match api_get(&http, &api_zones, &config.api_token).await {
            Ok(x) => x,
            Err(_) => {
                handle_errors(&ErrorKind::API);
                exit(1);
            }
        };

        let data = match deserialize_response::<Vec<ListZone>>(response, zone.name.clone()).await {
            Ok(x) => x,
            Err(e) => {
                handle_errors(&e);
                continue;
            }
        };

        let zone_id = match obtain_zone_id(data, &zone.name).await {
            Some(x) => x,
            None => continue,
        };

        let api_records = match api_base.join(format!("zones/{}/dns_records", zone_id).as_str()) {
            Ok(x) => x,
            Err(e) => {
                handle_errors(&ErrorKind::Unknown(Box::new(e)));
                exit(1)
            }
        };

        for record in zone.records {
            let response = match api_get(&http, &api_records, &config.api_token).await {
                Ok(x) => x,
                Err(_) => {
                    handle_errors(&ErrorKind::API);
                    exit(1);
                }
            };

            let data =
                match deserialize_response::<Vec<ListDnsRecords>>(response, record.clone()).await {
                    Ok(x) => x,
                    Err(e) => {
                        handle_errors(&e);
                        match e {
                            ErrorKind::NoSuccessHttp(_) | ErrorKind::NoSuccessJson(_) => continue,
                            _ => exit(1),
                        }
                    }
                };

            let record_id =
                match obtain_record_ids(data, format!("{}.{}", record, zone.name).as_str()).await {
                    Some(x) => x,
                    None => continue,
                };
        }
    }
}

async fn determine_ipv4(http: &HttpClient) -> Result<Ipv4Addr, reqwest::Error> {
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

async fn api_get(
    http: &HttpClient,
    url: &Url,
    api_token: &String,
) -> Result<Response, reqwest::Error> {
    let response = http
        .get(url.to_owned())
        .bearer_auth(api_token)
        .send()
        .await?;
    Ok(response)
}

async fn deserialize_response<T>(response: Response, name: String) -> Result<T, ErrorKind>
where
    T: DeserializeOwned,
{
    if !is_http_success(&response) {
        return Err(ErrorKind::NoSuccessHttp(name));
    }

    let data = response
        .json::<Cloudflare>()
        .await
        .map_err(|_| ErrorKind::JsonDeserialize)?;

    if !data.success {
        return Err(ErrorKind::NoSuccessJson(name));
    }

    let result =
        serde_json::from_value::<T>(data.result).map_err(|_| ErrorKind::JsonDeserialize)?;

    Ok(result)
}

async fn obtain_zone_id(data: Vec<ListZone>, zone_name: &str) -> Option<String> {
    let zone = data.into_iter().find(|x| x.name == zone_name)?;
    Some(zone.id)
}

async fn obtain_record_ids(data: Vec<ListDnsRecords>, record_name: &str) -> Option<RecordIds> {
    let records_filter = data.into_iter().filter(|x| x.name == record_name);
    let records_filter_v4 = records_filter.clone();
    let records_filter_v6 = records_filter;

    let record_ids_v4 = records_filter_v4
        .filter(|x| x.type_.to_uppercase() == "A")
        .map(|x| x.id)
        .collect::<Vec<_>>();

    let record_ids_v6 = records_filter_v6
        .filter(|x| x.type_.to_uppercase() == "AAAA")
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
