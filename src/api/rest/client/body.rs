#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum DefinedBodyParam {
    Nonce(u64),
    Trades(bool),
}

impl DefinedBodyParam {
    pub fn into_pair(self) -> (String, String) {
        match self {
            DefinedBodyParam::Nonce(timestamp) => ("nonce".to_string(), timestamp.to_string()),
            DefinedBodyParam::Trades(trades) => (
                "trades".to_string(),
                uppercase_first_letter(&trades.to_string()),
            ),
        }
    }
}

impl From<u64> for DefinedBodyParam {
    fn from(timestamp: u64) -> Self {
        DefinedBodyParam::Nonce(timestamp)
    }
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}
