use crate::rsx::page::Blog;
use crate::rsx::wrapper::{AccountWrapper, SiteWrapper};
use argentum_contact_us_ui::rsx::page::ContactUs;
use argentum_oidc_ui::rsx::page::AuthCallback;
use argentum_skeleton_ui::rsx::page::{About, Home};
use argentum_standard_ui::rsx::page::NotFound;
use argentum_user_account_ui::rsx::page::Login;
use argentum_user_account_ui::rsx::page::Registration;
use argentum_user_account_ui::rsx::page::UserAccount;
use argentum_user_account_ui::rsx::page::{
    DeleteAccount, EmailConfirmation, ForgotPassword, ResetPassword,
};
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
pub enum Route {
    #[layout(SiteWrapper)]
        #[route("/")]
        Home {},

        #[route("/about")]
        About {},


        #[route("/contact-us")]
        ContactUs {},

        #[nest("/user-account")]
            #[route("/login")]
            Login {},
            #[route("/register")]
            Registration {},
            #[route("/email-confirmation?:token")]
            EmailConfirmation { token: String },
            #[route("/forgot-password")]
            ForgotPassword {},
            #[route("/reset-password?:token")]
            ResetPassword { token: String },
        #[end_nest]

        #[nest("/oidc")]
            #[route("/auth/callback?:provider&:code&:state")]
            AuthCallback { provider: String, code: String, state: String },
        #[end_nest]
        #[route("/blog/:id")]
        Blog { id: i32 },

    #[end_layout]

    #[layout(AccountWrapper)]
        #[nest("/user-account")]
            #[route("")]
            UserAccount {},
            #[route("/delete-account")]
            DeleteAccount {},
        #[end_nest]
    #[end_layout]

    #[route("/:..route")]
    NotFound { route: Vec<String> },
}
