use web_sys::{Location, window};

pub fn external_redirect(auth_url: &str) -> Result<(), String> {
    if let Some(win) = window() {
        let location: Location = win.location();
        if let Err(e) = location.set_href(auth_url) {
            return Err(format!("Failed to navigate: {:?}", e));
        }
    } else {
        return Err("Cannot access window object".to_string());
    }

    Ok(())
}
