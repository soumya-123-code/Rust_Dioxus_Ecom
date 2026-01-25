use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    #[props(default)]
    pub variant: ButtonVariant,
    #[props(default)]
    pub size: ButtonSize,
    pub children: Element,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub loading: bool,
    #[props(default)]
    pub onclick: Option<Callback<Event<MouseData>>>,
    #[props(default)]
    pub class: String,
    #[props(default)]
    pub r#type: String,
}

#[derive(PartialEq, Clone)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
    Danger,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        Self::Primary
    }
}

#[derive(PartialEq, Clone)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

impl Default for ButtonSize {
    fn default() -> Self {
        Self::Medium
    }
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let variant_class = match props.variant {
        ButtonVariant::Primary => "bg-indigo-600 hover:bg-indigo-700 text-white",
        ButtonVariant::Secondary => "bg-gray-600 hover:bg-gray-700 text-white",
        ButtonVariant::Outline => "border border-indigo-600 text-indigo-600 hover:bg-indigo-50",
        ButtonVariant::Ghost => "text-indigo-600 hover:bg-indigo-50",
        ButtonVariant::Danger => "bg-red-600 hover:bg-red-700 text-white",
    };

    let size_class = match props.size {
        ButtonSize::Small => "px-3 py-1.5 text-sm",
        ButtonSize::Medium => "px-4 py-2 text-base",
        ButtonSize::Large => "px-6 py-3 text-lg",
    };

    rsx! {
        button {
            class: "rounded-md font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed {variant_class} {size_class} {props.class}",
            disabled: props.disabled || props.loading,
            onclick: move |evt| {
                if let Some(cb) = &props.onclick {
                    cb.call(evt);
                }
            },
            r#type: if props.r#type.is_empty() { "button" } else { props.r#type.as_str() },
            if props.loading {
                span { "Loading..." }
            } else {
                {props.children}
            }
        }
    }
}
