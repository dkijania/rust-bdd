use std::fmt;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum DefinedQuery {
    Pair,
}

impl fmt::Display for DefinedQuery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DefinedQuery::Pair => write!(f, "pair"),
        }
    }
}
