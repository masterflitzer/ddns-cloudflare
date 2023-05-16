use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    pub api_token: String,
    pub ipv6: Ipv6,
    pub records: HashMap<String, Vec<String>>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct Ipv6 {
    pub prefer_eui64: bool,
    pub prefer_outgoing: bool,
}
