use crate::service::AuthCodeExchangerError;

#[derive(thiserror::Error, Debug)]
pub enum ExchangesCodeToTokenError {
    #[error("Can't exchange token")]
    Exchanger(#[from] AuthCodeExchangerError),
}
