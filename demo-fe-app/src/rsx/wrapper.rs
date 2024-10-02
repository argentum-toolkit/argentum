use crate::route::Route;
use crate::rsx::footer::Footer;
use crate::rsx::navbar::NavBar;
use dioxus::prelude::*;

#[component]
pub(crate) fn Wrapper() -> Element {
    rsx! {
        header {
            NavBar {}
        }

        Outlet::<Route> {}

        Footer {}
    }
}
