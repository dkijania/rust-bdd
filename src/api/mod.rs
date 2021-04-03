pub mod data;
pub mod keys;
pub mod utils;

mod error;
mod rest;

pub use error::Error;
use rest::{RestApi, Url};

pub struct Api {
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
