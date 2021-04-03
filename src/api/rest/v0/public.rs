use crate::api::{
    data::{as_result, ServerTime, Ticker},
    error::Error,
    rest::{DefinedQuery, RestClient, Url},
};
use std::collections::HashMap;

/// public requests api
pub struct Public {
    url: Url,
    rest_client: RestClient,
}

impl Public {
    /// returns ticker from api
    /// # Arguments
    ///
    /// * `ticker` - A string that holds the pair symbol (like XBTUSD)
    pub fn ticker<S: Into<String>>(self, ticker: S) -> Result<HashMap<String, Ticker>, Error> {
        as_result(
            &self
                .rest_client
                .add_default_headers()?
                .query(DefinedQuery::Pair.to_string(), ticker.into())
                .post(self.url.join("AssetPairs"))?
                .text()?,
        )
    }
    /// returns api server time
    pub fn server_time(self) -> Result<ServerTime, Error> {
        as_result(&self.rest_client.post(self.url.join("Time"))?.text()?)
    }
}

impl From<Url> for Public {
    fn from(url: Url) -> Self {
        Self {
            url,
            rest_client: Default::default(),
        }
    }
}
