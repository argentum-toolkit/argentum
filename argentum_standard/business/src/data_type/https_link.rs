use crate::invariant_violation::InvariantResult;
use compiletime_regex::regex;
use std::sync::LazyLock;

const ERR_LINK_TOO_SHORT: &str = "Link is too short";
const ERR_WRONG_EMAIL: &str = "Wrong email address";

pub static LINK_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex!(
        r#"^https://(?:[a-zA-Z0-9-]+\.)+[a-zA-Z]{2,}(?:/[^\s/$.?#]*)*(?:\?[^\s#]*)?(?:\#[^\s]*)?$"#
    )
});

#[derive(Clone, PartialEq, Debug)]
pub struct HttpsLink(String);

impl HttpsLink {
    pub fn try_new(link: &str) -> InvariantResult<Self> {
        if link.len() < 11 {
            return Err(ERR_LINK_TOO_SHORT.into());
        }

        if LINK_REGEX.is_match(link) {
            Ok(Self(link.into()))
        } else {
            Err(ERR_WRONG_EMAIL.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_new_ok() {
        let valid_link = "https://example.com";
        let result = HttpsLink::try_new(valid_link);
        assert!(result.is_ok());
    }

    #[test]
    fn test_try_new_ok_minimal() {
        let valid_link = "https://a.us";
        let result = HttpsLink::try_new(valid_link);
        assert!(result.is_ok());
    }

    #[test]
    fn test_try_new_ok_with_path() {
        let link_with_path = "https://example.com/path/to/resource";
        let result = HttpsLink::try_new(link_with_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_try_new_ok_with_query() {
        let link_with_query = "https://example.com?query=value";
        let result = HttpsLink::try_new(link_with_query);
        assert!(result.is_ok());
    }

    #[test]
    fn test_try_new_ok_with_fragment() {
        let link_with_fragment = "https://example.com#section";
        let result = HttpsLink::try_new(link_with_fragment);
        assert!(result.is_ok());
    }

    #[test]
    fn test_try_new_err_too_short() {
        let short_link = "http://";
        let result = HttpsLink::try_new(short_link);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().errors[0].to_string(),
            ERR_LINK_TOO_SHORT
        );
    }

    #[test]
    fn test_try_new_err_invalid_format() {
        let invalid_link = "definitely-not-a-link";
        let result = HttpsLink::try_new(invalid_link);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().errors[0].to_string(), ERR_WRONG_EMAIL);
    }
}
