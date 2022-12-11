#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("double open")]
    DoubleOpen(String),
    #[error("double close")]
    DoubleClose(String),
    #[error("missing close")]
    MissingClose(String),
}

/// Parse a TiddlyWiki title list.
///
/// ```
/// # use twjsrs::parse::titles::parse;
/// assert_eq!(Ok(vec!["ok".to_string()]), parse("ok"));
/// assert_eq!(Ok(vec!["[[ok]]".to_string()]), parse("[[ok]]"));
/// assert_eq!(
///   Ok(vec!["two".to_string(), "titles".to_string()]),
///   parse("two titles")
/// );
/// assert_eq!(
///     Ok(vec!["[[two titles]]".to_string(), "second".to_string()]),
///     parse("[[two titles]] second")
/// );
/// ```
pub fn parse(titles_str: &str) -> Result<Vec<String>, Error> {
    let mut titles = Vec::new();
    let mut in_title = false;
    let mut title = String::new();
    for w in titles_str.split_ascii_whitespace() {
        if w.ends_with("]]") {
            if w.starts_with("[[") {
                if in_title {
                    return Err(Error::DoubleOpen(titles_str.to_string()));
                }
                titles.push(w.to_string());
            } else {
                if !in_title {
                    return Err(Error::DoubleClose(titles_str.to_string()));
                }
                in_title = false;
                title += " ";
                title += w;
                titles.push(title);
                title = String::new();
            }
        } else if w.starts_with("[[") {
            if in_title {
                return Err(Error::DoubleOpen(titles_str.to_string()));
            }
            in_title = true;
            title += w;
        } else if in_title {
            title += " ";
            title += w;
        } else {
            debug_assert!(!in_title);
            titles.push(w.to_string());
        }
    }
    if in_title {
        return Err(Error::DoubleOpen(titles_str.to_string()));
    }
    Ok(titles)
}

#[cfg(test)]
mod tests {
    use super::parse;
    use super::Error::*;

    #[test]
    fn test_parse() {
        assert_eq!(Err(DoubleOpen("[[".to_string())), parse("[["));
        assert_eq!(Err(DoubleClose("]]".to_string())), parse("]]"));
    }
}
