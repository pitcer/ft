use std::ffi::CString;
use std::os::unix::io::RawFd;
use std::slice;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::Result;
use lazy_static::lazy_static;
use nix::pty::Winsize;
use nix::sys::epoll::{EpollEvent, EpollFlags, EpollOp};
use nix::sys::signal::{SaFlags, SigAction, SigHandler, SigSet, Signal};
use nix::sys::{epoll, signal, wait};
use nix::unistd::{ForkResult, Pid};
use nix::{libc, pty, unistd};

use crate::color::Rgb;
use crate::display::Display;
use crate::font::FontRenderer;
use crate::input::InputTerminal;
use crate::point::Point;
use crate::terminal::cells::{Cells, RendererAction};
use crate::terminal::renderer::TerminalRenderer;

mod cells;
pub mod renderer;

lazy_static! {
    static ref SIGCHLD_FLAG: AtomicBool = AtomicBool::new(false);
}

extern "C" fn handle_sigchld(signal: libc::c_int) {
    let signal = Signal::try_from(signal).unwrap();
    let _result = wait::wait().unwrap();
    SIGCHLD_FLAG.store(signal == Signal::SIGCHLD, Ordering::Relaxed);
}

const BLOCK_CHARACTER: char = '█';
const BACKGROUND_COLOR: Rgb = Rgb::new(32, 32, 32);
const FONT_COLOR: Rgb = Rgb::new(249, 250, 244);

#[derive(Debug)]
pub struct Terminal {
    input: InputTerminal,
    renderer: TerminalRenderer,
    cells: Cells,
    master_fd: RawFd,
    child: Pid,
}

impl Terminal {
    pub fn new(input: InputTerminal, display: Display, font: FontRenderer) -> Result<Self> {
        let display_size = display.size();
        let cell_size = font.character_size(BLOCK_CHARACTER);
        let size = display_size.fit_cells(cell_size);
        let renderer = TerminalRenderer::new(display, font, size, cell_size);
        let cells = Cells::new(size);

        let size = Winsize {
            ws_row: size.height() as libc::c_ushort,
            ws_col: size.width() as libc::c_ushort,
            ws_xpixel: 0, // unused
            ws_ypixel: 0, // unused
        };
        let result = unsafe { pty::forkpty(Some(&size), None)? };
        match result.fork_result {
            ForkResult::Parent { child } => {
                let handler = SigHandler::Handler(handle_sigchld);
                let action = SigAction::new(handler, SaFlags::empty(), SigSet::empty());
                unsafe { signal::sigaction(Signal::SIGCHLD, &action)? };

                Ok(Self {
                    input,
                    renderer,
                    cells,
                    master_fd: result.master,
                    child,
                })
            }
            ForkResult::Child => {
                unistd::execv(
                    &CString::new("/usr/bin/sh")?,
                    &[CString::new("/usr/bin/sh")?],
                )?;
                unreachable!()
            }
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.renderer.fill_all(BACKGROUND_COLOR);
        self.push_string("ft 0.1.0");
        self.cells.carriage_return();
        let action = self.cells.new_line();
        if let Some(RendererAction::RenderAll) = action {
            self.render_all();
        }

        let epoll = epoll::epoll_create()?;
        let mut event = EpollEvent::new(EpollFlags::EPOLLIN, self.master_fd as u64);
        epoll::epoll_ctl(
            epoll,
            EpollOp::EpollCtlAdd,
            self.master_fd,
            Some(&mut event),
        )?;
        let mut event = EpollEvent::new(EpollFlags::EPOLLIN, libc::STDIN_FILENO as u64);
        epoll::epoll_ctl(
            epoll,
            EpollOp::EpollCtlAdd,
            libc::STDIN_FILENO,
            Some(&mut event),
        )?;

        let mut events = EpollEvent::empty();

        log::debug!("Entering main loop");
        while !SIGCHLD_FLAG.load(Ordering::Relaxed) {
            log::debug!("epoll_wait");
            let count = epoll::epoll_wait(epoll, slice::from_mut(&mut events), -1)?;
            assert_eq!(count, 1);

            let fd = events.data();
            log::debug!("epoll_wait passed, available data on fd {}", fd);
            let mut byte = 0;
            let bytes_read = unistd::read(fd as RawFd, slice::from_mut(&mut byte))?;
            assert_eq!(bytes_read, 1);
            log::debug!("read on fd {}: '{}' ({})", fd, byte as char, byte);

            if fd == 0 {
                unistd::write(self.master_fd, slice::from_ref(&byte))?;
                continue;
            }

            if byte == 13 {
                self.cells.carriage_return();
                continue;
            }

            if byte == 10 {
                let action = self.cells.new_line();
                if let Some(RendererAction::RenderAll) = action {
                    self.render_all();
                }
                continue;
            }

            self.push_character(byte as char);
        }
        log::debug!("Exiting main loop");

        Ok(())
    }

    fn push_string(&mut self, string: &str) {
        for character in string.chars() {
            self.push_character(character);
        }
    }

    fn push_character(&mut self, character: char) {
        let action = self.cells.push_character(character);
        match action {
            RendererAction::RenderAll => self.render_all(),
            RendererAction::RenderCell(cell) => {
                self.renderer.fill_cell(cell, BACKGROUND_COLOR);
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

    pub fn finish(self) -> Result<()> {
        self.input.finish()?;
        Ok(())
    }
}
