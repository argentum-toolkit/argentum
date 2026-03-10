use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct AuthBarProps<R>
where
    R: Routable + std::cmp::PartialEq,
{
    pub login_route: R,
    pub registration_route: R,
    pub logout_redirect: R,
}

#[component]
pub fn AuthBar<R: Routable + std::cmp::PartialEq>(props: AuthBarProps<R>) -> Element {
    #[cfg(feature = "web")]
    {
        let mut mounted = use_signal(|| false);

        use_effect(move || {
            mounted.set(true);
        });

        return rsx! {
            if mounted() {
                crate::security::rsx::AuthBar<R> { ..props}
            }
        };
    }

    #[cfg(not(feature = "web"))]
    {
        let _ = props;

        return rsx! {"Loading..."};
    }
}
