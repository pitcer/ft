use crate::dimension::{Cells, Dimensions, Pixels};
use crate::display::Display;
use crate::font::FontRenderer;
use crate::point::Point;
use crate::terminal::line::Line;

pub mod line;

const BLOCK_CHARACTER: char = '█';

#[derive(Debug)]
pub struct Terminal {
    display: Display,
    font: FontRenderer,
    size: Dimensions<Cells>,
    cell_size: Dimensions<Pixels>,
    lines: Vec<Line>,
}

impl Terminal {
    pub fn new(display: Display, font: FontRenderer) -> Self {
        let display_size = display.size();
        let cell_size = font.character_size(BLOCK_CHARACTER);
        let size = display_size.fit_cells(cell_size);
        let lines = vec![Line::new(); size.height() as usize];
        Self {
            display,
            font,
            size,
            cell_size,
            lines,
        }
    }

    pub fn render(&mut self) {
        self.display.clear();
        for (index, character) in "Framebuffer terminal".chars().enumerate() {
            self.render_character(character, Point::new(index as u32, 0));
        }
    }

    fn render_character(&mut self, character: char, cell: Point<Cells>) {
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
}
