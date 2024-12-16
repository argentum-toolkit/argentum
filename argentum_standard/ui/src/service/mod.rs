#[cfg(feature = "web")]
mod redirect;

#[cfg(feature = "web")]
pub use redirect::redirect;
