use crate::standard::rsx::LabeledInput;
use argentum_user_account_rest::dto::schema::UserName;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct UserNameProps {
    pub user_name: UserName,
    oninput: EventHandler<UserName>,
}

#[component]
pub fn UserNameComponent(props: UserNameProps) -> Element {
    let UserNameProps { user_name, .. } = props.clone();

    let mut first = use_signal(|| user_name.first.clone());
    let mut last = use_signal(|| user_name.last.unwrap_or("".to_string()));

    let update = move || {
        props
            .oninput
            .call(UserName::new(first(), Some(last()), None))
    };

    rsx! {
        LabeledInput {
            id: "first_name".to_string(),
            name: "first_name".to_string(),
            label: "First Name".to_string(),
            input_type: "text".to_string(),
            value: first,
            oninput: move |event: String| {
                first.set(event);
                update();
            },
        },

        LabeledInput {
            id: "last_name".to_string(),
            name: "last_name".to_string(),
            label: "Last Name".to_string(),
            input_type: "text".to_string(),
            value: last,
            oninput: move |event: String| {
                last.set(event);
                update();
            },
        },
    }
}
