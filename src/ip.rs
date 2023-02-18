use crate::structs::{config::Config, Ipify};
use local_ip_address::list_afinet_netifas;
use reqwest::Client as HttpClient;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub(crate) async fn determine_ip(config: &Config) -> (Option<Ipv4Addr>, Option<Ipv6Addr>) {
    let ipv4 = determine_ipv4().await;
    let ipv6 = determine_ipv6(config).await;
    (ipv4, ipv6)
}

pub(crate) async fn determine_ipv4() -> Option<Ipv4Addr> {
    let http = HttpClient::builder()
        .local_address(Some(IpAddr::V4(Ipv4Addr::UNSPECIFIED)))
        .build()
        .ok()?;
    let response: Ipify = http
        .get("https://api64.ipify.org?format=json")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    match response.ip {
        IpAddr::V4(x) => Some(x),
        IpAddr::V6(_) => None,
    }
}

pub(crate) async fn determine_ipv6(config: &Config) -> Option<Ipv6Addr> {
    // let test = IpAddr::from_str("2000:dead:beef::dead:beef:420").ok()?;
    let http = HttpClient::builder()
        .local_address(Some(IpAddr::V6(Ipv6Addr::UNSPECIFIED)))
        .build()
        .ok()?;
    let response: Ipify = http
        .get("https://api64.ipify.org?format=json")
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;

    let mut ip = response.ip;

    if !config.use_preferred_ipv6 {
        let prefix = response.ip.to_string().split(':').collect::<Vec<_>>()[..3].join(":") + ":";

        let network_interfaces = list_afinet_netifas().ok()?;

        let ips = network_interfaces
            .iter()
            .map(|(_, ip)| ip.to_canonical())
            .filter(|&ip| ip.is_ipv6() && ip.is_global())
            .filter(|&ip| ip.to_string().starts_with(&prefix))
            .filter(|&ip| ip != response.ip)
            .collect::<Vec<_>>();

        ip = ips.first().unwrap_or(&ip).to_owned();
    }

    match ip {
        IpAddr::V4(_) => None,
        IpAddr::V6(x) => Some(x),
    }
}
