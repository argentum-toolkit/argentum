use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct InputTextProps {
    pub value: String,
    pub id: String,
    pub name: String,
    pub input_type: String,
    pub label: String,

    oninput: EventHandler<String>,
}

#[component]
pub fn LabeledInput(props: InputTextProps) -> Element {
    rsx! {
        div {
            label { "for":"{props.id}", class:"block text-sm font-medium leading-6 text-gray-900 dark:text-body-color-dark", "{props.label}" }
            div { class:"mt-2",
                input {
                    id:"{props.id}", name:"{props.name}", "type":"{props.input_type}", autocomplete:"email", required:true,
                    value: "{props.value}",
                    class:"border-stroke dark:text-body-color-dark dark:shadow-two w-full rounded-sm border bg-[#f8f8f8] px-6 py-3 text-base text-body-color outline-none transition-all duration-300 focus:border-primary dark:border-transparent dark:bg-[#2C303B] dark:focus:border-primary dark:focus:shadow-none",
                    oninput: move |event| props.oninput.call(event.value())
                }
            }
        }
    }
}
