mod open_trades;
mod server_time;
mod ticker;

use crate::api::Error;
pub use open_trades::OpenTrades;
use serde::{Deserialize, Serialize};
pub use server_time::ServerTime;
pub use ticker::Ticker;

/// Template Response which is produced after each call
/// usually api won't send error code for invalid request
/// but each response got 'error' field which contains error message
/// and result field is empty then
///
/// This sturct can convert api response to standard rust Response
#[derive(Deserialize, Serialize, Debug)]
pub struct Response<T> {
    error: Vec<String>,
    result: Option<T>,
}

#[allow(clippy::from_over_into)]
impl<'a, T> Into<Result<T, Error>> for Response<T>
where
    T: Deserialize<'a>,
{
    fn into(self) -> Result<T, Error> {
        if self.error.is_empty() {
            Ok(self.result.unwrap())
        } else {
            Err(Error::ErrorResponse(self.error.join("")))
        }
    }
}

/// Converts response texts into standard rust std::restult::Result object
/// Returns a person with the name given them
///
/// # Arguments
///
/// * `text` - string that holds raw rest api response
///
pub fn as_result<'a, T: serde::Deserialize<'a>>(text: &'a str) -> Result<T, Error> {
    let response: Response<T> = serde_json::from_str(&text)?;
    response.into()
}
