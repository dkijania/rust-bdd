use serde::{Deserialize, Serialize};

/// Response template for ServerTime response
#[derive(Serialize, Deserialize, Debug)]
pub struct ServerTime {
    pub unixtime: i64,
    pub rfc1123: String,
}
