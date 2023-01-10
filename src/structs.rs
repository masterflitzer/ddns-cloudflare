use std;
// use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_token: String,
    pub zone: String,
}

#[derive(Serialize, Deserialize)]
pub struct Ipify {
    pub ip: Ipv4Addr,
}
