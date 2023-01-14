pub(crate) mod cloudflare;
pub(crate) mod config;

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{net::Ipv4Addr, path::PathBuf};

#[derive(Debug, Parser)]
pub(crate) struct Args {
    /// Use alternative configuration file
    #[arg(short, long)]
    pub config: Option<PathBuf>,
    /// Print location of configuration file
    #[arg(long)]
    pub configuration: bool,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Ipify {
    pub ip: Ipv4Addr,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct RecordIds {
    pub V4: Vec<String>,
    pub V6: Vec<String>,
}
