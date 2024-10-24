#[cfg(feature = "web")]
mod client_side_authenticator;

#[cfg(feature = "web")]
pub use client_side_authenticator::ClientSideAuthenticator;
