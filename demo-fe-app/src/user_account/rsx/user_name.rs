use argentum_user_account_rest::dto::schema::UserName;
use dioxus::prelude::*;
#[derive(PartialEq, Props, Clone)]
pub struct UserNameProps {
    pub user_name: UserName,
    oninput: EventHandler<UserName>,
}

#[component]
pub fn UserNameComponent(mut props: UserNameProps) -> Element {
    let mut first = use_signal(|| props.user_name.first.clone());
    let mut last = use_signal(|| props.user_name.last.unwrap_or("".to_string()));

    let update = move || {
        props
            .oninput
            .call(UserName::new(first(), Some(last()), None))
    };

    rsx! {

        div {
            div { class:"flex items-center justify-between",
                label {"for":"first_name", class:"block text-sm font-medium leading-6 text-gray-900 dark:text-body-color-dark", "First Name"}
            }
            div { class:"mt-2",
                input {
                    id:"first_name", name:"first_name", "type":"text", autocomplete:"first-name", required:true,
                    class: "border-stroke dark:text-body-color-dark dark:shadow-two w-full rounded-sm border bg-[#f8f8f8] px-6 py-3 text-base text-body-color outline-none transition-all duration-300 focus:border-primary dark:border-transparent dark:bg-[#2C303B] dark:focus:border-primary dark:focus:shadow-none",
                    value: "{props.user_name.first}",
                    oninput: move |event| {
                        first.set(event.value());
                        update();
                    }
                }
            }
        }

        div {
            div { class:"flex items-center justify-between",
                label {"for":"last_name", class:"block text-sm font-medium leading-6 text-gray-900 dark:text-body-color-dark", "Last Name"}
            }
            div { class:"mt-2",
                input {
                    id:"last_name", name:"last_name", "type":"text", autocomplete:"last-name", required:true,
                    class: "border-stroke dark:text-body-color-dark dark:shadow-two w-full rounded-sm border bg-[#f8f8f8] px-6 py-3 text-base text-body-color outline-none transition-all duration-300 focus:border-primary dark:border-transparent dark:bg-[#2C303B] dark:focus:border-primary dark:focus:shadow-none",
                    value: "{last}",
                    oninput: move |event| {
                        last.set(event.value());
                        update();
                    }
                }
            }
        }
    }
}
