use dioxus::prelude::*;
use crate::context::use_theme;

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
    pub readonly: bool,
    #[props(default)]
    pub class: String,
    #[props(default)]
    pub label: String,
    #[props(default)]
    pub error: String,
    #[props(default)]
    pub helper_text: String,
    #[props(default)]
    pub icon_left: Option<Element>,
    #[props(default)]
    pub icon_right: Option<Element>,
    #[props(default)]
    pub size: InputSize,
    #[props(default)]
    pub variant: InputVariant,
}

#[derive(PartialEq, Clone)]
pub enum InputSize {
    Small,
    Medium,
    Large,
}

impl Default for InputSize {
    fn default() -> Self {
        Self::Medium
    }
}

#[derive(PartialEq, Clone)]
pub enum InputVariant {
    Outlined,
    Filled,
    Underlined,
}

impl Default for InputVariant {
    fn default() -> Self {
        Self::Outlined
    }
}

#[component]
pub fn Input(props: InputProps) -> Element {
    let theme = use_theme();
    let theme_read = theme.read();
    let colors = &theme_read.theme.colors;
    
    let base_styles = "w-full transition-all duration-300 focus:outline-none";
    
    let size_styles = match props.size {
        InputSize::Small => "px-3 py-2 text-sm",
        InputSize::Medium => "px-4 py-3 text-base",
        InputSize::Large => "px-5 py-4 text-lg",
    };
    
    let variant_styles = match props.variant {
        InputVariant::Outlined => format!(
            "border-2 rounded-xl focus:border-[{}] bg-white",
            colors.primary
        ),
        InputVariant::Filled => format!(
            "border-0 rounded-xl bg-gray-100 focus:bg-white focus:ring-2 focus:ring-[{}]",
            colors.primary
        ),
        InputVariant::Underlined => format!(
            "border-0 border-b-2 rounded-none focus:border-[{}] bg-transparent",
            colors.primary
        ),
    };
    
    let error_styles = if !props.error.is_empty() {
        "border-red-500 focus:border-red-500 focus:ring-red-500"
    } else {
        "border-gray-200"
    };
    
    let icon_padding = if props.icon_left.is_some() {
        "pl-12"
    } else if props.icon_right.is_some() {
        "pr-12"
    } else {
        ""
    };

    rsx! {
        div {
            class: "w-full",
            
            // Label
            if !props.label.is_empty() {
                label {
                    class: "block text-sm font-semibold mb-2",
                    style: "color: {colors.text_primary};",
                    {props.label}
                }
            }
            
            // Input Container
            div {
                class: "relative",
                
                // Left Icon
                if let Some(icon) = props.icon_left {
                    div {
                        class: "absolute left-4 top-1/2 transform -translate-y-1/2 pointer-events-none",
                        style: "color: {colors.text_secondary};",
                        {icon}
                    }
                }
                
                // Input Field
                input {
                    class: "{base_styles} {variant_styles} {size_styles} {error_styles} {icon_padding} {props.class}",
                    style: "color: {colors.text_primary};",
                    r#type: if props.r#type.is_empty() { "text" } else { props.r#type.as_str() },
                    placeholder: "{props.placeholder}",
                    value: "{props.value}",
                    disabled: props.disabled,
                    readonly: props.readonly,
                    oninput: props.oninput,
                }
                
                // Right Icon
                if let Some(icon) = props.icon_right {
                    div {
                        class: "absolute right-4 top-1/2 transform -translate-y-1/2 pointer-events-none",
                        style: "color: {colors.text_secondary};",
                        {icon}
                    }
                }
            }
            
            // Helper Text or Error
            if !props.error.is_empty() {
                p {
                    class: "mt-2 text-sm text-red-500 flex items-center",
                    svg {
                        class: "w-4 h-4 mr-1",
                        view_box: "0 0 20 20",
                        fill: "currentColor",
                        path {
                            fill_rule: "evenodd",
                            d: "M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z",
                            clip_rule: "evenodd",
                        }
                    }
                    {props.error}
                }
            } else if !props.helper_text.is_empty() {
                p {
                    class: "mt-2 text-sm",
                    style: "color: {colors.text_secondary};",
                    {props.helper_text}
                }
            }
        }
    }
}

/// Textarea Component (similar to Input but multiline)
#[derive(Props, PartialEq, Clone)]
pub struct TextareaProps {
    #[props(default)]
    pub placeholder: String,
    #[props(default)]
    pub value: String,
    pub oninput: EventHandler<FormEvent>,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub rows: u32,
    #[props(default)]
    pub class: String,
    #[props(default)]
    pub label: String,
    #[props(default)]
    pub error: String,
    #[props(default)]
    pub helper_text: String,
}

#[component]
pub fn Textarea(props: TextareaProps) -> Element {
    let theme = use_theme();
    let theme_read = theme.read();
    let colors = &theme_read.theme.colors;
    
    let rows = if props.rows == 0 { 4 } else { props.rows };
    
    let error_styles = if !props.error.is_empty() {
        "border-red-500 focus:border-red-500 focus:ring-red-500"
    } else {
        "border-gray-200"
    };

    rsx! {
        div {
            class: "w-full",
            
            // Label
            if !props.label.is_empty() {
                label {
                    class: "block text-sm font-semibold mb-2",
                    style: "color: {colors.text_primary};",
                    {props.label}
                }
            }
            
            // Textarea
            textarea {
                class: "w-full px-4 py-3 border-2 rounded-xl transition-all duration-300 focus:outline-none resize-y {error_styles} {props.class}",
                style: "color: {colors.text_primary}; min-height: {}px; focus:border-color: {};",
                placeholder: "{props.placeholder}",
                value: "{props.value}",
                disabled: props.disabled,
                rows: rows,
                oninput: props.oninput,
            }
            
            // Helper Text or Error
            if !props.error.is_empty() {
                p {
                    class: "mt-2 text-sm text-red-500",
                    {props.error}
                }
            } else if !props.helper_text.is_empty() {
                p {
                    class: "mt-2 text-sm",
                    style: "color: {colors.text_secondary};",
                    {props.helper_text}
                }
            }
        }
    }
}

/// Search Input Component
#[component]
pub fn SearchInput(
    #[props(default)] placeholder: String,
    #[props(default)] value: String,
    oninput: EventHandler<FormEvent>,
    #[props(default)] class: String,
) -> Element {
    let theme = use_theme();
    let theme_read = theme.read();
    let colors = &theme_read.theme.colors;
    
    let placeholder_text = if placeholder.is_empty() {
        "Search...".to_string()
    } else {
        placeholder
    };

    rsx! {
        div {
            class: "relative {class}",
            
            // Search Icon
            div {
                class: "absolute left-4 top-1/2 transform -translate-y-1/2 pointer-events-none",
                style: "color: {colors.text_secondary};",
                svg {
                    class: "w-5 h-5",
                    view_box: "0 0 20 20",
                    fill: "currentColor",
                    path {
                        fill_rule: "evenodd",
                        d: "M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z",
                        clip_rule: "evenodd",
                    }
                }
            }
            
            // Input
            input {
                class: "w-full pl-12 pr-4 py-3 bg-gray-100 border-0 rounded-xl text-base transition-all duration-300 focus:outline-none focus:bg-white focus:ring-2",
                style: "color: {colors.text_primary}; focus:ring-color: {colors.primary};",
                r#type: "search",
                placeholder: "{placeholder_text}",
                value: "{value}",
                oninput: oninput,
            }
        }
    }
}
