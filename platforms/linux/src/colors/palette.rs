use eframe::egui;
use serde::{Deserialize, Serialize};

/// Цвет в формате RGB
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_egui_color(self) -> egui::Color32 {
        egui::Color32::from_rgb(self.r, self.g, self.b)
    }

    pub fn to_egui_color_alpha(self, alpha: f32) -> egui::Color32 {
        egui::Color32::from_rgba_premultiplied(self.r, self.g, self.b, (alpha * 255.0) as u8)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new(30, 30, 30)
    }
}

/// Палитра цветов для темы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub background: Color,
    pub panel: Color,
    pub text: Color,
    pub accent: Color,
    pub button: Color,
    pub button_hover: Color,
    pub button_active: Color,
    pub border: Color,
    pub selection: Color,
    pub window_background: Color,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            background: Color::new(30, 30, 30),
            panel: Color::new(40, 40, 40),
            text: Color::new(220, 220, 220),
            accent: Color::new(100, 150, 255),
            button: Color::new(60, 60, 60),
            button_hover: Color::new(80, 80, 80),
            button_active: Color::new(100, 100, 100),
            border: Color::new(60, 60, 60),
            selection: Color::new(100, 150, 255),
            window_background: Color::new(35, 35, 35),
        }
    }
}

impl ColorPalette {
    pub fn to_egui_style(&self) -> egui::Style {
        let mut style = egui::Style::default();
        
        style.visuals.dark_mode = true;
        style.visuals.window_fill = self.window_background.to_egui_color();
        style.visuals.panel_fill = self.panel.to_egui_color();
        style.visuals.widgets.noninteractive.bg_fill = self.panel.to_egui_color();
        style.visuals.widgets.inactive.bg_fill = self.button.to_egui_color();
        style.visuals.widgets.hovered.bg_fill = self.button_hover.to_egui_color();
        style.visuals.widgets.active.bg_fill = self.button_active.to_egui_color();
        style.visuals.widgets.open.bg_fill = self.panel.to_egui_color();
        style.visuals.widgets.noninteractive.fg_stroke.color = self.text.to_egui_color();
        style.visuals.widgets.inactive.fg_stroke.color = self.text.to_egui_color();
        style.visuals.widgets.hovered.fg_stroke.color = self.text.to_egui_color();
        style.visuals.widgets.active.fg_stroke.color = self.text.to_egui_color();
        style.visuals.selection.bg_fill = self.selection.to_egui_color_alpha(0.5);
        
        style
    }

    pub fn save_to_file(&self, path: &str) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let palette: ColorPalette = serde_json::from_str(&json)?;
        Ok(palette)
    }
}
