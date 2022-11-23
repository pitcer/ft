use anyhow::Result;

use crate::display::Display;
use crate::font::FontRenderer;
use crate::input::InputTerminal;
use crate::point::Point;
use crate::terminal::lines::Lines;
use crate::terminal::renderer::TerminalRenderer;

pub mod lines;
pub mod renderer;

const BLOCK_CHARACTER: char = '█';

// End of transmission
const EOT: u8 = 4;

#[derive(Debug)]
pub struct Terminal {
    input: InputTerminal,
    renderer: TerminalRenderer,
    lines: Lines,
}

impl Terminal {
    pub fn new(input: InputTerminal, display: Display, font: FontRenderer) -> Self {
        let display_size = display.size();
        let cell_size = font.character_size(BLOCK_CHARACTER);
        let size = display_size.fit_cells(cell_size);
        let renderer = TerminalRenderer::new(display, font, size, cell_size);
        let lines = Lines::new(size.height() as usize);
        Self {
            input,
            renderer,
            lines,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.renderer.clear();
        self.lines.push_line("ft 0.1.0".to_owned());
        self.render();

        loop {
            let byte = self.input.read_byte();
            let Some(byte) = byte else { continue; };
            let byte = byte?;
            if byte == EOT {
                return Ok(());
            }

            self.renderer.clear();
            self.lines.push_line(format!("'{}' {}", byte as char, byte));
            self.render();
        }
    }

    fn render(&mut self) {
        for (index, line) in self.lines.iter().enumerate() {
            for (character_index, character) in line.text().chars().enumerate() {
                let cell = Point::new(character_index as u32, index as u32);
                self.renderer.render_character(character, cell);
            }
        }
    }

    pub fn finish(self) -> Result<()> {
        self.input.finish()?;
        Ok(())
    }
}
