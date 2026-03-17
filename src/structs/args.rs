use clap::Parser;
use std::path::PathBuf;

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
