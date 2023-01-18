use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Cloudflare {
    pub success: bool,
    pub result: Json,
}

pub(crate) mod request {
    use serde::{Deserialize, Serialize};
    use std::net::IpAddr;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub(crate) struct PatchDnsRecord {
        pub comment: Option<String>,
        pub content: Option<IpAddr>,
        pub name: Option<String>,
        pub proxied: Option<bool>,
        pub tags: Option<Vec<String>>,
        pub ttl: Option<u32>,
    }
}

pub(crate) mod response {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub(crate) struct ListZone {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub(crate) struct ListDnsRecords {
        pub content: String,
        pub id: String,
        pub name: String,
        pub proxied: bool,
        pub ttl: u32,
        #[serde(rename = "type")]
        pub type_: String,
        pub zone_id: String,
        pub zone_name: String,
    }
}
