mod anonymous_registers_handler;
mod anonymous_registers_handler_trait;
mod authentication_bar;

#[cfg(feature = "web")]
mod client_side_authenticator;

mod client_side_authenticator_provider;

pub use anonymous_registers_handler::AnonymousRegistersHandler;
pub use anonymous_registers_handler_trait::AnonymousRegistersHandlerTrait;
pub use authentication_bar::AuthenticationBar;

#[cfg(feature = "web")]
pub use client_side_authenticator::ClientSideAuthenticator;
#[cfg(feature = "web")]
pub use client_side_authenticator::SecurityRepository;

pub use client_side_authenticator_provider::use_client_side_authenticator_provider;


