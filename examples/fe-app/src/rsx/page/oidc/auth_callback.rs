use dioxus::prelude::*;

#[cfg(feature = "web")]
enum Status {
    New,
    Failed(String),
}

#[cfg(feature = "web")]
impl Status {
    pub(crate) fn fail(&self, err: &str) -> Self {
        Self::Failed(err.into())
    }
}

#[component]
pub fn AuthCallback(provider: String, code: String, state: Option<String>) -> Element {
    #[cfg(not(feature = "web"))]
    return rsx! {};

    #[cfg(feature = "web")]
    {
        let auth_token = {
            use argentum_user_account_ui::security::ClientSideAuthenticator;
            let authenticator = use_context::<Signal<ClientSideAuthenticator>>();

            authenticator
                .read()
                .get_token()
                .expect("Can't get authentication token")
        };

        use crate::route::Route;

        use argentum_oidc_rest::{
            client::Client,
            dto::{
                params::ExchangeCodeParams, path_params::ExchangeCodePathParams,
                request::ExchangeCodeRequest, schema::ExchangeCodeSchema,
            },
        };
        use argentum_rest_infrastructure::data_type::{
            AuthHeaderParams, EmptyQueryParams, HttpParams, HttpRequest,
        };

        use argentum_oidc_rest::dto::operation_response_enum::ExchangeCodeOperationResponseEnum::Status201;
        use argentum_oidc_rest::dto::operation_response_enum::ExchangeCodeOperationResponseEnum::Status400;
        use argentum_oidc_rest::dto::response::CodeExchangedSuccessfullyResponse::ApplicationJson;
        use argentum_oidc_rest::dto::response::Status400Response::ApplicationProblemJson;

        let mut status = use_signal(|| Status::New);

        let client = use_context::<Signal<Client>>();
        use argentum_standard_ui::service::redirect;
        use argentum_user_account_ui::security::ClientSideAuthenticator;
        let mut authenticator = use_context::<Signal<ClientSideAuthenticator>>();

        use_resource(move || {
            let code = code.clone();
            let provider = provider.clone();
            let auth_token = auth_token.clone();

            async move {
                let req = ExchangeCodeRequest::new(
                    ExchangeCodeSchema::new(code.clone(), provider.clone()),
                    ExchangeCodeParams::new(
                        ExchangeCodePathParams::new(),
                        EmptyQueryParams {},
                        AuthHeaderParams::new(auth_token),
                    ),
                );

                match client.read().exchange_code(req).await {
                    Ok(r) => match r {
                        Status201(ApplicationJson(j)) => {
                            authenticator
                                .write()
                                .auth_user(j.0.access_token, j.0.user_id);

                            let _ = redirect(Route::Home {});
                        }
                        Status400(ApplicationProblemJson(j)) => {
                            status.set(Status::Failed(j.0.title));
                        }
                    },
                    Err(e) => status.set(Status::Failed(e)),
                }
            }
        });

        match &*status.read() {
            Status::New => rsx! {
                "Getting token...",
            },
            Status::Failed(err) => rsx! {
                div { class: "alert alert-error alert-outline mt-10", role: "alert",
                    "Can't exchange a code for a token: {err}"
                }
                div { class: "pt-10 pl-3",
                    Link { class: "text-primary hover:underline", to: Route::Home {}, "Go to main page" }
                }
            },
        }
    }
}
