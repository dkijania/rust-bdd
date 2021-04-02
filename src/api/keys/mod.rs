mod sign;

pub use sign::RequestSigner;

#[derive(Clone, Debug)]
pub struct ApiKeys {
    api_key: String,
    secret_key: String,
}

impl ApiKeys {
    pub fn new<S: Into<String>>(api_key: S, secret_key: S) -> Self {
        Self {
            api_key: api_key.into(),
            secret_key: secret_key.into(),
        }
    }

    pub fn secret_key(&self) -> String {
        self.secret_key.to_string()
    }

    pub fn api_key(&self) -> String {
        self.api_key.to_string()
    }
}
