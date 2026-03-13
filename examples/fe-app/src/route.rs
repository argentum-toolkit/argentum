use crate::rsx::page::Blog;
use crate::rsx::wrapper::Wrapper;
use argentum_oidc_ui::rsx::page::AuthCallback;
use argentum_skeleton_ui::rsx::page::Home;
use argentum_standard_ui::rsx::page::NotFound;
use argentum_user_account_ui::rsx::page::EmailConfirmation;
use argentum_user_account_ui::rsx::page::Login;
use argentum_user_account_ui::rsx::page::Registration;
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Wrapper)]
        #[route("/")]
        Home {},
        #[nest("/user-account")]
            #[route("/login")]
            Login {},
            #[route("/register")]
            Registration {},
            #[route("/email-confirmation?:token")]
            EmailConfirmation { token: String },
        #[end_nest]
    
        #[nest("/oidc")]
            #[route("/auth/callback?:provider&:code&:state")]
            AuthCallback { provider: String, code: String, state: String },
        #[end_nest]
        #[route("/blog/:id")]
        Blog { id: i32 },

    #[end_layout]
    
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}
