/// Default content type used in each requests
/// with body
pub const DEFAULT_CONTENT_TYPE: &str = "application/x-www-form-urlencoded";

use std::fmt;

/// Helper structs which defined headers needed for signing requests
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum DefinedHeader {
    ApiKey,
    ApiSign,
}

impl fmt::Display for DefinedHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DefinedHeader::ApiKey => write!(f, "API-Key"),
            DefinedHeader::ApiSign => write!(f, "API-Sign"),
        }
    }
}
