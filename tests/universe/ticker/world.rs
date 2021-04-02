use async_trait::async_trait;
use cucumber_rust::World;
use rust_bdd::api::{data::Ticker, Error as ApiError};
use std::collections::HashMap;
use std::convert::Infallible;

#[async_trait(?Send)]
impl World for TickerWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self::BeforeRequest)
    }
}

pub enum TickerWorld {
    BeforeRequest,
    AfterRequest(Result<HashMap<String, Ticker>, ApiError>),
}
