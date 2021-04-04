/// api context utilities
mod context;
/// environment variables utilities
mod env;
/// small reflection methods package
mod reflection;
/// time related utilities
mod time;

pub use context::ApiContext;
pub use env::*;
use indexmap::IndexMap;
pub use reflection::get_value;
pub use time::*;

/// converts index map to string, used in request body
pub fn convert_to_multipart(body: &IndexMap<String, String>) -> String {
    let mut output = body
        .into_iter()
        .fold(String::new(), |acc, (x, y)| acc + &x + "=" + &y + "&");
    output.pop();
    output
}
