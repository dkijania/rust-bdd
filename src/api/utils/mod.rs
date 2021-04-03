mod context;
mod env;
mod reflection;
mod time;

pub use context::ApiContext;
pub use env::*;
use indexmap::IndexMap;
pub use reflection::get_value;
pub use time::*;

pub fn convert_to_multipart(body: &IndexMap<String, String>) -> String {
    let mut output = body
        .into_iter()
        .fold(String::new(), |acc, (x, y)| acc + &x + "=" + &y + "&");
    output.pop();
    output
}
