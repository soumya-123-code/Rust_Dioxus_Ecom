use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct CardProps {
    pub children: Element,
    #[props(default)]
    pub class: String,
    #[props(default)]
    pub onclick: Option<Callback<Event<MouseData>>>,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    rsx! {
        div {
            class: "bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow {props.class}",
            onclick: move |evt| {
                if let Some(cb) = &props.onclick {
                    cb.call(evt);
                }
            },
            {props.children}
        }
    }
}
