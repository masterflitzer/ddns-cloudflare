use crate::structs::config::Config;
use local_ip_address::list_afinet_netifas;
use mac_address::get_mac_address;
use reqwest::Client as HttpClient;
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    str::FromStr,
    time::Duration,
};

const IPV4_UNSPECIFIED: IpAddr = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
const IPV6_UNSPECIFIED: IpAddr = IpAddr::V6(Ipv6Addr::UNSPECIFIED);

pub(crate) async fn determine_ip(config: &Config) -> (Option<Ipv4Addr>, Option<Ipv6Addr>) {
    let ipv4 = determine_ipv4(config).await;
    let ipv6 = determine_ipv6(config).await;
    (ipv4, ipv6)
}

pub(crate) async fn query_ip(ip_enum: IpAddr) -> Option<IpAddr> {
    let local_ip: IpAddr = match ip_enum {
        IpAddr::V4(_) => IPV4_UNSPECIFIED,
        IpAddr::V6(_) => IPV6_UNSPECIFIED,
    };

    let http = HttpClient::builder()
        .local_address(local_ip)
        .timeout(Duration::from_secs(30))
        .build()
        .ok()?;

    let response = http
        .get("https://cloudflare.com/cdn-cgi/trace")
        .send()
        .await
        .ok()?
        .text()
        .await
        .ok()?;

    let data: HashMap<String, String> = response
        .lines()
        .map(|x| match x.split_once('=') {
            Some((key, value)) => (key.to_owned(), value.to_owned()),
            None => (String::new(), String::new()),
        })
        .collect();

    let ip = data.get("ip")?;
    let ip_address = IpAddr::from_str(ip).ok()?.to_canonical();
    Some(ip_address)
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

pub(crate) async fn determine_ipv4(config: &Config) -> Option<Ipv4Addr> {
    let _ = config;

    let ip = query_ip(IPV4_UNSPECIFIED).await?;

    let ipv4 = match ip {
        IpAddr::V4(x) => Some(x),
        IpAddr::V6(_) => None,
    }?;

    Some(ipv4)
}

pub(crate) async fn determine_ipv6(config: &Config) -> Option<Ipv6Addr> {
    let ip = query_ip(IPV6_UNSPECIFIED).await?;

    let ipv6 = match ip {
        IpAddr::V4(_) => None,
        IpAddr::V6(x) => Some(x),
    }?;

    if config.ipv6.prefer_outgoing {
        return Some(ipv6);
    }

    let (prefix, _) = split_ipv6(&ipv6)?;

    let network_interfaces = list_afinet_netifas().ok()?;
    let ipv6_addresses = network_interfaces
        .iter()
        .cloned()
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

    if ipv6_addresses.is_empty() {
        return None;
    }

    if ipv6_addresses.len() == 1 {
        return ipv6_addresses.first().cloned();
    }

    if config.ipv6.prefer_eui64 {
        let mac = get_mac_address().ok()?;
        let suffix = eui48_to_modified_eui64(&mac?.bytes())?;
        if let Some(ipv6_eui64) = ipv6_addresses
            .iter()
            .cloned()
            .find(|ip| match split_ipv6(ip) {
                Some((_, s)) => s == suffix,
                None => false,
            })
        {
            return Some(ipv6_eui64.to_owned());
        };
    }

    ipv6_addresses
        .iter()
        .cloned()
        .filter(|ip| ip != &ipv6)
        .collect::<Vec<_>>()
        .first()
        .cloned()
}
