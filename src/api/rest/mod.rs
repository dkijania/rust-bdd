mod client;
mod v0;

use v0::V0;

pub use client::{
    DefinedBodyParam, DefinedHeader, DefinedQuery, RestClient, SecureRestClient, Url,
    DEFAULT_CONTENT_TYPE,
};

pub struct RestApi {
    endpoint: Url,
}

impl RestApi {
    pub fn new(endpoint: Url) -> Self {
        Self { endpoint }
    }

    pub fn v0(self) -> V0 {
        self.endpoint.join("0").into()
    }
}
