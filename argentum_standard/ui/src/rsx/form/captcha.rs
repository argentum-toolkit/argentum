use crate::rsx::component::ErrorBlock;
use crate::{CaptchaEnabled, CaptchaTurnstileSiteKey};
use argentum_standard_infrastructure::invariant_violation::ViolationsDto;
use dioxus::prelude::*;
use serde::Deserialize;

const TURNSTILE_SCRIPT_URL: &str =
    "https://challenges.cloudflare.com/turnstile/v0/api.js?render=explicit";

#[derive(Clone, PartialEq, Props)]
#[props(borrowed)]
pub struct CaptchaProps {
    pub value: String,
    pub id: String,
    pub name: String,
    pub label: String,
    pub required: bool,
    pub violations: Option<ViolationsDto>,

    processor: CaptchaProcessor,

    oninput: EventHandler<String>,
}

#[derive(Clone, PartialEq, Default)]
pub struct CaptchaProcessor {}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "event", rename_all = "snake_case")]
enum TurnstileEvent {
    Token,
    Expired,
    Error,
    Blocked,
}

#[component]
pub fn Captcha(props: CaptchaProps) -> Element {
    let captcha_enabled = try_use_context::<CaptchaEnabled>().is_some_and(|enabled| enabled.0);
    let site_key = try_use_context::<CaptchaTurnstileSiteKey>()
        .and_then(|key| turnstile_site_key(Some(&key.0)))
        .filter(|_| captcha_enabled);

    let label_class = match props.violations {
        Some(_) => "text-error",
        None => "",
    };

    let required_class = match props.required {
        true => "required-label",
        false => "",
    };

    let is_enabled = site_key.is_some();

    rsx! {
        fieldset { class: "fieldset w-full",
            if !props.label.is_empty() {
                label {
                    "for": "{props.id}",
                    class: "label-text {label_class} {required_class}",
                    "{props.label}"
                }
            }

            input {
                id: "{props.id}",
                name: "{props.name}",
                r#type: "hidden",
                value: "{props.value}",
                "data-ag-captcha-required": "{is_enabled}",
                oninput: move |event| props.oninput.call(event.value()),
            }

            if let Some(site_key) = site_key {
                CaptchaWidget { site_key, input_id: props.id.clone() }
            }

            if let Some(v) = props.violations {
                ErrorBlock {errors: v.errors}
            }
        }
    }
}

#[component]
fn CaptchaWidget(site_key: String, input_id: String) -> Element {
    let mut error: Signal<Option<&'static str>> = use_signal(|| None);
    let widget_dom_id = widget_id(&input_id);

    use_future(move || {
        let script = turnstile_script(&site_key, &input_id);

        async move {
            let mut eval = document::eval(&script);

            while let Ok(event) = eval.recv::<TurnstileEvent>().await {
                error.set(turnstile_error_message(&event));
            }
        }
    });

    rsx! {
        div { class: "mt-5",
            div { id: widget_dom_id }
            if let Some(message) = error() {
                div { class: "text-error text-sm mt-2", "{message}" }
            }
        }
    }
}

fn widget_id(input_id: &str) -> String {
    format!("{input_id}-turnstile")
}

fn turnstile_site_key(raw: Option<&str>) -> Option<String> {
    let key = raw?.trim();

    let is_valid = !key.is_empty()
        && key
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');

    is_valid.then(|| key.to_string())
}

fn turnstile_error_message(event: &TurnstileEvent) -> Option<&'static str> {
    match event {
        TurnstileEvent::Token => None,
        TurnstileEvent::Expired => Some("The verification has expired. Please confirm again."),
        TurnstileEvent::Error => Some("Verification failed. Please try again."),
        TurnstileEvent::Blocked => Some("Please confirm that you are not a robot."),
    }
}

// The token is written into the hidden input of the captcha element, its `oninput` handler
// keeps the form state in sync. The submit guard blocks a form whose captcha input is empty
// while the widget is enabled and reports the block to this widget's error message.
fn turnstile_script(site_key: &str, input_id: &str) -> String {
    let widget_id = widget_id(input_id);

    format!(
        r#"
const siteKey = "{site_key}";
const inputId = "{input_id}";
const widgetId = "{widget_id}";

const send = (event) => dioxus.send(event);

const setCaptcha = (token) => {{
    const input = document.getElementById(inputId);
    if (!input) return;
    input.value = token;
    input.dispatchEvent(new Event("input", {{ bubbles: true }}));
}};

const loadTurnstile = () => {{
    if (window.turnstile) return Promise.resolve();
    if (!window.agTurnstileLoading) {{
        window.agTurnstileLoading = new Promise((resolve, reject) => {{
            const script = document.createElement("script");
            script.src = "{TURNSTILE_SCRIPT_URL}";
            script.async = true;
            script.onload = () => resolve();
            script.onerror = () => {{
                window.agTurnstileLoading = null;
                reject(new Error("Turnstile script failed to load"));
            }};
            document.head.appendChild(script);
        }});
    }}
    return window.agTurnstileLoading;
}};

if (!window.agCaptchaGuard) {{
    window.agCaptchaGuard = true;
    document.addEventListener("submit", (e) => {{
        if (!(e.target instanceof HTMLFormElement)) return;
        const pending = [...e.target.querySelectorAll('input[data-ag-captcha-required="true"]')]
            .find((input) => input.value.trim() === "");
        if (!pending) return;
        e.preventDefault();
        e.stopImmediatePropagation();
        pending.agCaptchaBlocked?.();
    }}, true);
}}

while (!document.getElementById(widgetId)) {{
    await new Promise((resolve) => setTimeout(resolve, 50));
}}

document.getElementById(inputId).agCaptchaBlocked = () => send({{ event: "blocked" }});

try {{
    await loadTurnstile();
}} catch (_) {{
    send({{ event: "error" }});
    await new Promise(() => {{}});
}}

turnstile.render(`#${{widgetId}}`, {{
    sitekey: siteKey,
    callback: (token) => {{
        setCaptcha(token);
        send({{ event: "token" }});
    }},
    "error-callback": () => {{
        setCaptcha("");
        send({{ event: "error" }});
    }},
    "expired-callback": () => {{
        setCaptcha("");
        send({{ event: "expired" }});
    }},
}});

await new Promise(() => {{}});
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn site_key_is_absent_when_not_configured() {
        assert_eq!(turnstile_site_key(None), None);
        assert_eq!(turnstile_site_key(Some("")), None);
        assert_eq!(turnstile_site_key(Some("   ")), None);
    }

    #[test]
    fn site_key_is_trimmed_and_accepted_when_valid() {
        assert_eq!(
            turnstile_site_key(Some(" 1x00000000000000000000AA ")),
            Some("1x00000000000000000000AA".to_string())
        );
    }

    #[test]
    fn site_key_with_characters_unsafe_for_js_is_rejected() {
        assert_eq!(turnstile_site_key(Some("abc\"; alert(1); \"")), None);
    }

    #[test]
    fn token_event_clears_error_and_others_show_message() {
        assert_eq!(turnstile_error_message(&TurnstileEvent::Token), None);
        assert!(turnstile_error_message(&TurnstileEvent::Expired).is_some());
        assert!(turnstile_error_message(&TurnstileEvent::Error).is_some());
        assert!(turnstile_error_message(&TurnstileEvent::Blocked).is_some());
    }

    #[test]
    fn script_embeds_site_key_and_targets_captcha_input() {
        let script = turnstile_script("1x00000000000000000000AA", "captcha");

        assert!(script.contains(r#"const siteKey = "1x00000000000000000000AA";"#));
        assert!(script.contains(r#"const inputId = "captcha";"#));
        assert!(script.contains(r#"const widgetId = "captcha-turnstile";"#));
        assert!(script.contains(TURNSTILE_SCRIPT_URL));
    }
}
