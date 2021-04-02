use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Ticker {
    pub altname: String,
    pub wsname: String,
    pub aclass_base: String,
    pub base: String,
    pub aclass_quote: String,
    pub quote: String,
    pub lot: String,
    pub pair_decimals: u8,
    pub lot_decimals: u8,
    pub lot_multiplier: u8,
    pub leverage_buy: Vec<u8>,
    pub leverage_sell: Vec<u8>,
    pub fees: Vec<Vec<f64>>,
    pub fees_maker: Vec<Vec<f64>>,
    pub fee_volume_currency: String,
    pub margin_call: u8,
    pub margin_stop: u8,
    pub ordermin: String,
}
