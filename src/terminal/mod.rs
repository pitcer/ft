use std::os::unix::io::RawFd;

use anyhow::Result;
use nix::errno::Errno;
use nix::sys::epoll::{EpollEvent, EpollFlags};
use nix::sys::wait;
use nix::sys::wait::{Id, WaitPidFlag};
use nix::unistd;

use crate::color::Rgb;
use crate::display::Display;
use crate::font::FontRenderer;
use crate::input::InputTerminal;
use crate::spatial::point::Point;
use crate::terminal::cells::line::RendererAction;
use crate::terminal::cells::Cells;
use crate::terminal::event::Events;
use crate::terminal::parser::{Parser, ParserAction};
use crate::terminal::renderer::TerminalRenderer;
use crate::terminal::shell::Shell;

mod cells;
mod event;
mod parser;
pub mod renderer;
mod shell;

const BLOCK_CHARACTER: char = '█';
const BACKGROUND_COLOR: Rgb = Rgb::new(32, 32, 32);
const FONT_COLOR: Rgb = Rgb::new(249, 250, 244);

#[derive(Debug)]
pub struct Terminal {
    input: InputTerminal,
    shell: Shell,
    renderer: TerminalRenderer,
    cells: Cells,
    events: Events,
    parser: Parser,
}

impl Terminal {
    pub fn new(
        input: InputTerminal,
        display: Display,
        font: FontRenderer,
        shell_path: &str,
    ) -> Result<Self> {
        let display_size = display.size();
        let cell_size = font.character_size(BLOCK_CHARACTER);
        let size = display_size.fit_cells(cell_size);
        let shell = Shell::spawn(size, shell_path, &[display.device_fd()])?;
        let renderer = TerminalRenderer::new(display, font, size, cell_size);
        let cells = Cells::new(size);
        let events = Events::new()?;
        let parser = Parser::new();

        Ok(Self {
            input,
            renderer,
            cells,
            events,
            shell,
            parser,
        })
    }

    pub fn run(mut self) -> Result<()> {
        self.renderer.fill_all(BACKGROUND_COLOR);
        self.push_string("ft 0.1.0");
        self.cells.carriage_return();
        let action = self.cells.new_line();
        if let Some(RendererAction::RenderAll) = action {
            self.render_all();
        }

        self.events.register_read_event(self.shell.master_fd())?;
        self.events
            .register_read_event(InputTerminal::TERMINAL_FD)?;
        self.events.register_read_event(self.shell.pid_fd())?;

        let mut events = [EpollEvent::empty(); 4];
        let mut bytes = [0; 4096];

        log::debug!("Entering main loop");
        loop {
            log::debug!("Waiting for new event...");
            let events = self.events.wait(&mut events)?;
            log::debug!("epoll_wait passed, available events: {:?}", events);

            for event in events {
                let source = event.data() as RawFd;
                let flags = event.events();

                if flags == EpollFlags::EPOLLHUP && source == self.shell.master_fd() {
                    self.events.unregister_event(self.shell.master_fd())?;
                    unistd::close(self.shell.master_fd())?;
                    log::debug!("Shell closed");
                    continue;
                }

                let pid_fd = self.shell.pid_fd();
                if source == pid_fd {
                    log::debug!("Wait for shell status");
                    let status = wait::waitid(Id::PIDFd(pid_fd), WaitPidFlag::WEXITED)?;
                    log::info!("Shell exit status: {:?}", status);
                    unistd::close(pid_fd)?;
                    self.finish()?;
                    return Ok(());
                }

                let bytes_read = unistd::read(source, &mut bytes)?;
                let bytes = &bytes[0..bytes_read];
                log::debug!(
                    "Read on fd {} ({} bytes): \"{}\" ({:?})",
                    source,
                    bytes.len(),
                    String::from_utf8_lossy(bytes),
                    bytes
                );

                if source == InputTerminal::TERMINAL_FD {
                    let result = unistd::write(self.shell.master_fd(), bytes);
                    match result {
                        Err(Errno::EBADF) => {
                            log::warn!("Cannot write to master fd");
                        }
                        Err(_) => {
                            result?;
                        }
                        _ => {}
                    }
                    continue;
                }

                self.handle_bytes(bytes)?;
            }
        }
    }

    fn handle_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        let mut refresh = false;
        for byte in bytes {
            let byte = *byte;
            let action = self.parser.push_byte(byte)?;
            match action {
                ParserAction::InsertCharacter(character) => {
                    self.push_character(Some(character));
                }
                ParserAction::CarriageReturn => {
                    self.cells.carriage_return();
                }
                ParserAction::NewLine => {
                    let action = self.cells.new_line();
                    if let Some(RendererAction::RenderAll) = action {
                        refresh = true;
                    }
                }
                ParserAction::MoveCursorUp(up) => {
                    for _ in 0..up {
                        self.cells.move_up();
                    }
                }
                ParserAction::MoveCursorDown(down) => {
                    for _ in 0..down {
                        let action = self.cells.new_line();
                        if let Some(RendererAction::RenderAll) = action {
                            refresh = true;
                        }
                    }
                }
                ParserAction::MoveCursorForward(forward) => {
                    for _ in 0..forward {
                        self.push_character(None);
                    }
                }
                ParserAction::MoveCursorBack(back) => {
                    for _ in 0..back {
                        self.cells.move_back();
                    }
                }
                ParserAction::MoveCursorToNextMultipleOf(multiple) => {
                    let horizontal_distance = self.cells.current_cell().horizontal_distance();
                    let forward = horizontal_distance % multiple;
                    for _ in 0..forward {
                        self.push_character(None);
                    }
                }
                ParserAction::EnableBracketedPasteMode => {
                    log::warn!("Unsupported sequence EnableBracketedPasteMode");
                }
                ParserAction::DisableBracketedPasteMode => {
                    log::warn!("Unsupported sequence DisableBracketedPasteMode");
                }
                ParserAction::Clear => {
                    self.cells.clear();
                    self.render_all();
                }
                ParserAction::MoreBytes => {}
                ParserAction::Ignore => {}
                ParserAction::UnsupportedSequence => {
                    log::warn!("Parser received invalid bytes");
                }
            }
        }

        if refresh {
            self.render_all();
        }

        Ok(())
    }

    fn push_string(&mut self, string: &str) {
        for character in string.chars() {
            self.push_character(Some(character));
        }
    }

    fn push_character(&mut self, character: Option<char>) {
        let action = self.cells.push_character(character);
        match action {
            RendererAction::RenderAll => self.render_all(),
            RendererAction::RenderCell(cell) => {
                self.renderer.fill_cell(cell, BACKGROUND_COLOR);
                let Some(character) = character else { return; };
                self.renderer
                    .render_character(character, cell, FONT_COLOR, BACKGROUND_COLOR);
            }
        }
    }

    fn render_all(&mut self) {
        self.renderer.fill_all(BACKGROUND_COLOR);

        for (index, line) in self.cells.iter().enumerate() {
            for (character_index, character) in line.iter().enumerate() {
                let character = character.character();
                let Some(character) = character else { continue; };
                let cell = Point::new(character_index as u32, index as u32);
                self.renderer
                    .render_character(character, cell, FONT_COLOR, BACKGROUND_COLOR);
            }
        }
    }

    fn finish(self) -> Result<()> {
        self.input.finish()?;
        self.events.finish()?;
        Ok(())
    }
}
