use async_trait::async_trait;
use cucumber_rust::World;
use rust_bdd::api::{data::OpenTrades, keys::ApiKeys, Error as ApiError};
use std::convert::Infallible;

#[async_trait(?Send)]
impl World for OpenOrdersWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self::Define)
    }
}

pub enum OpenOrdersWorld {
    Define,
    BeforeRequest(Option<ApiKeys>),
    AfterRequest(Result<OpenTrades, ApiError>),
}
