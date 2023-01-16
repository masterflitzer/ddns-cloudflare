use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Cloudflare {
    pub success: bool,
    pub result: Json,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CloudflareResultVector<T> {
    pub result: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CloudflareResultHashMap<T> {
    pub result: HashMap<String, T>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub(crate) enum RecordType {
    A,
    Aaaa,
}

pub(crate) mod request {
    use serde::{Deserialize, Serialize};
    use std::net::IpAddr;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub(crate) struct PatchDnsRecord {
        pub comment: String,
        pub content: IpAddr,
        pub name: String,
        pub proxied: bool,
        pub tags: Vec<String>,
        pub ttl: u32,
    }
}

pub(crate) mod response {
    use crate::structs::cloudflare::RecordType;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub(crate) struct ListZone {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub(crate) struct ListDnsRecords {
        pub id: String,
        pub name: String,
        #[serde(rename = "type")]
        pub type_: RecordType,
        pub zone_id: String,
        pub zone_name: String,
    }
}
