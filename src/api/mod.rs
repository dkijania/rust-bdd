/// requests data
pub mod data;
/// api keys
pub mod keys;
/// utilities
pub mod utils;

mod error;
/// rest clients
pub mod rest;

pub use error::Error;
use rest::{RestApi, Url};

/// single entrypoint for all api operations
pub struct Api {
    /// url endpoint
    endpoint: Url,
}

impl Api {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str<S: Into<String>>(root: S) -> Self {
        Self {
            endpoint: Url::new(root.into()),
        }
    }

    pub fn rest(&self) -> RestApi {
        RestApi::new(self.endpoint.clone())
    }
}

impl From<Url> for Api {
    fn from(endpoint: Url) -> Self {
        Self { endpoint }
    }
}
