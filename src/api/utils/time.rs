use chrono::{prelude::*, ParseError};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_time_ticks() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        * 1_000_000_000
}

pub fn current_time_unix() -> i64 {
    let utc: DateTime<Utc> = Utc::now();
    utc.timestamp()
}

pub fn parse_as_unixtime<S: Into<String>, P: Into<String>>(
    date_time: S,
    format: P,
) -> Result<i64, ParseError> {
    let date_time = DateTime::parse_from_str(&date_time.into(), &format.into())?;
    Ok(date_time.timestamp())
}
