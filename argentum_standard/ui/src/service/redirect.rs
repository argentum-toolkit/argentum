use dioxus::prelude::*;

pub fn redirect(r: impl Routable) -> Result<(), String> {
    let Some(window) = web_sys::window() else {
        return Err("Could not get Window object".to_string());
    };

    let Some(document) = window.document() else {
        return Err("Could not get Document object".to_string());
    };

    let Some(location) = document.location() else {
        return Err("Could not get location".to_string());
    };

    match location.set_href(r.to_string().as_str()) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Can't redirect. Error: {:?}", e)),
    }
}
