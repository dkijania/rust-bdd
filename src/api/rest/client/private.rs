use super::DefinedHeader;
use crate::api::{
    keys::{ApiKeys, RequestSigner},
    rest::{client::DefinedBodyParam, RestClient},
    utils::current_time_ticks,
    Error, Url,
};
use reqwest::blocking::Response;

/// Specialized rest client for sending private requests
pub struct SecureRestClient {
    /// general client
    client: RestClient,
    /// full url for endpoint
    url: Option<Url>,
}

impl SecureRestClient {
    pub fn new() -> Self {
        Self {
            client: RestClient::new(),
            url: None,
        }
    }

    /// add defaults headers like content type
    pub fn add_default_headers(mut self) -> Result<Self, Error> {
        self.client = self.client.add_default_headers()?;
        Ok(self)
    }

    /// sets url for request
    pub fn url(mut self, url: Url) -> Self {
        self.url = Some(url);
        self
    }

    /// adds body param
    pub fn defined_body_param(mut self, param: DefinedBodyParam) -> Self {
        self.client = self.client.defined_body_param(param);
        self
    }

    /// signs requests with provided api keys
    /// uses nonce equal to current time ticks
    /// # Arguments
    ///
    /// * `api_keys` - user keys
    ///
    pub fn sign(mut self, api_keys: ApiKeys) -> Result<Self, Error> {
        if self.url.is_none() {
            return Err(Error::UrlMustBeDefinedPriorToSign);
        }

        let nonce = current_time_ticks();
        self.client = self
            .client
            .defined_body_param(DefinedBodyParam::Nonce(nonce));

        let signature = RequestSigner::new()
            .nonce(nonce)
            .secret_key(api_keys.secret_key())
            .path(self.url.as_ref().unwrap().local_as_string())
            .params(self.client.lookup_body())
            .sign();

        self.client = self
            .client
            .defined_header(DefinedHeader::ApiKey, api_keys.api_key())?
            .defined_header(DefinedHeader::ApiSign, signature)?;
        Ok(self)
    }

    pub fn post(self) -> Result<Response, Error> {
        if self.url.is_none() {
            return Err(Error::UrlMustBeDefinedPriorToPost);
        }
        self.client.post(self.url.unwrap())
    }
}

impl Default for SecureRestClient {
    fn default() -> Self {
        Self::new()
    }
}
