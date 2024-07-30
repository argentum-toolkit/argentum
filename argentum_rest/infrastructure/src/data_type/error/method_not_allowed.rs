use hyper::Method;
use serde::{Serialize, Serializer};
use std::fmt::Display;

#[derive(Debug)]
pub struct MethodNotAllowedError {
    pub method: Method,
}

impl Display for MethodNotAllowedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = format!(
            "Method {} not allowed for this endpoint",
            self.method.as_str()
        );
        write!(f, "{}", str)
    }
}
impl Serialize for MethodNotAllowedError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl MethodNotAllowedError {
    pub fn new(method: Method) -> Self {
        Self { method }
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::error::MethodNotAllowedError;
    use hyper::Method;

    #[test]
    fn test_constructor() {
        let err_post = MethodNotAllowedError::new(Method::POST);
        assert_eq!(Method::POST, err_post.method);

        let err_delete = MethodNotAllowedError::new(Method::DELETE);
        assert_eq!(Method::DELETE, err_delete.method);
    }

    #[test]
    fn test_to_string() {
        let err = MethodNotAllowedError::new(Method::POST);

        assert_eq!("Method POST not allowed for this endpoint", err.to_string())
    }

    #[test]
    fn test_serialize() {
        let err = MethodNotAllowedError::new(Method::POST);

        let actual = serde_json::to_string(&err).unwrap();

        assert_eq!("\"Method POST not allowed for this endpoint\"", actual)
    }
}
