use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    pub api_token: String,
    pub ipv6_preferred: bool,
    pub zones: Vec<Zone>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct Zone {
    pub name: String,
    pub records: Vec<String>,
}
