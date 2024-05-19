#[derive(thiserror::Error, Debug)]
#[error("DB error: {msg}")]
pub struct DbAdapterError {
    pub msg: String,
}
