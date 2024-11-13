#[cfg(feature = "web")]
use crate::route::Route;

#[cfg(feature = "web")]
pub fn redirect(r: Route) {
    let window = web_sys::window().expect("Missing Window");
    let document = window.document().expect("Could not get document");
    let location = document.location().expect("Could not get location");

    _ = location.set_href(r.to_string().as_str());
}
