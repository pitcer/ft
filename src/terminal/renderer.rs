use crate::dimension::{Cells, Dimensions, Pixels};
use crate::display::Display;
use crate::font::FontRenderer;
use crate::point::Point;

#[derive(Debug)]
pub struct TerminalRenderer {
    display: Display,
    font: FontRenderer,
    size: Dimensions<Cells>,
    cell_size: Dimensions<Pixels>,
}

impl TerminalRenderer {
    pub fn new(
        display: Display,
        font: FontRenderer,
        size: Dimensions<Cells>,
        cell_size: Dimensions<Pixels>,
    ) -> Self {
        Self {
            display,
            font,
            size,
            cell_size,
        }
    }

    pub fn render_character(&mut self, character: char, cell: Point<Cells>) {
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

    pub fn clear(&mut self) {
        self.display.clear();
    }
}
