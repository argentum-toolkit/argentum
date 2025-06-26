mod authentication_bar;

//todo: not pub
#[cfg(feature = "web")]
pub mod client_side_authenticator;

mod client_side_authenticator_provider;

pub use authentication_bar::AuthenticationBar;

#[cfg(feature = "web")]
pub use client_side_authenticator::ClientSideAuthenticator;
#[cfg(feature = "web")]
pub use client_side_authenticator::SecurityRepository;

pub use client_side_authenticator_provider::use_client_side_authenticator_provider;
