use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerTime {
    pub unixtime: i64,
    pub rfc1123: String,
}
