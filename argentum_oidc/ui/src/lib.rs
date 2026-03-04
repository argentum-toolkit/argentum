pub mod di;

pub mod rsx;

#[cfg(feature = "web")]
pub mod service;

#[derive(Clone)]
pub struct GithubClientId(pub String);

#[derive(Clone)]
pub struct GoogleClientId(pub String);

#[derive(Clone)]
pub struct OidcRedirectUrl(pub String);
