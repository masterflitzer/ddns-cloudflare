use crate::structs::Ipify;
use reqwest::Client as HttpClient;
use std::net::{Ipv4Addr, Ipv6Addr};

pub(crate) async fn determine_ipv4(http: &HttpClient) -> Option<Ipv4Addr> {
    let response: Ipify = http
        .get("https://api.ipify.org?format=json")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    Some(response.ip)
}

pub(crate) async fn determine_ipv6() -> Option<Ipv6Addr> {
    // Some(Ipv6Addr::from_str("2000:dead:beef::dead:beef:420").unwrap());
    None
}
