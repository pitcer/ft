use anyhow::Result;

use crate::args::Args;
use crate::display::Display;
use crate::font::FontRenderer;
use crate::input::InputTerminal;
use crate::terminal::Terminal;

mod args;
mod dimension;
mod display;
mod font;
mod input;
mod point;
mod rgb;
mod terminal;

fn main() -> Result<()> {
    let args = Args::parse()?;

    let input = InputTerminal::initialize()?;
    let display = Display::new(&args.framebuffer_device_path)?;
    let font = FontRenderer::new(args.font_size_px, &args.font_path)?;
    let mut terminal = Terminal::new(input, display, font)?;
    terminal.run()?;
    terminal.finish()?;

    Ok(())
}
