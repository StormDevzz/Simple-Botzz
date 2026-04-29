use crate::colors::palette::ColorPalette;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Менеджер тем для управления палитрами
pub struct ThemeManager {
    palette: Arc<Mutex<ColorPalette>>,
    config_path: PathBuf,
}

impl ThemeManager {
    pub fn new(config_path: PathBuf) -> Self {
        let palette = if config_path.exists() {
            ColorPalette::load_from_file(config_path.to_str().unwrap()).unwrap_or_default()
        } else {
            ColorPalette::default()
        };

        Self {
            palette: Arc::new(Mutex::new(palette)),
            config_path,
        }
    }

    pub fn get_palette(&self) -> ColorPalette {
        self.palette.lock().unwrap().clone()
    }

    pub fn set_palette(&self, palette: ColorPalette) {
        *self.palette.lock().unwrap() = palette;
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let palette = self.palette.lock().unwrap();
        palette.save_to_file(self.config_path.to_str().unwrap())?;
        Ok(())
    }

    pub fn get_palette_arc(&self) -> Arc<Mutex<ColorPalette>> {
        self.palette.clone()
    }
}
