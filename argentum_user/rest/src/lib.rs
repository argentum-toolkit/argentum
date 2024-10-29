#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "server")]
mod di;

pub mod dto;

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "server")]
pub use di::ApiDiC;
