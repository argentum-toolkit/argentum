use dioxus::prelude::*;

#[component]
pub fn UserName() -> Element {
    let first = use_signal(|| "".to_string());
    rsx! {

        div {
            div { class:"flex items-center justify-between",
                label {"for":"first_name", class:"block text-sm font-medium leading-6 text-gray-900 dark:text-body-color-dark", "First Name"}
            }
            div { class:"mt-2",
                input {
                    id:"first_name", name:"first_name", "type":"text", autocomplete:"first-name", required:true,
                    class: "border-stroke dark:text-body-color-dark dark:shadow-two w-full rounded-sm border bg-[#f8f8f8] px-6 py-3 text-base text-body-color outline-none transition-all duration-300 focus:border-primary dark:border-transparent dark:bg-[#2C303B] dark:focus:border-primary dark:focus:shadow-none"
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
                    class: "border-stroke dark:text-body-color-dark dark:shadow-two w-full rounded-sm border bg-[#f8f8f8] px-6 py-3 text-base text-body-color outline-none transition-all duration-300 focus:border-primary dark:border-transparent dark:bg-[#2C303B] dark:focus:border-primary dark:focus:shadow-none"
                }
            }
        }

        div {
            div { class:"flex items-center justify-between",
                label {"for":"patronymic", class:"block text-sm font-medium leading-6 text-gray-900 dark:text-body-color-dark", "Patronymic"}
            }
            div { class:"mt-2",
                input {
                    id:"patronymic", name:"patronymic", "type":"text", autocomplete:"patronymic", required:true,
                    class: "border-stroke dark:text-body-color-dark dark:shadow-two w-full rounded-sm border bg-[#f8f8f8] px-6 py-3 text-base text-body-color outline-none transition-all duration-300 focus:border-primary dark:border-transparent dark:bg-[#2C303B] dark:focus:border-primary dark:focus:shadow-none"
                }
            }
        }
    }
}
