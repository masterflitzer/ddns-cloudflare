pub(crate) mod config;
pub(crate) mod errors;
pub(crate) mod structs;

use clap::Parser;
use errors::{handle_errors, ErrorKind};
use reqwest::{Client as HttpClient, Response, Url};
use serde::de::DeserializeOwned;
use serde_json::Value as Json;
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
            handle_errors(&ErrorKind::API);
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

    for zone in config.zones {
        let zone_id = match obtain_zone_id(&data_zones, &zone.name).await {
            Some(x) => x,
            None => {
                println!(
                    "{}: Skipping this zone because the corresponding zone id could not be found",
                    &zone.name
                );
                continue;
            }
        };

        let url_list_dns_records =
            match api_base.join(format!("zones/{}/dns_records", zone_id).as_str()) {
                Ok(x) => x,
                Err(e) => {
                    handle_errors(&ErrorKind::Unknown(Box::new(e)));
                    exit(1)
                }
            };

        let response_records = match api_get(&http, url_list_dns_records, &config.api_token).await {
            Ok(x) => x,
            Err(_) => {
                handle_errors(&ErrorKind::API);
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

        for record in zone.records {
            let record_id = match obtain_record_ids(
                &data_records,
                format!("{}.{}", record, zone.name).as_str(),
            )
            .await
            {
                Some(x) => x,
                None => {
                    println!(
                    "{}: Skipping this record because the corresponding record id could not be found",
                    &record
                );
                    continue;
                }
            };

            let url_patch_dns_records =
                match api_base.join(format!("zones/{}/dns_records", zone_id).as_str()) {
                    Ok(x) => x,
                    Err(e) => {
                        handle_errors(&ErrorKind::Unknown(Box::new(e)));
                        exit(1)
                    }
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
    url: Url,
    api_token: impl AsRef<str>,
) -> Result<Response, reqwest::Error> {
    let response = http.get(url).bearer_auth(api_token.as_ref()).send().await?;
    Ok(response)
}

async fn api_patch(
    http: &HttpClient,
    url: Url,
    api_token: impl AsRef<str>,
) -> Result<Response, reqwest::Error> {
    let response = http
        .patch(url)
        .bearer_auth(api_token.as_ref())
        .send()
        .await?;
    Ok(response)
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

async fn obtain_zone_id(data: &Vec<ListZone>, zone_name: impl AsRef<str>) -> Option<String> {
    let zone = data.into_iter().find(|x| x.name == zone_name.as_ref())?;
    Some(zone.id.to_owned())
}

async fn obtain_record_ids(
    data: &Vec<ListDnsRecords>,
    record_name: impl AsRef<str>,
) -> Option<RecordIds> {
    let records_filter = data
        .to_owned()
        .into_iter()
        .filter(|x| x.name == record_name.as_ref());
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

async fn update_ip(response: Response, zone_id: impl AsRef<str>, record_id: impl AsRef<str>) {
    todo!()
}

fn is_http_success(response: &Response) -> bool {
    response.status().is_success()
}
