use std::num::ParseIntError;

use chrono::{Duration, NaiveDateTime};

/// Time parsing errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("time parse error")]
    Time(#[from] chrono::ParseError),
    #[error("milliseconds parse error")]
    Millis(#[from] ParseIntError),
}

// https://tiddlywiki.com/#Date%20Fields
//
// 4 digits for the year
// 2 digits for the month
// 2 digits for the day
// 2 digits for the hour
// 2 digits for the minute
// 2 digits for the second
// 3 digits for the millisecond
//
// Format:  YYYYMMDDHHMMSSMMM
// Example: 20220131074400001
const FORMAT: &str = "%Y%m%d%H%M%S";

/// Parse a TiddlyWiki timestamp
///
/// ```
/// # use chrono::naive::*;
/// # use twjsrs::parse::time::parse;
/// assert_eq!(
///   NaiveDateTime::new(
///     NaiveDate::from_ymd_opt(2022, 1, 2).unwrap(),
///     NaiveTime::from_hms_milli_opt(12, 11, 10, 9).unwrap()
///   ),
///   parse("20220102121110009").unwrap()
/// )
/// ```
pub fn parse(s: &str) -> Result<NaiveDateTime, Error> {
    debug_assert!(s.len() == "YYYYMMDDHHMMSSMMM".len());
    let (time, millis) = s.split_at("YYYYMMDDHHMMSS".len());
    debug_assert!(millis.len() == 3);
    let t = NaiveDateTime::parse_from_str(time, FORMAT)?;
    Ok(t + Duration::milliseconds(millis.parse::<i64>()?))
}
