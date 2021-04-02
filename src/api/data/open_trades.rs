use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenTrades {
    pub open: HashMap<String, Empty>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Empty;
