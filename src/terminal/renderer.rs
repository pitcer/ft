use crate::dimension::{CellsUnit, Dimensions, PixelsUnit};
use crate::display::Display;
use crate::font::FontRenderer;
use crate::point::Point;
use crate::rgb::Rgb;

#[derive(Debug)]
pub struct TerminalRenderer {
    display: Display,
    font: FontRenderer,
    size: Dimensions<CellsUnit>,
    cell_size: Dimensions<PixelsUnit>,
}

impl TerminalRenderer {
    pub fn new(
        display: Display,
        font: FontRenderer,
        size: Dimensions<CellsUnit>,
        cell_size: Dimensions<PixelsUnit>,
    ) -> Self {
        Self {
            display,
            font,
            size,
            cell_size,
        }
    }

    pub fn render_character(&mut self, character: char, cell: Point<CellsUnit>) {
        debug_assert!(self.size.contains(cell));

        let cell_origin = cell.to_pixels(self.cell_size);
        let raster = self.font.create_raster(character);
        for (point, color) in raster {
            debug_assert!(self.cell_size.contains(point));

            let display_point = point.with_origin(cell_origin);
            let mut display_pixel = self.display.pixel_mut(display_point);
            display_pixel.set_rgb(color);
        }
    }

    pub fn fill_cell(&mut self, cell: Point<CellsUnit>, color: Rgb) {
        debug_assert!(self.size.contains(cell));

        let cell_origin = cell.to_pixels(self.cell_size);
        for horizontal_distance in 0..self.cell_size.width() {
            for vertical_distance in 0..self.cell_size.height() {
                let point = Point::new(horizontal_distance, vertical_distance);
                let point = point.with_origin(cell_origin);
                let mut pixel = self.display.pixel_mut(point);
                pixel.set_rgb(color);
            }
        }
    }

    pub fn fill_all(&mut self, color: Rgb) {
        let size = self.display.size();
        for horizontal_distance in 0..size.width() {
            for vertical_distance in 0..size.height() {
                let point = Point::new(horizontal_distance, vertical_distance);
                let mut pixel = self.display.pixel_mut(point);
                pixel.set_rgb(color);
            }
        }
    }

    pub fn clear(&mut self) {
        self.display.clear();
    }
}
