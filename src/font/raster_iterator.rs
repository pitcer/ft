use std::iter::Enumerate;
use std::slice::Iter;

use fontdue::Metrics;

use crate::color::Alpha;
use crate::dimension::PixelsUnit;
use crate::point::Point;

pub struct RasterIterator<'a> {
    metrics: Metrics,
    ascent: i32,
    raster_iterator: Enumerate<Iter<'a, u8>>,
}

impl<'a> RasterIterator<'a> {
    pub fn new(metrics: Metrics, raster: &'a Vec<u8>, ascent: i32) -> Self {
        debug_assert_eq!(raster.len(), metrics.width * metrics.height);

        let raster_iterator = raster.iter().enumerate();
        Self {
            metrics,
            ascent,
            raster_iterator,
        }
    }
}

impl<'a> Iterator for RasterIterator<'a> {
    type Item = (Point<PixelsUnit>, Alpha);

    fn next(&mut self) -> Option<Self::Item> {
        let (index, alpha) = self.raster_iterator.next()?;

        let horizontal_distance = self.metrics.xmin + (index % self.metrics.width) as i32;
        debug_assert!(horizontal_distance >= 0);

        let shift = (-self.metrics.bounds.height - self.metrics.bounds.ymin).floor() as i32;
        let vertical_distance = shift + self.ascent + (index / self.metrics.width) as i32;
        debug_assert!(vertical_distance >= 0);

        let point = Point::new(horizontal_distance as u32, vertical_distance as u32);
        let alpha = Alpha::new(*alpha);

        Some((point, alpha))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.raster_iterator.size_hint()
    }
}
