use std::env::var;

pub fn env_has_value<S: Into<String>>(name: S, expected: S) -> bool {
    match var(name.into()) {
        Err(_) => false,
        Ok(actual) => actual == expected.into(),
    }
}
