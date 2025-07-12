use crate::ui::form_data::AnonymousRegistersFormProps;

use argentum_standard_ui::rsx::form::{LabeledInput, Submit};
use argentum_standard_ui::rsx::ErrorBlock;

use dioxus::prelude::*;

#[component]
pub fn AnonymousRegistersForm(props: AnonymousRegistersFormProps) -> Element {
    let mut form_data = props.form_data;

    rsx! {
        form {
            action:"#",
            class:"space-y-6",
            "novalidate": true,
            onsubmit: props.on_submit,

            ErrorBlock {errors: (form_data.errors)()}


            Submit {
                title: "Submit".to_string(),
                disabled: (form_data.disabled)(),
            }
        }
    }
}
