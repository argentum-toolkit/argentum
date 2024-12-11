use super::form_data::LoginWithPasswordFormData;
use super::form_data::LoginWithPasswordProps;
use dioxus::prelude::*;

pub fn create_form_boilerplate(
    _props: LoginWithPasswordProps,
) -> (impl FnMut(Event<FormData>), LoginWithPasswordFormData) {
    let on_submit: fn(Event<FormData>) = move |_| {};
    let data = LoginWithPasswordFormData::new();

    (on_submit, data)
}
