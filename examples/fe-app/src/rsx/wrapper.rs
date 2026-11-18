use crate::route::Route;
use crate::rsx::shell::Shell;
use dioxus::prelude::*;

#[component]
pub(crate) fn SiteWrapper() -> Element {
    rsx! {
        Shell {
            main_class: "relative prose flex-grow p-6 w-full max-w-4xl",
            Outlet::<Route> {}
        }
    }
}

#[component]
pub(crate) fn AccountWrapper() -> Element {
    rsx! {
        Shell {
            main_class: "relative prose flex-grow p-6 w-full",
            Outlet::<Route> {}
        }
    }
}
