use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct EmptyStateProps {
    pub title: String,
    #[props(default)]
    pub description: String,
    #[props(default)]
    pub action_text: String,
    #[props(default)]
    pub on_action: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn EmptyState(props: EmptyStateProps) -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center py-12 px-4",
            div {
                class: "text-gray-400 text-6xl mb-4",
                "📦"
            }
            h3 {
                class: "text-lg font-semibold text-gray-900 mb-2",
                {props.title}
            }
            if !props.description.is_empty() {
                p {
                    class: "text-sm text-gray-500 mb-4 text-center",
                    {props.description}
                }
            }
            if !props.action_text.is_empty() {
                button {
                    class: "px-4 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700",
                    onclick: move |evt| {
                        if let Some(handler) = props.on_action {
                            handler.call(evt);
                        }
                    },
                    {props.action_text}
                }
            }
        }
    }
}
