use crate::api::{
    data::{as_result, OpenTrades},
    error::Error,
    rest::{v0::ApiKeys, DefinedBodyParam, SecureRestClient, Url},
};

pub struct Private {
    url: Url,
    api_keys: Option<ApiKeys>,
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
