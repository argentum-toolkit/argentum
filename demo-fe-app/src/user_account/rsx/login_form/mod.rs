pub(crate) mod login_form;
#[cfg(not(feature = "web"))]
mod server_side;
#[cfg(feature = "web")]
mod web;

#[cfg(not(feature = "web"))]
pub use server_side::create_form_boilerplate;
#[cfg(feature = "web")]
pub use web::create_form_boilerplate;
