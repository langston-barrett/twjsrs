use std::collections::HashMap;

use chrono::NaiveDateTime;

use crate::raw::RawTiddler;

pub mod time;
pub mod titles;

/// Errors that can arise when converting from a [`RawTiddler`] to a
/// [`Tiddler`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("time parse error")]
    Time(#[from] time::Error),
    #[error("title list parse error")]
    Titles(#[from] titles::Error),
}

/// Tiddler represented with idiomatic Rust data structures.
#[derive(Debug, Eq, PartialEq)]
pub struct Tiddler {
    pub created: Option<NaiveDateTime>,
    pub creator: Option<String>,
    pub fields: HashMap<String, String>,
    pub modified: NaiveDateTime,
    pub modifier: String,
    pub tags: Vec<String>,
    pub text: Option<String>,
    pub title: String,
}

fn optional_time(ot: Option<String>) -> Result<Option<NaiveDateTime>, time::Error> {
    match ot {
        None => Ok(None),
        Some(s) => Ok(Some(time::parse(&s)?)),
    }
}

impl Tiddler {
    /// Parse the information in a [`RawTiddler`].
    pub fn from_raw(raw: RawTiddler) -> Result<Self, Error> {
        Ok(Tiddler {
            created: optional_time(raw.created)?,
            creator: raw.creator,
            fields: raw.fields,
            modified: time::parse(&raw.modified)?,
            modifier: raw.modifier,
            tags: titles::parse(&raw.tags.unwrap_or_default())?,
            text: raw.text,
            title: raw.title,
        })
    }
}

// impl Tiddler {
//     pub fn raw(&self) -> RawTiddler {
//         RawTiddler {
//             created: todo!(),
//             creator: todo!(),
//             fields: self.fields,
//             modified: todo!(),
//             modifier: self.modifier,
//             tags: todo!(),
//             text: self.text,
//             title: self.title,
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::RawTiddler;

    #[test]
    fn deserialize() {
        let serialized = "{
            \"created\": \"20220131074400001\",
            \"creator\": \"user\",
            \"text\": \"text\",
            \"tags\": \"tag [[second tag]]\",
            \"title\": \"Title\",
            \"modified\": \"20220131074400001\",
            \"modifier\": \"user\"
        }";
        let _tiddler: RawTiddler = serde_json::from_str(&serialized).unwrap();
    }
}
