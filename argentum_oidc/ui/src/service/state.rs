use rand::{RngExt, distr::Alphanumeric};
use web_sys::window;

pub fn create_state() -> Result<String, String> {
    // CSRF
    let state: String = rand::rng()
        .sample_iter(Alphanumeric)
        .take(32)
        .map(char::from)
        .collect::<String>();

    // Optional: save state in localStorage or sessionStorage
    // to verify later when callback occurs (recommended)
    if let Some(window) = window()
        && let Ok(storage) = window.session_storage()
    {
        let _ = storage
            .expect("Can't create storage object")
            .set_item("oauth_state", &state);
    }

    Ok(state)
}
