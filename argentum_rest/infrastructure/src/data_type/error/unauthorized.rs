use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Unauthorized {
    pub msg: String,
}

impl Unauthorized {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::error::Unauthorized;

    #[test]
    fn test_constructor() {
        let msg = "test msg".to_string();
        let err = Unauthorized::new(msg.clone());
        assert_eq!(msg, err.msg);
    }
}
