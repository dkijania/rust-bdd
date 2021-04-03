use crate::api::{keys::ApiKeys, Api};
use std::env::var;

const API_KEY_ENV_NAME: &str = "API_KEY";
const SECRET_KEY_ENV_NAME: &str = "SECRET_KEY";
const ENDPOINT: &str = "API_ENDPOINT";

#[derive(Clone, Debug)]
pub struct ApiContext {
    api_keys: Option<ApiKeys>,
    address: String,
}

impl ApiContext {
    pub fn from_env_variables() -> Self {
        let api_key = read_env_variable(API_KEY_ENV_NAME);
        let secret_key = read_env_variable(SECRET_KEY_ENV_NAME);
        let address = read_env_variable(ENDPOINT);

        Self {
            api_keys: Some(ApiKeys::new(api_key, secret_key)),
            address,
        }
    }

    pub fn new<S: Into<String>>(address: S) -> Self {
        Self {
            address: address.into(),
            api_keys: None,
        }
    }

    pub fn new_with_keys<S: Into<String>>(address: S, api_keys: ApiKeys) -> Self {
        Self {
            address: address.into(),
            api_keys: Some(api_keys),
        }
    }

    pub fn keys(&self) -> &Option<ApiKeys> {
        &self.api_keys
    }

    pub fn api(&self) -> Api {
        Api::from_str(&self.address)
    }
}

fn read_env_variable<S: Into<String>>(key: S) -> String {
    let env_name = key.into();
    match var(&env_name) {
        Ok(val) => val,
        Err(e) => panic!(
            "couldn't find env variable {}, due to: {}. {}",
            env_name,
            e,
            get_error_message()
        ),
    }
}

fn get_error_message() -> String {
    format!(
        "In order to make tests run 3 env variables need to be defined: 
    {}: valid api-key in format '68m51U...'   
    {}: valid user secret-key in format 'b67VivGPzyumN4Rj52h...'  
    {}: endpoint in format https://host.com/",
        API_KEY_ENV_NAME, SECRET_KEY_ENV_NAME, ENDPOINT
    )
}
