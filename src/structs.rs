pub(crate) mod cloudflare;
pub(crate) mod config;

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{net::IpAddr, path::PathBuf};

#[derive(Debug, Parser)]
pub(crate) struct Args {
    /// Use alternative configuration file
    #[arg(short, long)]
    pub config: Option<PathBuf>,
    /// Print location of configuration file
    #[arg(long)]
    pub configuration: bool,
    /// Print app version
    #[arg(short, long)]
    pub version: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Ipify {
    pub ip: IpAddr,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RecordIds {
    pub v4: Vec<String>,
    pub v6: Vec<String>,
}
