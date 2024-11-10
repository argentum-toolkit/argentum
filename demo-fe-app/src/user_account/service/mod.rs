mod client_side_authenticator;

#[cfg(feature = "web")]
pub use client_side_authenticator::ClientSideAuthenticator;

pub use client_side_authenticator::use_client_side_authenticator;
