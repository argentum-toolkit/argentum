use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NotFoundError {
    pub msg: String,
}

impl NotFoundError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::error::NotFoundError;

    #[test]
    fn test_constructor() {
        let msg = "test msg".to_string();
        let err = NotFoundError::new(msg.clone());
        assert_eq!(msg, err.msg);
    }
}
