use dioxus::prelude::*;

#[component]
pub fn GoogleButton() -> Element {
    rsx! {
        button {
            onclick: move |_| {
                spawn(async move {
                    #[cfg(feature = "web")]
                    use crate::service::google::start_google_oauth;
                    #[cfg(feature = "web")]
                    if let Err(e) = start_google_oauth().await {
                        // errors.push(e.clone());
                    }
                });
            },
            class: "btn bg-white text-black border-[#e5e5e5]",
            svg {
                "aria-label": "Google logo",
                width: "24",
                height: "24",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 512 512",
                g {
                    path { d: "m0 0H512V512H0", fill: "#fff" }
                    path { fill: "#34a853", d: "M153 292c30 82 118 95 171 60h62v48A192 192 0 0190 341" }
                    path { fill: "#4285f4", d: "m386 400a140 175 0 0053-179H260v74h102q-7 37-38 57" }
                    path { fill: "#fbbc02", d: "m90 341a208 200 0 010-171l63 49q-12 37 0 73" }
                    path { fill: "#ea4335", d: "m153 219c22-69 116-109 179-50l55-54c-78-75-230-72-297 55" }
                }
            }
            "Login with Google"
        }
    }
}
