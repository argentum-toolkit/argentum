use dioxus::core::Element;

#[cfg(feature = "web")]
pub mod security;

pub mod rsx;

#[derive(Clone)]
pub struct AdditionalLoginBlock(pub Element);
