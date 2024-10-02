use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub(crate) struct DarkMode(pub bool);

pub fn DarkModeToggle() -> Element {
    let mut dark_mode = use_context::<Signal<DarkMode>>();
    let style = if dark_mode().0 { "color:white" } else { "" };
    rsx! {
        label {
            style: "{style}",
            "Dark mode"
            input {
                r#type: "checkbox",
                oninput: move |event| {
                    let is_enabled = event.value() == "true";
                    dark_mode.write().0 = is_enabled;
                }
            }
        }
    }
}
