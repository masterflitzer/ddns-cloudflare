use crate::structs::{config::Config, Ipify};
use local_ip_address::list_afinet_netifas;
use mac_address::get_mac_address;
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

    let ipv6 = match response.ip {
        IpAddr::V4(_) => None,
        IpAddr::V6(x) => Some(x),
    }?;

    let (prefix, _) = split_ipv6(&ipv6)?;

    let network_interfaces = list_afinet_netifas().ok()?;
    let ipv6_addresses = network_interfaces
        .into_iter()
        .filter_map(|(_, ip)| match ip.to_canonical() {
            IpAddr::V4(_) => None,
            IpAddr::V6(x) => Some(x),
        })
        .filter(|ip| ip.is_global())
        .filter(|ip| match split_ipv6(ip) {
            Some((p, _)) => p == prefix,
            None => false,
        })
        .collect::<Vec<_>>();

    if ipv6_addresses.len() <= 1 {
        return ipv6_addresses.first().cloned();
    }

    if config.ipv6.prefer_eui64 {
        let mac = get_mac_address().ok()?;
        let suffix = eui48_to_modified_eui64(&mac?.bytes())?;
        if let Some(x) = ipv6_addresses.iter().find(|ip| match split_ipv6(ip) {
            Some((_, s)) => s == suffix,
            None => false,
        }) {
            return Some(x.to_owned());
        };
    }

    if !config.ipv6.prefer_outgoing {
        return ipv6_addresses
            .into_iter()
            .filter(|ip| ip != &ipv6)
            .collect::<Vec<_>>()
            .first()
            .cloned();
    }

    Some(ipv6)
}

fn split_ipv6(ipv6: &Ipv6Addr) -> Option<([u8; 8], [u8; 8])> {
    let octets = ipv6.octets();
    let (p, s) = octets.split_at(8);
    let prefix: [u8; 8] = p.try_into().ok()?;
    let suffix: [u8; 8] = s.try_into().ok()?;
    Some((prefix, suffix))
}

fn eui48_to_modified_eui64(eui48: &[u8; 6]) -> Option<[u8; 8]> {
    let (p, s) = eui48.split_at(3);
    let prefix: [u8; 3] = p.try_into().ok()?;
    let suffix: [u8; 3] = s.try_into().ok()?;
    let eui64 = [
        prefix[0], prefix[1], prefix[2], 0xff, 0xfe, suffix[0], suffix[1], suffix[2],
    ];
    let mut modified_eui64 = eui64;
    modified_eui64[0] ^= 0b0000_0010;
    Some(modified_eui64)
}
