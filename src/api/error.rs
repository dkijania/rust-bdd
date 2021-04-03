use reqwest::{
    header::{InvalidHeaderName, InvalidHeaderValue},
    Error as ReqwestError,
};
use std::env::VarError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    ErrorResponse(String),
    #[error("request error: {0}")]
    RequestError(#[from] ReqwestError),
    #[error("unacceptable header value")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    #[error("unacceptable header name")]
    InvalidHeaderName(#[from] InvalidHeaderName),
    #[error("cannot parse url")]
    CannotSerializeResponse(#[from] serde_json::Error),
    #[error("url must be defined prior to sign request")]
    UrlMustBeDefinedPriorToSign,
    #[error("url must be defined prior to post request")]
    UrlMustBeDefinedPriorToPost,
    #[error("cannot read env variable")]
    VarError(#[from] VarError),
}
