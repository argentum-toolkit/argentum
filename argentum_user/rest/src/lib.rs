#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "server")]
mod di;

pub mod dto;

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "server")]
pub use di::ApiDi;
#[cfg(feature = "server")]
pub use di::user_api_provide;

#[cfg(feature = "ui")]
pub mod ui;
