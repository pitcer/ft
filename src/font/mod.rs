use std::fs;
use std::num::NonZeroUsize;

use anyhow::{anyhow, Result};
use fontdue::{Font, FontSettings, Metrics};
use lru::LruCache;

use crate::dimension::{Dimensions, PixelsUnit};
use crate::font::raster_iterator::RasterIterator;

pub mod raster_iterator;

// SAFETY: 256 is not equal to 0
const CACHE_CAPACITY: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(256) };

#[derive(Debug)]
pub struct FontRenderer {
    size: f32,
    font: Font,
    ascent: i32,
    cache: LruCache<char, (Metrics, Vec<u8>)>,
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

        let cache = LruCache::new(CACHE_CAPACITY);

        Ok(Self {
            size,
            font,
            ascent,
            cache,
        })
    }

    pub fn create_raster(&mut self, character: char) -> RasterIterator {
        let rasterize = || self.font.rasterize(character, self.size);
        let (metrics, raster) = self.cache.get_or_insert(character, rasterize);
        RasterIterator::new(*metrics, raster, self.ascent)
    }

    pub fn character_size(&self, character: char) -> Dimensions<PixelsUnit> {
        let metrics = self.font.metrics(character, self.size);
        Dimensions::new(metrics.width as u32, metrics.height as u32)
    }
}
