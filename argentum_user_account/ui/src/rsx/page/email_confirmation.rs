use dioxus::prelude::*;

#[cfg(feature = "web")]
enum Status {
    New,
    Failed(String),
    Success,
}

#[cfg(feature = "web")]
impl Status {
    pub(crate) fn fail(&self, err: &str) -> Self {
        Self::Failed(err.into())
    }
}

#[component]
pub fn EmailConfirmation(token: String) -> Element {
    #[cfg(not(feature = "web"))]
    return rsx! {};

    #[cfg(feature = "web")]
    {
        use argentum_rest_infrastructure::data_type::{
            AuthHeaderParams, EmptyQueryParams, HttpParams,
        };
        use argentum_standard_ui::{HomePageUrl, rsx::component::ErrorBlock};
        use argentum_user_account_rest::{
            client::Client,
            dto::{
                params::MemberVerifiesEmailParams, path_params::MemberVerifiesEmailPathParams,
                request::MemberVerifiesEmailRequest, schema::EmailVerificationSchema,
            },
        };

        use argentum_user_account_rest::dto::operation_response_enum::MemberVerifiesEmailOperationResponseEnum::Status200;
        use argentum_user_account_rest::dto::operation_response_enum::MemberVerifiesEmailOperationResponseEnum::Status400;
        use argentum_user_account_rest::dto::operation_response_enum::MemberVerifiesEmailOperationResponseEnum::Status401;
        use argentum_user_account_rest::dto::response::Status400Response::ApplicationProblemJson;
        use argentum_user_account_rest::dto::response::Status401Response::ApplicationProblemJson as Status401ApplicationProblemJson;

        let home_url = use_context::<HomePageUrl>().0;

        let auth_token = {
            use crate::security::ClientSideAuthenticator;

            let authenticator = use_context::<Signal<ClientSideAuthenticator>>();

            authenticator
                .read()
                .get_token()
                .expect("Can't get authentication token")
        };

        let mut status = use_signal(|| Status::New);

        let client = use_context::<Signal<Client>>();

        let home = home_url.clone();
        use_resource(move || {
            use argentum_rest_infrastructure::data_type::HttpRequest;

            let token = token.clone();
            let auth_token = auth_token.clone();
            let home = home.clone();

            async move {
                let req = MemberVerifiesEmailRequest::new(
                    EmailVerificationSchema::new(token.clone()),
                    MemberVerifiesEmailParams::new(
                        MemberVerifiesEmailPathParams::new(),
                        EmptyQueryParams {},
                        AuthHeaderParams::new(auth_token),
                    ),
                );

                match client.read().member_verifies_email(req).await {
                    Ok(r) => match r {
                        Status200(_) => {
                            status.set(Status::Success);
                        }
                        Status400(ApplicationProblemJson(j)) => {
                            status.set(Status::Failed(j.0.title));
                        }
                        Status401(Status401ApplicationProblemJson(j)) => {
                            status.set(Status::Failed(j.0.title));
                        }
                    },
                    Err(e) => status.set(Status::Failed(e)),
                }
            }
        });

        match &*status.read() {
            Status::New => rsx! {
                "Confirming email...",
            },
            Status::Failed(err) => rsx! {
                ErrorBlock { errors: vec![format!("Can't verify email: {}", err)]}

                div { class: "pt-10 pl-3",
                    a { class: "text-primary hover:underline", href: home_url, "Go to main page" }
                }
            },
            Status::Success => rsx! {
                section {
                    div { class: "container",
                        div { class: "pb-40 sm:max-w-sm",
                            h2 { "Email Confirmation" }
                            p { class: "text-center text-sm mt-10",
                                "Your email has been confirmed successfully."
                            }
                        }
                    }
                }
            },
        }
    }
}
