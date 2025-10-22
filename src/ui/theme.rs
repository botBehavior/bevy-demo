use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiTheme {
    Dark,
    Light,
}

#[derive(Resource, Clone, Copy)]
pub struct UiPalette {
    theme: UiTheme,
    text_primary: Color,
    text_secondary: Color,
    text_muted: Color,
    surface: Color,
    surface_elevated: Color,
    surface_highlight: Color,
    button_idle: Color,
    button_hover: Color,
    button_pressed: Color,
    button_outline: Color,
    danger_idle: Color,
    danger_hover: Color,
    danger_pressed: Color,
    success: Color,
    warning: Color,
    error: Color,
    accent: Color,
    health_bar: Color,
    toast_background: Color,
}

impl UiPalette {
    pub fn new(theme: UiTheme) -> Self {
        match theme {
            UiTheme::Dark => Self {
                theme,
                text_primary: Color::srgb(0.95, 0.98, 1.0),
                text_secondary: Color::srgb(0.78, 0.84, 0.94),
                text_muted: Color::srgb(0.58, 0.64, 0.75),
                surface: Color::srgba(0.05, 0.06, 0.09, 0.92),
                surface_elevated: Color::srgba(0.1, 0.11, 0.17, 0.96),
                surface_highlight: Color::srgba(0.18, 0.2, 0.28, 0.85),
                button_idle: Color::srgba(0.22, 0.24, 0.32, 0.95),
                button_hover: Color::srgba(0.3, 0.34, 0.44, 0.95),
                button_pressed: Color::srgba(0.16, 0.18, 0.26, 0.95),
                button_outline: Color::srgb(0.42, 0.58, 0.92),
                danger_idle: Color::srgba(0.45, 0.16, 0.22, 0.95),
                danger_hover: Color::srgba(0.62, 0.22, 0.3, 0.95),
                danger_pressed: Color::srgba(0.32, 0.1, 0.16, 0.95),
                success: Color::srgb(0.53, 0.84, 0.5),
                warning: Color::srgb(0.95, 0.77, 0.38),
                error: Color::srgb(0.98, 0.43, 0.5),
                accent: Color::srgb(0.44, 0.76, 1.0),
                health_bar: Color::srgb(0.98, 0.36, 0.36),
                toast_background: Color::srgba(0.0, 0.0, 0.0, 0.7),
            },
            UiTheme::Light => Self {
                theme,
                text_primary: Color::srgb(0.05, 0.07, 0.12),
                text_secondary: Color::srgb(0.24, 0.28, 0.38),
                text_muted: Color::srgb(0.5, 0.52, 0.6),
                surface: Color::srgba(0.96, 0.97, 1.0, 0.96),
                surface_elevated: Color::srgba(0.92, 0.94, 1.0, 0.98),
                surface_highlight: Color::srgba(0.85, 0.88, 1.0, 0.9),
                button_idle: Color::srgba(0.82, 0.85, 0.98, 0.95),
                button_hover: Color::srgba(0.74, 0.79, 0.98, 0.95),
                button_pressed: Color::srgba(0.65, 0.7, 0.9, 0.95),
                button_outline: Color::srgb(0.36, 0.46, 0.82),
                danger_idle: Color::srgba(0.92, 0.52, 0.56, 0.95),
                danger_hover: Color::srgba(0.86, 0.38, 0.42, 0.95),
                danger_pressed: Color::srgba(0.74, 0.28, 0.32, 0.95),
                success: Color::srgb(0.24, 0.6, 0.38),
                warning: Color::srgb(0.84, 0.58, 0.16),
                error: Color::srgb(0.76, 0.22, 0.28),
                accent: Color::srgb(0.2, 0.5, 0.9),
                health_bar: Color::srgb(0.92, 0.28, 0.32),
                toast_background: Color::srgba(1.0, 1.0, 1.0, 0.9),
            },
        }
    }

    pub fn theme(&self) -> UiTheme {
        self.theme
    }

    pub fn text_primary(&self) -> Color {
        self.text_primary
    }

    pub fn text_secondary(&self) -> Color {
        self.text_secondary
    }

    pub fn text_muted(&self) -> Color {
        self.text_muted
    }

    pub fn surface(&self) -> Color {
        self.surface
    }

    pub fn surface_elevated(&self) -> Color {
        self.surface_elevated
    }

    pub fn surface_highlight(&self) -> Color {
        self.surface_highlight
    }

    pub fn button_idle(&self) -> Color {
        self.button_idle
    }

    pub fn button_hover(&self) -> Color {
        self.button_hover
    }

    pub fn button_pressed(&self) -> Color {
        self.button_pressed
    }

    pub fn button_outline(&self) -> Color {
        self.button_outline
    }

    pub fn danger_idle(&self) -> Color {
        self.danger_idle
    }

    pub fn danger_hover(&self) -> Color {
        self.danger_hover
    }

    pub fn danger_pressed(&self) -> Color {
        self.danger_pressed
    }

    pub fn success(&self) -> Color {
        self.success
    }

    pub fn warning(&self) -> Color {
        self.warning
    }

    pub fn error(&self) -> Color {
        self.error
    }

    pub fn accent(&self) -> Color {
        self.accent
    }

    pub fn health_bar(&self) -> Color {
        self.health_bar
    }

    pub fn toast_background(&self) -> Color {
        self.toast_background
    }
}

pub fn detect_theme() -> UiTheme {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use web_sys::window;

        if let Some(window) = window() {
            if let Ok(Some(query)) = window.match_media("(prefers-color-scheme: dark)") {
                if query.matches() {
                    return UiTheme::Dark;
                } else {
                    return UiTheme::Light;
                }
            }
        }
    }

    UiTheme::Dark
}
