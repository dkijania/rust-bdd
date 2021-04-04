use indexmap::IndexMap;
use reqwest::{
    blocking::Response,
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
};
use std::{collections::HashMap, str::FromStr};

use super::Url;
use crate::api::{
    rest::{DefinedBodyParam, DefinedHeader, DEFAULT_CONTENT_TYPE},
    utils::convert_to_multipart,
    Error,
};

/// Rest client for sending requests
pub struct RestClient {
    /// headers to be used in request
    headers: HeaderMap,
    /// query parameters
    query: HashMap<String, String>,
    /// body parameters
    body: IndexMap<String, String>,
}

impl RestClient {
    pub fn new() -> Self {
        Self {
            headers: HeaderMap::new(),
            query: HashMap::new(),
            body: IndexMap::new(),
        }
    }

    pub fn add_default_headers(mut self) -> Result<Self, Error> {
        self.headers
            .insert(CONTENT_TYPE, HeaderValue::from_str(DEFAULT_CONTENT_TYPE)?);
        Ok(self)
    }

    pub fn lookup_body(&self) -> IndexMap<String, String> {
        self.body.clone()
    }

    pub fn defined_header<S: Into<String>>(
        self,
        name: DefinedHeader,
        value: S,
    ) -> Result<Self, Error> {
        self.header(name.to_string(), value.into())
    }

    pub fn header<S: Into<String>>(mut self, name: S, value: S) -> Result<Self, Error> {
        self.headers.insert(
            HeaderName::from_str(&name.into())?,
            HeaderValue::from_str(&value.into())?,
        );
        Ok(self)
    }

    pub fn query<S: Into<String>>(mut self, name: S, value: S) -> Self {
        self.query.insert(name.into(), value.into());
        self
    }

    pub fn defined_body_param(self, param: DefinedBodyParam) -> Self {
        let (name, value) = param.into_pair();
        self.add_body_param(name, value)
    }

    pub fn add_body_param<S: Into<String>>(mut self, name: S, value: S) -> Self {
        self.body.insert(name.into(), value.into());
        self
    }

    /// Sends request and returns general response
    /// # Arguments
    ///
    /// * `url` - full url to endpoint
    ///
    pub fn post(self, url: Url) -> Result<Response, Error> {
        println!("Headers: {:#?}", self.headers);

        let client = reqwest::blocking::Client::new()
            .post(&url.as_string())
            .headers(self.headers)
            .query(
                &self
                    .query
                    .iter()
                    .map(|(x, y)| (x.as_str(), y.as_str()))
                    .collect::<Vec<(&str, &str)>>(),
            )
            .body(convert_to_multipart(&self.body));

        println!(
            "Request: {:#?}, Body: {:#?}",
            client,
            convert_to_multipart(&self.body)
        );
        client.send().map_err(Into::into)
    }
}

impl Default for RestClient {
    fn default() -> Self {
        Self::new()
    }
}
