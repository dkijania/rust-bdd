use crate::api::utils::convert_to_multipart;
use hmac::{Hmac, Mac, NewMac};
use indexmap::IndexMap;
use sha2::{Digest, Sha256, Sha512};

pub struct RequestSigner {
    nonce: u64,
    secret_key: String,
    path: String,
    params: IndexMap<String, String>,
}

impl RequestSigner {
    pub fn new() -> Self {
        Self {
            nonce: 0u64,
            secret_key: "".to_owned(),
            path: "".to_owned(),
            params: IndexMap::new(),
        }
    }

    pub fn nonce(mut self, nonce: u64) -> Self {
        self.nonce = nonce;
        self
    }

    pub fn secret_key<S: Into<String>>(mut self, secret_key: S) -> Self {
        self.secret_key = secret_key.into();
        self
    }

    pub fn path<S: Into<String>>(mut self, path: S) -> Self {
        self.path = path.into();
        self
    }

    pub fn params(mut self, params: IndexMap<String, String>) -> Self {
        self.params = params;
        self
    }

    pub fn sign(self) -> String {
        println!(
            "nonce: {} \n secret: {} \n path: {} \n params: {:#?}",
            self.nonce, self.secret_key, self.path, self.params
        );

        let secret_bytes = base64::decode(self.secret_key).unwrap();
        let path_bytes = self.path.as_bytes();
        let params = convert_to_multipart(&self.params);
        let np = format!("{}{}", self.nonce, params);
        let mut hasher = Sha256::new();
        hasher.update(np.as_bytes());
        let hash256_bytes = hasher.finalize();

        let mut mac = Hmac::<Sha512>::new_varkey(&secret_bytes).unwrap();
        mac.update(&path_bytes);
        mac.update(&hash256_bytes);
        let signature = mac.finalize().into_bytes();
        let output = base64::encode(signature);
        println!("{}", output);
        output
    }
}

impl Default for RequestSigner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::RequestSigner;
    use indexmap::IndexMap;

    #[test]
    pub fn sign_tx() {
        let mut params = IndexMap::new();
        let nonce = 637529717071992864u64;
        params.insert("nonce".to_owned(), nonce.to_string());
        params.insert("trades".to_owned(), "True".to_owned());

        let hash = RequestSigner::new()
            .nonce(nonce)
            .secret_key("b67VivGPzyumN4Rj52h/ZONvze9JnI5LdVn62e/RuVc1cNJcHmsBkzT2JLVfFvlw6LYC+ySTIr0GUx0PIbsx+w==")
            .path("/0/private/OpenOrders")
            .params(params)
            .sign();

        assert_eq!(&hash,"eUo69FbdMc1XoVQsoa+t3XJ+oqls0EhZ5vAV6hBJTZHhmHqBwei79HTtUHeKao44EZa2QvDMy8eaQM+bMPiEWw==");
    }
}
