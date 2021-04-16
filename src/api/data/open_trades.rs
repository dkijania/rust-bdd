use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Response template for OpenTrades response
#[derive(Serialize, Deserialize, Debug)]
pub struct OpenTrades {
    pub open: HashMap<String, Empty>,
}

/// At this moment only empty open trades are supported
/// Thats why this placeholed is created
#[derive(Serialize, Deserialize, Debug)]
pub struct Empty;
