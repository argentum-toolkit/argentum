pub mod rsx;

#[cfg(feature = "web")]
pub mod service;

#[derive(Clone)]
pub struct HomePageUrl(pub String);

#[derive(Clone)]
pub struct LoginUrl(pub String);

#[derive(Clone)]
pub struct RegistrationUrl(pub String);
