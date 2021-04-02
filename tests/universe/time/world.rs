use async_trait::async_trait;
use cucumber_rust::World;
use rust_bdd::api::{data::ServerTime, Error as ApiError};
use std::convert::Infallible;

#[async_trait(?Send)]
impl World for TimeWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self::BeforeRequest)
    }
}

pub enum TimeWorld {
    BeforeRequest,
    AfterRequest(Result<ServerTime, ApiError>),
}
