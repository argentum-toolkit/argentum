#[cfg(feature = "web")]
mod client_side_authenticator;
mod client_side_authenticator_provider;
mod authentication_bar;

#[cfg(feature = "web")]
pub use client_side_authenticator::ClientSideAuthenticator;

pub use client_side_authenticator_provider::use_client_side_authenticator_provider;

pub use authentication_bar::AuthenticationBar;
