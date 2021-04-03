use std::env::var;
/// checks if environment variable has expected value
///
/// # Arguments
///
/// * 'name' - name of environment variable
/// * 'expected' - expected value of environment variable
///
pub fn env_has_value<S: Into<String>>(name: S, expected: S) -> bool {
    match var(name.into()) {
        Err(_) => false,
        Ok(actual) => actual == expected.into(),
    }
}
