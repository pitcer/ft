use std::collections::VecDeque;

use crate::display::Display;
use crate::font::FontRenderer;
use crate::point::Point;
use crate::terminal::line::Line;
use crate::terminal::renderer::TerminalRenderer;

pub mod line;
pub mod renderer;

const BLOCK_CHARACTER: char = '█';

#[derive(Debug)]
pub struct Terminal {
    renderer: TerminalRenderer,
    lines: VecDeque<Line>,
}

impl Terminal {
    pub fn new(display: Display, font: FontRenderer) -> Self {
        let display_size = display.size();
        let cell_size = font.character_size(BLOCK_CHARACTER);
        let size = display_size.fit_cells(cell_size);
        let renderer = TerminalRenderer::new(display, font, size, cell_size);
        let lines = VecDeque::with_capacity(size.height() as usize);
        Self { renderer, lines }
    }

    pub fn render(&mut self) {
        self.renderer.clear();
        self.lines
            .push_back(Line::new("Framebuffer terminal".to_owned()));
        self.lines.push_back(Line::new("0.1.0".to_owned()));

        for (index, line) in self.lines.iter().enumerate() {
            for (character_index, character) in line.text().chars().enumerate() {
                let cell = Point::new(character_index as u32, index as u32);
                self.renderer.render_character(character, cell);
            }
        }
    }
}
