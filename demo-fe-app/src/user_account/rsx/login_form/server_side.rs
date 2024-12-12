use super::form_data::LoginWithPasswordProps;

use argentum_user_account_rest::ui::form_data::UserLoginsWithPasswordFormData;
use dioxus::prelude::*;

pub fn create_form_boilerplate(
    _props: LoginWithPasswordProps,
) -> (impl FnMut(Event<FormData>), UserLoginsWithPasswordFormData) {
    let on_submit: fn(Event<FormData>) = move |_| {};
    let data = UserLoginsWithPasswordFormData::new();

    (on_submit, data)
}
