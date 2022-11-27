use std::fs::File;
use std::panic;

use anyhow::Result;
use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};

use crate::args::Args;
use crate::display::Display;
use crate::font::FontRenderer;
use crate::input::InputTerminal;
use crate::terminal::Terminal;

mod args;
mod color;
mod display;
mod font;
mod input;
mod spatial;
mod terminal;

fn main() -> Result<()> {
    let config = ConfigBuilder::new().set_time_format_rfc3339().build();
    let log_file = File::create("/tmp/ft.log")?;
    WriteLogger::init(LevelFilter::Debug, config, log_file)?;

    panic::set_hook(Box::new(|info| {
        log::error!("Panic occurred: {:#?}", info);
    }));

    let result = start();
    if let Err(ref error) = result {
        log::error!("Error occurred: {:#?}", error);
    }
    result
}

fn start() -> Result<()> {
    log::info!("Initializing ft...");

    let args = Args::parse()?;
    log::debug!("Command line arguments parsed: {:?}", args);

    let input = InputTerminal::initialize()?;
    let display = Display::new(&args.framebuffer_device_path)?;
    let font = FontRenderer::new(args.font_size_px, &args.font_path)?;
    let mut terminal = Terminal::new(input, display, font)?;
    terminal.run()?;
    terminal.finish()?;

    Ok(())
}
