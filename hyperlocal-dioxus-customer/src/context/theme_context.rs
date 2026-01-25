use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Clone, Debug)]
pub struct ThemeContext {
    pub theme: Theme,
}

impl Default for ThemeContext {
    fn default() -> Self {
        Self {
            theme: Theme::System,
        }
    }
}

pub fn use_theme() -> Signal<ThemeContext> {
    use_context::<Signal<ThemeContext>>()
}
