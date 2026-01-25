use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct InputProps {
    #[props(default)]
    pub r#type: String,
    #[props(default)]
    pub placeholder: String,
    #[props(default)]
    pub value: String,
    pub oninput: EventHandler<FormEvent>,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub class: String,
    #[props(default)]
    pub label: String,
    #[props(default)]
    pub error: String,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    rsx! {
        div {
            class: "w-full",
            if !props.label.is_empty() {
                label {
                    class: "block text-sm font-medium text-gray-700 mb-1",
                    {props.label}
                }
            }
            input {
                class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 disabled:bg-gray-100 disabled:cursor-not-allowed {props.class}",
                r#type: if props.r#type.is_empty() { "text" } else { props.r#type.as_str() },
                placeholder: "{props.placeholder}",
                value: "{props.value}",
                disabled: props.disabled,
                oninput: props.oninput,
            }
            if !props.error.is_empty() {
                p {
                    class: "mt-1 text-sm text-red-600",
                    {props.error}
                }
            }
        }
    }
}
