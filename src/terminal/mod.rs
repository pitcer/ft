use crate::dimension::{Cells, Dimensions, Pixels};
use crate::display::Display;
use crate::font::FontRenderer;
use crate::point::Point;
use crate::rgb::Rgb;
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
        let (metrics, raster) = self.font.create_raster(character);
        let line_metrics = self
            .font
            .font
            .horizontal_line_metrics(self.font.size)
            .unwrap();
        for x in 0..metrics.width {
            for y in 0..metrics.height {
                let index = y * metrics.width + x;
                let raster_pixel = raster[index];

                let point = Point::new(
                    (metrics.xmin + x as i32) as u32
                        + (cell.horizontal_distance() * self.cell_size.width()),
                    (((-metrics.bounds.height - metrics.bounds.ymin).floor()
                        + line_metrics.ascent.ceil()) as u32
                        + y as u32)
                        + (cell.vertical_distance() * self.cell_size.height()),
                );
                let mut display_pixel = self.display.pixel_mut(point);
                let rgb = Rgb::new_gray(raster_pixel);
                display_pixel.set_rgb(rgb);
            }
        }
    }
}
