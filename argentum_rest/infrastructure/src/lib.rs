pub mod data_type;

#[cfg(feature = "server")]
mod di;
#[cfg(feature = "server")]
pub mod service;
#[cfg(feature = "server")]
pub use di::RestDiC;
