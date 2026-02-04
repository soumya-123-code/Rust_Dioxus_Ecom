use dioxus::prelude::*;
use crate::context::use_theme;

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
    pub full_width: bool,
    #[props(default)]
    pub icon_left: Option<Element>,
    #[props(default)]
    pub icon_right: Option<Element>,
    #[props(default)]
    pub onclick: Option<Callback<Event<MouseData>>>,
    #[props(default)]
    pub class: String,
    #[props(default)]
    pub r#type: String,
}

#[derive(PartialEq, Clone)]
pub enum ButtonVariant {
    Primary,      // Laza purple gradient
    Secondary,    // Outline with primary color
    Outline,      // Ghost with border
    Ghost,        // Text only
    Danger,       // Error color
    Success,      // Success color
    Light,        // Light background
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
    ExtraLarge,
}

impl Default for ButtonSize {
    fn default() -> Self {
        Self::Medium
    }
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let theme = use_theme();
    let theme_read = theme.read();
    let colors = &theme_read.theme.colors;
    
    let base_styles = "inline-flex items-center justify-center font-medium transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed";
    
    let variant_styles = match props.variant {
        ButtonVariant::Primary => format!(
            "text-white shadow-lg hover:shadow-xl active:scale-95 rounded-xl",
        ),
        ButtonVariant::Secondary => format!(
            "border-2 hover:bg-opacity-10 active:scale-95 rounded-xl",
        ),
        ButtonVariant::Outline => format!(
            "border border-gray-300 hover:border-gray-400 hover:bg-gray-50 rounded-xl",
        ),
        ButtonVariant::Ghost => format!(
            "hover:bg-gray-100 rounded-xl",
        ),
        ButtonVariant::Danger => format!(
            "bg-red-500 hover:bg-red-600 text-white shadow-lg hover:shadow-xl active:scale-95 rounded-xl",
        ),
        ButtonVariant::Success => format!(
            "text-white shadow-lg hover:shadow-xl active:scale-95 rounded-xl",
        ),
        ButtonVariant::Light => format!(
            "bg-gray-100 hover:bg-gray-200 text-gray-900 rounded-xl",
        ),
    };
    
    let size_styles = match props.size {
        ButtonSize::Small => "px-4 py-2 text-sm min-h-[36px]",
        ButtonSize::Medium => "px-6 py-3 text-base min-h-[48px]",
        ButtonSize::Large => "px-8 py-4 text-lg min-h-[56px]",
        ButtonSize::ExtraLarge => "px-10 py-5 text-xl min-h-[64px]",
    };
    
    let width_styles = if props.full_width { "w-full" } else { "" };
    
    // Dynamic background for Primary and Secondary variants
    let bg_style = match props.variant {
        ButtonVariant::Primary => format!(
            "background: linear-gradient(135deg, {} 0%, {} 100%);",
            colors.primary, colors.primary_dark
        ),
        ButtonVariant::Secondary => format!(
            "border-color: {}; color: {};",
            colors.primary, colors.primary
        ),
        ButtonVariant::Success => format!(
            "background: linear-gradient(135deg, {} 0%, #2DA44E 100%);",
            colors.success
        ),
        ButtonVariant::Ghost => format!("color: {};", colors.primary),
        _ => String::new(),
    };
    
    let focus_ring_color = format!("focus:ring-[{}]", colors.primary);

    rsx! {
        button {
            class: "{base_styles} {variant_styles} {size_styles} {width_styles} {focus_ring_color} {props.class}",
            style: "{bg_style}",
            disabled: props.disabled || props.loading,
            onclick: move |evt| {
                if let Some(cb) = &props.onclick {
                    cb.call(evt);
                }
            },
            r#type: if props.r#type.is_empty() { "button" } else { props.r#type.as_str() },
            
            // Icon left
            if let Some(icon) = props.icon_left {
                span { class: "mr-2", {icon} }
            }
            
            // Content
            if props.loading {
                LoadingSpinner {}
                span { class: "ml-2", "Loading..." }
            } else {
                {props.children}
            }
            
            // Icon right
            if let Some(icon) = props.icon_right {
                span { class: "ml-2", {icon} }
            }
        }
    }
}

#[component]
fn LoadingSpinner() -> Element {
    rsx! {
        svg {
            class: "animate-spin h-5 w-5",
            view_box: "0 0 24 24",
            fill: "none",
            circle {
                class: "opacity-25",
                cx: "12",
                cy: "12",
                r: "10",
                stroke: "currentColor",
                stroke_width: "4",
            }
            path {
                class: "opacity-75",
                fill: "currentColor",
                d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z",
            }
        }
    }
}

/// Icon Button variant (square with just icon)
#[component]
pub fn IconButton(
    icon: Element,
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] disabled: bool,
    #[props(default)] onclick: Option<Callback<Event<MouseData>>>,
    #[props(default)] class: String,
) -> Element {
    let theme = use_theme();
    let theme_read = theme.read();
    let colors = &theme_read.theme.colors;
    
    let base_styles = "inline-flex items-center justify-center rounded-full transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed";
    
    let size_styles = match size {
        ButtonSize::Small => "w-8 h-8 text-sm",
        ButtonSize::Medium => "w-10 h-10 text-base",
        ButtonSize::Large => "w-12 h-12 text-lg",
        ButtonSize::ExtraLarge => "w-14 h-14 text-xl",
    };
    
    let variant_styles = match variant {
        ButtonVariant::Primary => "text-white shadow-md hover:shadow-lg",
        ButtonVariant::Ghost => "hover:bg-gray-100",
        ButtonVariant::Light => "bg-gray-100 hover:bg-gray-200",
        _ => "hover:bg-gray-100",
    };
    
    let bg_style = match variant {
        ButtonVariant::Primary => format!(
            "background: linear-gradient(135deg, {} 0%, {} 100%);",
            colors.primary, colors.primary_dark
        ),
        _ => String::new(),
    };

    rsx! {
        button {
            class: "{base_styles} {variant_styles} {size_styles} {class}",
            style: "{bg_style}",
            disabled: disabled,
            onclick: move |evt| {
                if let Some(cb) = &onclick {
                    cb.call(evt);
                }
            },
            r#type: "button",
            {icon}
        }
    }
}
