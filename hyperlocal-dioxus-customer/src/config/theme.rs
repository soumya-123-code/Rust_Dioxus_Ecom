/// Laza Ecommerce UI Kit - Design System
/// Based on the Laza Mobile App UI Kit specifications
/// 
/// This module contains all design tokens including colors, typography,
/// spacing, and other design system values.

use serde::{Deserialize, Serialize};

/// Color tokens for Laza UI Kit
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LazaColors {
    // Primary Colors
    pub primary: &'static str,
    pub primary_dark: &'static str,
    pub primary_light: &'static str,
    
    // Secondary Colors
    pub secondary: &'static str,
    pub secondary_dark: &'static str,
    pub secondary_light: &'static str,
    
    // Accent Colors
    pub accent: &'static str,
    pub accent_light: &'static str,
    
    // Semantic Colors
    pub success: &'static str,
    pub warning: &'static str,
    pub error: &'static str,
    pub info: &'static str,
    
    // Neutral Colors
    pub background: &'static str,
    pub surface: &'static str,
    pub card_bg: &'static str,
    
    // Text Colors
    pub text_primary: &'static str,
    pub text_secondary: &'static str,
    pub text_tertiary: &'static str,
    pub text_disabled: &'static str,
    pub text_on_primary: &'static str,
    
    // Border & Divider
    pub border: &'static str,
    pub divider: &'static str,
    
    // Overlay
    pub overlay: &'static str,
    pub shadow: &'static str,
}

/// Light theme colors (Default Laza theme)
pub const LAZA_LIGHT_COLORS: LazaColors = LazaColors {
    // Purple/Violet primary brand color (Laza signature color)
    primary: "#9775FA",
    primary_dark: "#7950F2",
    primary_light: "#B197FC",
    
    // Orange secondary color
    secondary: "#FF6B6B",
    secondary_dark: "#FA5252",
    secondary_light: "#FF8787",
    
    // Accent colors
    accent: "#34C759",
    accent_light: "#5CD68C",
    
    // Semantic colors
    success: "#34C759",
    warning: "#FFB800",
    error: "#FF3B30",
    info: "#007AFF",
    
    // Backgrounds
    background: "#FAFAFA",
    surface: "#FFFFFF",
    card_bg: "#FFFFFF",
    
    // Text
    text_primary: "#1A1A1A",
    text_secondary: "#8E8E93",
    text_tertiary: "#C7C7CC",
    text_disabled: "#D1D1D6",
    text_on_primary: "#FFFFFF",
    
    // Borders
    border: "#E5E5E5",
    divider: "#F2F2F7",
    
    // Overlays
    overlay: "rgba(0, 0, 0, 0.5)",
    shadow: "rgba(0, 0, 0, 0.1)",
};

/// Dark theme colors (Laza dark mode)
pub const LAZA_DARK_COLORS: LazaColors = LazaColors {
    primary: "#9775FA",
    primary_dark: "#7950F2",
    primary_light: "#B197FC",
    
    secondary: "#FF6B6B",
    secondary_dark: "#FA5252",
    secondary_light: "#FF8787",
    
    accent: "#34C759",
    accent_light: "#5CD68C",
    
    success: "#34C759",
    warning: "#FFB800",
    error: "#FF3B30",
    info: "#007AFF",
    
    background: "#000000",
    surface: "#1C1C1E",
    card_bg: "#2C2C2E",
    
    text_primary: "#FFFFFF",
    text_secondary: "#EBEBF5",
    text_tertiary: "#8E8E93",
    text_disabled: "#636366",
    text_on_primary: "#FFFFFF",
    
    border: "#38383A",
    divider: "#2C2C2E",
    
    overlay: "rgba(0, 0, 0, 0.7)",
    shadow: "rgba(0, 0, 0, 0.3)",
};

/// Blue theme colors (Laza blue variant)
pub const LAZA_BLUE_COLORS: LazaColors = LazaColors {
    primary: "#007AFF",
    primary_dark: "#0051D5",
    primary_light: "#4DA3FF",
    
    secondary: "#5AC8FA",
    secondary_dark: "#32ADE6",
    secondary_light: "#7ED8FC",
    
    accent: "#34C759",
    accent_light: "#5CD68C",
    
    success: "#34C759",
    warning: "#FFB800",
    error: "#FF3B30",
    info: "#007AFF",
    
    background: "#F2F7FF",
    surface: "#FFFFFF",
    card_bg: "#FFFFFF",
    
    text_primary: "#1A1A1A",
    text_secondary: "#8E8E93",
    text_tertiary: "#C7C7CC",
    text_disabled: "#D1D1D6",
    text_on_primary: "#FFFFFF",
    
    border: "#E5E5E5",
    divider: "#F2F2F7",
    
    overlay: "rgba(0, 0, 0, 0.5)",
    shadow: "rgba(0, 0, 0, 0.1)",
};

/// Typography system based on Poppins font (Laza standard)
#[derive(Debug, Clone)]
pub struct LazaTypography {
    pub font_family: &'static str,
    
    // Font sizes (in pixels)
    pub h1: u32,
    pub h2: u32,
    pub h3: u32,
    pub h4: u32,
    pub h5: u32,
    pub h6: u32,
    pub body_large: u32,
    pub body: u32,
    pub body_small: u32,
    pub caption: u32,
    pub overline: u32,
    
    // Font weights
    pub weight_light: u32,
    pub weight_regular: u32,
    pub weight_medium: u32,
    pub weight_semibold: u32,
    pub weight_bold: u32,
    
    // Line heights (multipliers)
    pub line_height_tight: f32,
    pub line_height_normal: f32,
    pub line_height_relaxed: f32,
    
    // Letter spacing
    pub letter_spacing_tight: f32,
    pub letter_spacing_normal: f32,
    pub letter_spacing_wide: f32,
}

pub const LAZA_TYPOGRAPHY: LazaTypography = LazaTypography {
    font_family: "'Poppins', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif",
    
    // Font sizes
    h1: 32,
    h2: 28,
    h3: 24,
    h4: 20,
    h5: 18,
    h6: 16,
    body_large: 16,
    body: 14,
    body_small: 12,
    caption: 11,
    overline: 10,
    
    // Font weights
    weight_light: 300,
    weight_regular: 400,
    weight_medium: 500,
    weight_semibold: 600,
    weight_bold: 700,
    
    // Line heights
    line_height_tight: 1.2,
    line_height_normal: 1.5,
    line_height_relaxed: 1.8,
    
    // Letter spacing
    letter_spacing_tight: -0.02,
    letter_spacing_normal: 0.0,
    letter_spacing_wide: 0.02,
};

/// Spacing system (in pixels)
#[derive(Debug, Clone)]
pub struct LazaSpacing {
    pub xs: u32,
    pub sm: u32,
    pub md: u32,
    pub lg: u32,
    pub xl: u32,
    pub xxl: u32,
    pub xxxl: u32,
}

pub const LAZA_SPACING: LazaSpacing = LazaSpacing {
    xs: 4,
    sm: 8,
    md: 16,
    lg: 24,
    xl: 32,
    xxl: 48,
    xxxl: 64,
};

/// Border radius system
#[derive(Debug, Clone)]
pub struct LazaBorderRadius {
    pub none: u32,
    pub sm: u32,
    pub md: u32,
    pub lg: u32,
    pub xl: u32,
    pub full: &'static str,
}

pub const LAZA_BORDER_RADIUS: LazaBorderRadius = LazaBorderRadius {
    none: 0,
    sm: 4,
    md: 8,
    lg: 12,
    xl: 16,
    full: "9999px",
};

/// Shadow system
pub struct LazaShadows {
    pub none: &'static str,
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
}

pub const LAZA_SHADOWS: LazaShadows = LazaShadows {
    none: "none",
    sm: "0 1px 2px 0 rgba(0, 0, 0, 0.05)",
    md: "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)",
    lg: "0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)",
    xl: "0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)",
};

/// Breakpoints for responsive design
#[derive(Debug, Clone)]
pub struct LazaBreakpoints {
    pub mobile: u32,
    pub tablet: u32,
    pub desktop: u32,
}

pub const LAZA_BREAKPOINTS: LazaBreakpoints = LazaBreakpoints {
    mobile: 375,  // Mobile first (Laza is designed for mobile)
    tablet: 768,
    desktop: 1024,
};

/// Animation durations (in milliseconds)
#[derive(Debug, Clone)]
pub struct LazaAnimations {
    pub fast: u32,
    pub normal: u32,
    pub slow: u32,
}

pub const LAZA_ANIMATIONS: LazaAnimations = LazaAnimations {
    fast: 150,
    normal: 300,
    slow: 500,
};

/// Theme enum for switching between themes
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
    Blue,
}

impl Default for ThemeMode {
    fn default() -> Self {
        ThemeMode::Light
    }
}

/// Main theme struct that combines all design tokens
#[derive(Debug, Clone)]
pub struct LazaTheme {
    pub mode: ThemeMode,
    pub colors: LazaColors,
    pub typography: LazaTypography,
    pub spacing: LazaSpacing,
    pub border_radius: LazaBorderRadius,
    pub shadows: LazaShadows,
    pub breakpoints: LazaBreakpoints,
    pub animations: LazaAnimations,
}

impl LazaTheme {
    pub fn new(mode: ThemeMode) -> Self {
        let colors = match mode {
            ThemeMode::Light => LAZA_LIGHT_COLORS,
            ThemeMode::Dark => LAZA_DARK_COLORS,
            ThemeMode::Blue => LAZA_BLUE_COLORS,
        };
        
        Self {
            mode,
            colors,
            typography: LAZA_TYPOGRAPHY.clone(),
            spacing: LAZA_SPACING.clone(),
            border_radius: LAZA_BORDER_RADIUS.clone(),
            shadows: LAZA_SHADOWS,
            breakpoints: LAZA_BREAKPOINTS.clone(),
            animations: LAZA_ANIMATIONS.clone(),
        }
    }
    
    pub fn light() -> Self {
        Self::new(ThemeMode::Light)
    }
    
    pub fn dark() -> Self {
        Self::new(ThemeMode::Dark)
    }
    
    pub fn blue() -> Self {
        Self::new(ThemeMode::Blue)
    }
}

impl Default for LazaTheme {
    fn default() -> Self {
        Self::light()
    }
}

/// Helper functions for CSS generation
impl LazaTheme {
    pub fn font_size_css(&self, size: u32) -> String {
        format!("{}px", size)
    }
    
    pub fn spacing_css(&self, spacing: u32) -> String {
        format!("{}px", spacing)
    }
    
    pub fn border_radius_css(&self, radius: u32) -> String {
        format!("{}px", radius)
    }
    
    pub fn generate_css_variables(&self) -> String {
        format!(
            r#"
            :root {{
                /* Colors */
                --color-primary: {};
                --color-primary-dark: {};
                --color-primary-light: {};
                --color-secondary: {};
                --color-accent: {};
                --color-success: {};
                --color-warning: {};
                --color-error: {};
                --color-info: {};
                --color-background: {};
                --color-surface: {};
                --color-text-primary: {};
                --color-text-secondary: {};
                --color-border: {};
                
                /* Typography */
                --font-family: {};
                --font-size-h1: {}px;
                --font-size-h2: {}px;
                --font-size-body: {}px;
                
                /* Spacing */
                --spacing-xs: {}px;
                --spacing-sm: {}px;
                --spacing-md: {}px;
                --spacing-lg: {}px;
                --spacing-xl: {}px;
                
                /* Border Radius */
                --border-radius-sm: {}px;
                --border-radius-md: {}px;
                --border-radius-lg: {}px;
                
                /* Shadows */
                --shadow-sm: {};
                --shadow-md: {};
                --shadow-lg: {};
            }}
            "#,
            self.colors.primary,
            self.colors.primary_dark,
            self.colors.primary_light,
            self.colors.secondary,
            self.colors.accent,
            self.colors.success,
            self.colors.warning,
            self.colors.error,
            self.colors.info,
            self.colors.background,
            self.colors.surface,
            self.colors.text_primary,
            self.colors.text_secondary,
            self.colors.border,
            self.typography.font_family,
            self.typography.h1,
            self.typography.h2,
            self.typography.body,
            self.spacing.xs,
            self.spacing.sm,
            self.spacing.md,
            self.spacing.lg,
            self.spacing.xl,
            self.border_radius.sm,
            self.border_radius.md,
            self.border_radius.lg,
            self.shadows.sm,
            self.shadows.md,
            self.shadows.lg,
        )
    }
}
