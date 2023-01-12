use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{net::Ipv4Addr, path::PathBuf};

#[derive(Debug, Parser)]
pub struct Args {
    /// Use alternative configuration file
    #[arg(short, long)]
    pub config: Option<PathBuf>,
    /// Print location of configuration file
    #[arg(long)]
    pub configuration: bool,
}

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
