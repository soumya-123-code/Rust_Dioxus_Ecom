use dioxus::prelude::*;
use crate::config::theme::{LazaTheme, ThemeMode};

#[derive(Clone, Debug)]
pub struct ThemeContext {
    pub mode: ThemeMode,
    pub theme: LazaTheme,
}

impl Default for ThemeContext {
    fn default() -> Self {
        Self {
            mode: ThemeMode::Light,
            theme: LazaTheme::light(),
        }
    }
}

impl ThemeContext {
    pub fn new(mode: ThemeMode) -> Self {
        Self {
            mode,
            theme: LazaTheme::new(mode),
        }
    }
    
    pub fn set_mode(&mut self, mode: ThemeMode) {
        self.mode = mode;
        self.theme = LazaTheme::new(mode);
    }
    
    pub fn toggle_theme(&mut self) {
        self.mode = match self.mode {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Blue,
            ThemeMode::Blue => ThemeMode::Light,
        };
        self.theme = LazaTheme::new(self.mode);
    }
}

pub fn use_theme() -> Signal<ThemeContext> {
    use_context::<Signal<ThemeContext>>()
}
