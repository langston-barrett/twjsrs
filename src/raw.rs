use std::collections::HashMap;

/// Tiddler as represented in JSON.
#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RawTiddler {
    pub created: Option<String>,
    pub creator: Option<String>,
    pub modified: String,
    pub modifier: String,
    pub tags: Option<String>,
    pub text: Option<String>,
    pub title: String,
    #[serde(flatten)]
    pub fields: HashMap<String, String>,
}

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
