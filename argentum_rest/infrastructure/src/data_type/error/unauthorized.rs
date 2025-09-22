use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Unauthorized {
    pub msg: String,
}

impl Unauthorized {
    pub fn new(msg: &str) -> Self {
        Self { msg: msg.into() }
    }
}

#[cfg(test)]
mod tests {
    use crate::data_type::error::Unauthorized;

    #[test]
    fn test_constructor() {
        let msg = "test msg";
        let err = Unauthorized::new(msg);
        assert_eq!(msg, err.msg);
    }
}
