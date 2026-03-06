pub mod rsx;

#[cfg(feature = "web")]
pub mod service;

#[derive(Clone)]
pub struct HomePageUrl(pub String);
