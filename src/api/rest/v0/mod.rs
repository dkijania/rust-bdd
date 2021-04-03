mod private;
mod public;

use crate::api::keys::ApiKeys;
use private::Private;
use public::Public;

use super::Url;

pub struct V0 {
    url: Url,
}

impl V0 {
    pub fn public(self) -> Public {
        self.url.join("public").into()
    }

    pub fn private(self, api_keys: Option<ApiKeys>) -> Private {
        let url = self.url.join("private");
        Private::new(url, api_keys)
    }
}

impl From<Url> for V0 {
    fn from(url: Url) -> Self {
        Self { url }
    }
}
