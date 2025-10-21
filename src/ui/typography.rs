use bevy::prelude::*;

#[derive(Resource, Clone, Copy, Debug)]
pub struct UiTypography {
    scale: f32,
    title: f32,
    subtitle: f32,
    body: f32,
    label: f32,
    caption: f32,
}

impl UiTypography {
    pub fn from_window(window: &Window) -> Self {
        let logical_height = window.resolution.height();
        let logical_width = window.resolution.width();
        let reference = logical_height.min(logical_width).max(400.0);
        let scale = (reference / 720.0).clamp(0.85, 1.35);

        Self {
            scale,
            title: 36.0 * scale,
            subtitle: 24.0 * scale,
            body: 18.0 * scale,
            label: 16.0 * scale,
            caption: 14.0 * scale,
        }
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn title(&self) -> f32 {
        self.title
    }

    pub fn subtitle(&self) -> f32 {
        self.subtitle
    }

    pub fn body(&self) -> f32 {
        self.body
    }

    pub fn label(&self) -> f32 {
        self.label
    }

    pub fn caption(&self) -> f32 {
        self.caption
    }

    pub fn hud_primary(&self) -> f32 {
        self.subtitle
    }

    pub fn hud_secondary(&self) -> f32 {
        self.body
    }

    pub fn hud_caption(&self) -> f32 {
        self.label
    }

    pub fn shop_title(&self) -> f32 {
        self.title
    }

    pub fn shop_heading(&self) -> f32 {
        self.subtitle
    }

    pub fn shop_body(&self) -> f32 {
        self.body
    }

    pub fn shop_label(&self) -> f32 {
        self.label
    }

    pub fn shop_caption(&self) -> f32 {
        self.caption
    }
}
