use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_token: String,
    pub zones: Vec<Zone>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Zone {
    pub name: String,
    pub records: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Ipify {
    pub ip: Ipv4Addr,
}
