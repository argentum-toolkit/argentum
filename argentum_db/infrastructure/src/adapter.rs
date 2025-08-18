#[derive(thiserror::Error, Debug, Default)]
#[error("DB error: `{msg}`, query: {sql:?}", sql=sql.clone().unwrap_or("".to_string()))]
pub struct DbAdapterError {
    pub msg: String,
    pub sql: Option<String>,
}
