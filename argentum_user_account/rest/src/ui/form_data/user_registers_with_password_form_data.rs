use crate::dto::response::{
    Status400Response, Status409Response, UserRegisteredSuccessfullyResponse,
};
use crate::dto::schema::RegistrationWithPasswordSchema;
use argentum_standard_infrastructure::invariant_violation::ViolationsDto;

use crate::dto::schema::UserName;

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct Values {
    pub email: Signal<String>,
    pub name: Signal<UserName>,
    pub password: Signal<String>,
    pub terms: Signal<bool>,
}

impl Values {
    pub fn new() -> Self {
        Self {
            email: use_signal(|| String::default()),
            name: use_signal(|| UserName::default()),
            password: use_signal(|| String::default()),
            terms: use_signal(|| bool::default()),
        }
    }
}
impl Into<RegistrationWithPasswordSchema> for Values {
    fn into(self) -> RegistrationWithPasswordSchema {
        RegistrationWithPasswordSchema {
            email: (self.email)(),
            name: (self.name)(),
            password: (self.password)(),
            terms: (self.terms)(),
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct UserRegistersWithPasswordFormData {
    pub values: Values,
    pub violations: ViolationsDto,
    pub errors: Signal<Vec<String>>,
    pub disabled: Signal<bool>,
}

impl UserRegistersWithPasswordFormData {
    pub fn new() -> Self {
        Self {
            values: Values::new(),
            violations: ViolationsDto::new(vec![], None),
            errors: use_signal(|| vec![]),
            disabled: use_signal(|| false),
        }
    }
}

#[deprecated(since = "0.3.0", note = "please use `*Callbacks` instead")]
#[derive(Clone, PartialEq, Props)]
pub struct UserRegistersWithPasswordFormProps {
    pub on_submit: EventHandler<FormEvent>,
    pub form_data: UserRegistersWithPasswordFormData,
}

pub struct UserRegistersWithPasswordCallbacks {
    pub on_error: EventHandler<String>,

    pub on_user_registered_successfully: EventHandler<UserRegisteredSuccessfullyResponse>,
    pub on_status_400: EventHandler<Status400Response>,
    pub on_status_409: EventHandler<Status409Response>,
}

impl Default for UserRegistersWithPasswordCallbacks {
    fn default() -> Self {
        Self {
            on_error: EventHandler::new(move |e: String| {
                dioxus_logger::tracing::error!("API error: `{:?}`", e);
            }),

            on_user_registered_successfully: EventHandler::new(
                move |_response: UserRegisteredSuccessfullyResponse| {},
            ),
            on_status_400: EventHandler::new(move |_response: Status400Response| {}),
            on_status_409: EventHandler::new(move |_response: Status409Response| {}),
        }
    }
}
