use crate::api::{
    data::{as_result, OpenTrades},
    error::Error,
    rest::{v0::ApiKeys, DefinedBodyParam, SecureRestClient, Url},
};

/// Private requests api
pub struct Private {
    /// full url to endpoint
    url: Url,
    /// optional (in case of negative tests) api keys used in request
    api_keys: Option<ApiKeys>,
    /// rest client used in requests sending
    rest_client: SecureRestClient,
}

impl Private {
    pub fn new(url: Url, api_keys: Option<ApiKeys>) -> Self {
        Self {
            url,
            api_keys,
            rest_client: SecureRestClient::new(),
        }
    }

    /// sends open orders request, signs it with api keys (if defined)
    /// returns response as result
    pub fn open_orders(self) -> Result<OpenTrades, Error> {
        let mut client = self
            .rest_client
            .add_default_headers()?
            .url(self.url.join("OpenOrders"))
            .defined_body_param(DefinedBodyParam::Trades(true));

        if let Some(api_keys) = self.api_keys {
            client = client.sign(api_keys)?;
        }

        as_result(&client.post()?.text()?)
    }
}
