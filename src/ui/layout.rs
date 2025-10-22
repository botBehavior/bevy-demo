use bevy::prelude::*;

#[derive(Resource, Clone, Copy, Debug)]
pub struct UiLayout {
    safe_margin_px: f32,
    hud_max_width_percent: f32,
    compact: bool,
    shop_columns: usize,
    shop_max_width_percent: f32,
}

impl UiLayout {
    pub fn from_window(window: &Window) -> Self {
        let width = window.resolution.width();
        let compact = width < 800.0;
        let shop_columns = if width < 640.0 {
            1
        } else if width < 1080.0 {
            2
        } else {
            3
        };

        Self {
            safe_margin_px: if compact { 12.0 } else { 24.0 },
            hud_max_width_percent: if compact { 92.0 } else { 40.0 },
            compact,
            shop_columns,
            shop_max_width_percent: if compact { 96.0 } else { 70.0 },
        }
    }

    pub fn safe_margin(&self) -> f32 {
        self.safe_margin_px
    }

    pub fn hud_max_width_percent(&self) -> f32 {
        self.hud_max_width_percent
    }

    pub fn vertical_gap(&self) -> f32 {
        if self.compact { 6.0 } else { 10.0 }
    }

    pub fn horizontal_gap(&self) -> f32 {
        if self.compact { 6.0 } else { 10.0 }
    }

    pub fn is_compact(&self) -> bool {
        self.compact
    }

    pub fn shop_columns(&self) -> usize {
        self.shop_columns
    }

    pub fn shop_container_width(&self) -> Val {
        Val::Percent(self.shop_max_width_percent)
    }

    pub fn shop_card_basis(&self) -> Val {
        let columns = self.shop_columns.max(1) as f32;
        let gap = if self.compact { 4.0 } else { 12.0 };
        let percent = (100.0 / columns) - (gap / columns);
        Val::Percent(percent.clamp(40.0, 100.0))
    }

    pub fn shop_card_gap(&self) -> f32 {
        if self.compact { 8.0 } else { 16.0 }
    }
}
