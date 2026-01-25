use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct LoadingProps {
    #[props(default)]
    pub size: LoadingSize,
    #[props(default)]
    pub text: String,
}

#[derive(PartialEq, Clone)]
pub enum LoadingSize {
    Small,
    Medium,
    Large,
}

impl Default for LoadingSize {
    fn default() -> Self {
        Self::Medium
    }
}

#[component]
pub fn Loading(props: LoadingProps) -> Element {
    let size_class = match props.size {
        LoadingSize::Small => "h-4 w-4",
        LoadingSize::Medium => "h-8 w-8",
        LoadingSize::Large => "h-12 w-12",
    };

    rsx! {
        div {
            class: "flex flex-col items-center justify-center",
            div {
                class: "animate-spin rounded-full border-b-2 border-indigo-600 {size_class}",
            }
            if !props.text.is_empty() {
                p {
                    class: "mt-2 text-sm text-gray-600",
                    {props.text}
                }
            }
        }
    }
}
