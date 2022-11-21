use std::fs;

use anyhow::{anyhow, Result};
use fontdue::{Font, FontSettings};

use crate::dimension::{Dimensions, Pixels};
use crate::font::raster_iterator::RasterIterator;

pub mod raster_iterator;

#[derive(Debug)]
pub struct FontRenderer {
    size: f32,
    font: Font,
    ascent: i32,
}

impl FontRenderer {
    pub fn new(size: u32, font_path: &str) -> Result<Self> {
        let size = size as f32;

        let font_settings = FontSettings::default();
        let font_data = fs::read(font_path)?;
        let font = Font::from_bytes(font_data, font_settings).map_err(|error| anyhow!(error))?;

        let line_metrics = font
            .horizontal_line_metrics(size)
            .ok_or_else(|| anyhow!("Missing horizontal line metrics"))?;
        let ascent = line_metrics.ascent.ceil() as i32;

        Ok(Self { size, font, ascent })
    }

    pub fn create_raster(&self, character: char) -> RasterIterator {
        let (metrics, raster) = self.font.rasterize(character, self.size);
        RasterIterator::new(metrics, raster, self.ascent)
    }

    pub fn character_size(&self, character: char) -> Dimensions<Pixels> {
        let metrics = self.font.metrics(character, self.size);
        Dimensions::new(metrics.width as u32, metrics.height as u32)
    }
}
