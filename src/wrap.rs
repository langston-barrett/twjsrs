//! Convenience functions.

use crate::parse;
use crate::parse::Tiddler;
use crate::raw::RawTiddler;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("tiddler parse error")]
    Parse(#[from] parse::Error),
    #[error("json parse error")]
    Serde(#[from] serde_json::Error),
}

/// Convert a string containing a JSON list of tiddlers into a vector of
/// [`RawTiddler`].
pub fn raw_from_str(s: &str) -> Result<Vec<RawTiddler>, serde_json::Error> {
    serde_json::from_str(s)
}

pub fn tiddlers_from_str(s: &str) -> Result<Vec<Tiddler>, Error> {
    let raw_tiddlers: Vec<RawTiddler> = serde_json::from_str(s)?;
    let mut tiddlers = Vec::new();
    for raw_tiddler in raw_tiddlers {
        tiddlers.push(Tiddler::from_raw(raw_tiddler)?);
    }
    Ok(tiddlers)
}
