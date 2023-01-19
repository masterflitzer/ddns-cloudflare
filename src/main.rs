pub(crate) mod api;
pub(crate) mod config;
pub(crate) mod errors;
pub(crate) mod ip;
pub(crate) mod structs;

use api::{api_get, api_patch};
use clap::Parser;
use errors::{handle_errors, ErrorKind};
use ip::{determine_ipv4, determine_ipv6};
use reqwest::{Client as HttpClient, Response, Url};
use serde::de::DeserializeOwned;
use serde_json::Value as Json;
use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    process::exit,
};
use structs::{
    cloudflare::request::PatchDnsRecord,
    cloudflare::response::{ListDnsRecords, ListZone},
    cloudflare::Cloudflare,
    Args,
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

    let ipv4: Option<Ipv4Addr> = determine_ipv4(&http).await;
    let ipv6: Option<Ipv6Addr> = determine_ipv6().await;

    if ipv4.is_none() {
        handle_errors(&ErrorKind::IPv4)
    };

    if ipv6.is_none() {
        handle_errors(&ErrorKind::IPv6)
    };

    if ipv4.is_none() && ipv6.is_none() {
        println!("Neither IPv4 nor IPv6 address could be determined");
        exit(1)
    }

    let url_list_zones = match api_base.join("zones") {
        Ok(x) => x,
        Err(e) => {
            handle_errors(&ErrorKind::Unknown(Box::new(e)));
            exit(1)
        }
    };

    let response_zones = match api_get(&http, url_list_zones, &config.api_token).await {
        Ok(x) => x,
        Err(_) => {
            handle_errors(&ErrorKind::Api);
            exit(1);
        }
    };

    let json_zones = match deserialize_response(response_zones).await {
        Ok(x) => x,
        Err(e) => {
            handle_errors(&e);
            exit(1);
        }
    };

    let data_zones = match deserialize_json_value::<Vec<ListZone>>(json_zones.result).await {
        Ok(x) => x,
        Err(e) => {
            handle_errors(&e);
            exit(1);
        }
    };

    for config_zone in config.zones {
        let zone = match obtain_zone(&data_zones, &config_zone.name).await {
            Some(x) => x,
            None => {
                println!(
                    "Skipping \"{}\" because the corresponding zone could not be found",
                    &config_zone.name
                );
                continue;
            }
        };

        let url_list_dns_records =
            match api_base.join(format!("zones/{}/dns_records", zone.id).as_str()) {
                Ok(x) => x,
                Err(e) => {
                    handle_errors(&ErrorKind::Unknown(Box::new(e)));
                    exit(1)
                }
            };

        let response_records = match api_get(&http, url_list_dns_records, &config.api_token).await {
            Ok(x) => x,
            Err(_) => {
                handle_errors(&ErrorKind::Api);
                exit(1);
            }
        };

        let json_records = match deserialize_response(response_records).await {
            Ok(x) => x,
            Err(e) => {
                handle_errors(&e);
                match e {
                    ErrorKind::NoSuccessHttp | ErrorKind::NoSuccessJson => continue,
                    _ => exit(1),
                }
            }
        };

        let data_records =
            match deserialize_json_value::<Vec<ListDnsRecords>>(json_records.result).await {
                Ok(x) => x,
                Err(e) => {
                    handle_errors(&e);
                    exit(1);
                }
            };

        for config_record in config_zone.records {
            let records = obtain_records(
                &data_records,
                format!("{}.{}", config_record, config_zone.name).as_str(),
            )
            .await;

            if records.is_empty() {
                println!(
                    "Skipping \"{}\" because the corresponding records could not be found",
                    &config_record
                );
                continue;
            }

            'outer: for record in records {
                let url_patch_dns_records = match api_base
                    .join(format!("zones/{}/dns_records/{}", zone.id, record.id).as_str())
                {
                    Ok(x) => x,
                    Err(e) => {
                        handle_errors(&ErrorKind::Unknown(Box::new(e)));
                        exit(1)
                    }
                };

                let ip: IpAddr = match record.type_.to_uppercase().as_str() {
                    "A" => 'inner: {
                        if let Some(ip) = ipv4 {
                            break 'inner IpAddr::V4(ip);
                        }
                        continue 'outer;
                    }
                    "AAAA" => 'inner: {
                        if let Some(ip) = ipv6 {
                            break 'inner IpAddr::V6(ip);
                        }
                        continue 'outer;
                    }
                    _ => {
                        handle_errors(&ErrorKind::NonAddressRecord);
                        continue;
                    }
                };

                let payload = PatchDnsRecord {
                    comment: None,
                    content: Some(ip),
                    name: None,
                    proxied: None,
                    tags: None,
                    ttl: None,
                };

                let response_record = match api_patch(
                    &http,
                    url_patch_dns_records,
                    &config.api_token,
                    &payload,
                )
                .await
                {
                    Ok(x) => x,
                    Err(_) => {
                        handle_errors(&ErrorKind::Api);
                        exit(1);
                    }
                };

                match deserialize_response(response_record).await {
                    Ok(x) => x,
                    Err(e) => {
                        handle_errors(&e);
                        match e {
                            ErrorKind::NoSuccessHttp | ErrorKind::NoSuccessJson => continue,
                            _ => exit(1),
                        }
                    }
                };

                println!(
                    "Successfully updated IP of \"{}\" record \"{}\" in zone \"{}\" to \"{}\"",
                    record.type_, record.name, zone.name, ip
                );
            }
        }
    }
}

async fn deserialize_response(response: Response) -> Result<Cloudflare, ErrorKind> {
    if !is_http_success(&response) {
        return Err(ErrorKind::NoSuccessHttp);
    }

    let data = response
        .json::<Cloudflare>()
        .await
        .map_err(|_| ErrorKind::Json)?;

    if !data.success {
        return Err(ErrorKind::NoSuccessJson);
    }

    Ok(data)
}

async fn deserialize_json_value<T: DeserializeOwned>(data: Json) -> Result<T, ErrorKind> {
    let result = serde_json::from_value::<T>(data).map_err(|_| ErrorKind::Json)?;
    Ok(result)
}

async fn obtain_zone(data: &[ListZone], zone_name: &str) -> Option<ListZone> {
    let zone = data.iter().find(|&x| x.name == zone_name).cloned();
    zone
}

async fn obtain_records(data: &[ListDnsRecords], record_name: &str) -> Vec<ListDnsRecords> {
    let record_ids = data
        .iter()
        .filter(|&x| x.name == record_name)
        .filter(|&x| x.type_.to_uppercase() == "A" || x.type_.to_uppercase() == "AAAA")
        .cloned()
        .collect();
    record_ids
}

fn is_http_success(response: &Response) -> bool {
    response.status().is_success()
}
