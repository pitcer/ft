use std::fs;

use anyhow::{anyhow, Result};
use fontdue::{Font, FontSettings, Metrics};

use crate::dimension::{Dimensions, Pixels};

#[derive(Debug)]
pub struct FontRenderer {
    pub(super) size: f32,
    pub(super) font: Font,
}

impl FontRenderer {
    pub fn new(size: u32, font_path: &str) -> Result<Self> {
        let size = size as f32;
        let font_settings = FontSettings::default();
        let font_data = fs::read(font_path)?;
        let font = Font::from_bytes(font_data, font_settings).map_err(|error| anyhow!(error))?;
        Ok(Self { size, font })
    }

    pub fn create_raster(&self, character: char) -> (Metrics, Vec<u8>) {
        self.font.rasterize(character, self.size)
    }

    pub fn character_size(&self, character: char) -> Dimensions<Pixels> {
        let metrics = self.font.metrics(character, self.size);
        Dimensions::new(metrics.width as u32, metrics.height as u32)
    }
}
