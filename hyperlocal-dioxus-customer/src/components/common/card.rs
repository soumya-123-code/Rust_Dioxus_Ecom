use dioxus::prelude::*;
use crate::context::use_theme;

#[derive(Props, PartialEq, Clone)]
pub struct CardProps {
    pub children: Element,
    #[props(default)]
    pub class: String,
    #[props(default)]
    pub padding: CardPadding,
    #[props(default)]
    pub shadow: CardShadow,
    #[props(default)]
    pub onclick: Option<Callback<Event<MouseData>>>,
    #[props(default)]
    pub hoverable: bool,
}

#[derive(PartialEq, Clone)]
pub enum CardPadding {
    None,
    Small,
    Medium,
    Large,
}

impl Default for CardPadding {
    fn default() -> Self {
        Self::Medium
    }
}

#[derive(PartialEq, Clone)]
pub enum CardShadow {
    None,
    Small,
    Medium,
    Large,
}

impl Default for CardShadow {
    fn default() -> Self {
        Self::Medium
    }
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let theme = use_theme();
    let theme_read = theme.read();
    let colors = &theme_read.theme.colors;
    
    let padding_class = match props.padding {
        CardPadding::None => "",
        CardPadding::Small => "p-3",
        CardPadding::Medium => "p-4",
        CardPadding::Large => "p-6",
    };
    
    let shadow_class = match props.shadow {
        CardShadow::None => "",
        CardShadow::Small => "shadow-sm",
        CardShadow::Medium => "shadow-md",
        CardShadow::Large => "shadow-lg",
    };
    
    let hover_class = if props.hoverable {
        "hover:shadow-xl hover:scale-[1.02] cursor-pointer"
    } else {
        ""
    };
    
    let clickable = props.onclick.is_some();

    rsx! {
        div {
            class: "rounded-xl transition-all duration-300 {padding_class} {shadow_class} {hover_class} {props.class}",
            style: "background-color: {colors.surface}; border: 1px solid {colors.border};",
            onclick: move |evt| {
                if let Some(cb) = &props.onclick {
                    cb.call(evt);
                }
            },
            tabindex: if clickable { 0 } else { -1 },
            {props.children}
        }
    }
}

/// Card Header Component
#[component]
pub fn CardHeader(
    title: String,
    #[props(default)] subtitle: String,
    #[props(default)] action: Option<Element>,
    #[props(default)] class: String,
) -> Element {
    let theme = use_theme();
    let theme_read = theme.read();
    let colors = &theme_read.theme.colors;

    rsx! {
        div {
            class: "flex items-start justify-between mb-4 {class}",
            div {
                class: "flex-1",
                h3 {
                    class: "text-lg font-bold",
                    style: "color: {colors.text_primary};",
                    {title}
                }
                if !subtitle.is_empty() {
                    p {
                        class: "text-sm mt-1",
                        style: "color: {colors.text_secondary};",
                        {subtitle}
                    }
                }
            }
            if let Some(action_element) = action {
                div {
                    class: "ml-4",
                    {action_element}
                }
            }
        }
    }
}

/// Card Footer Component
#[component]
pub fn CardFooter(
    children: Element,
    #[props(default)] class: String,
) -> Element {
    let theme = use_theme();
    let theme_read = theme.read();
    let colors = &theme_read.theme.colors;

    rsx! {
        div {
            class: "mt-4 pt-4 {class}",
            style: "border-top: 1px solid {colors.divider};",
            {children}
        }
    }
}

/// Image Card Component (for product images, etc.)
#[component]
pub fn ImageCard(
    image_url: String,
    #[props(default)] alt: String,
    #[props(default)] aspect_ratio: String,
    #[props(default)] onclick: Option<Callback<Event<MouseData>>>,
    #[props(default)] class: String,
) -> Element {
    let theme = use_theme();
    let theme_read = theme.read();
    let colors = &theme_read.theme.colors;
    
    let aspect = if aspect_ratio.is_empty() {
        "aspect-square"
    } else {
        aspect_ratio.as_str()
    };
    
    let clickable = onclick.is_some();
    let hover_class = if clickable {
        "hover:scale-105 cursor-pointer"
    } else {
        ""
    };

    rsx! {
        div {
            class: "rounded-xl overflow-hidden transition-transform duration-300 {hover_class} {class}",
            style: "background-color: {colors.surface};",
            onclick: move |evt| {
                if let Some(cb) = &onclick {
                    cb.call(evt);
                }
            },
            div {
                class: "relative {aspect} w-full",
                img {
                    src: "{image_url}",
                    alt: "{alt}",
                    class: "absolute inset-0 w-full h-full object-cover",
                    loading: "lazy",
                }
            }
        }
    }
}

/// Stats Card Component
#[component]
pub fn StatsCard(
    title: String,
    value: String,
    #[props(default)] icon: Option<Element>,
    #[props(default)] trend: Option<String>,
    #[props(default)] trend_positive: bool,
    #[props(default)] class: String,
) -> Element {
    let theme = use_theme();
    let theme_read = theme.read();
    let colors = &theme_read.theme.colors;

    rsx! {
        Card {
            padding: CardPadding::Medium,
            shadow: CardShadow::Small,
            hoverable: false,
            class: "{class}",
            
            div {
                class: "flex items-start justify-between",
                div {
                    class: "flex-1",
                    p {
                        class: "text-sm font-medium mb-2",
                        style: "color: {colors.text_secondary};",
                        {title}
                    }
                    h3 {
                        class: "text-2xl font-bold",
                        style: "color: {colors.text_primary};",
                        {value}
                    }
                    if let Some(trend_value) = trend {
                        p {
                            class: "text-sm mt-2",
                            style: "color: {};",
                            if trend_positive { colors.success } else { colors.error },
                            {trend_value}
                        }
                    }
                }
                if let Some(icon_element) = icon {
                    div {
                        class: "flex items-center justify-center w-12 h-12 rounded-full",
                        style: "background: linear-gradient(135deg, {} 0%, {} 100%);",
                        {icon_element}
                    }
                }
            }
        }
    }
}

/// Skeleton Card for loading states
#[component]
pub fn SkeletonCard(
    #[props(default)] class: String,
) -> Element {
    rsx! {
        div {
            class: "rounded-xl p-4 animate-pulse {class}",
            div {
                class: "bg-gray-200 h-48 rounded-lg mb-4",
            }
            div {
                class: "space-y-3",
                div { class: "bg-gray-200 h-4 rounded w-3/4" }
                div { class: "bg-gray-200 h-4 rounded w-1/2" }
                div { class: "bg-gray-200 h-4 rounded w-2/3" }
            }
        }
    }
}
