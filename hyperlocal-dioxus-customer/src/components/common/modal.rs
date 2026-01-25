use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ModalProps {
    pub open: bool,
    pub on_close: EventHandler<MouseEvent>,
    pub title: String,
    pub children: Element,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    if !props.open {
        return rsx! { div {} };
    }

    rsx! {
        div {
            class: "fixed inset-0 z-50 overflow-y-auto",
            div {
                class: "flex items-center justify-center min-h-screen px-4",
                    div {
                        class: "fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity",
                        onclick: move |evt| { let _ = props.on_close.call(evt); },
                    }
                div {
                    class: "relative bg-white rounded-lg shadow-xl max-w-lg w-full p-6",
                    div {
                        class: "flex items-center justify-between mb-4",
                        h3 {
                            class: "text-lg font-semibold text-gray-900",
                            {props.title}
                        }
                        button {
                            class: "text-gray-400 hover:text-gray-600",
                            onclick: move |evt| props.on_close.call(evt),
                            "✕"
                        }
                    }
                    {props.children}
                }
            }
        }
    }
}
