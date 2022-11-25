use std::ffi::CString;
use std::os::unix::io::RawFd;
use std::slice;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::Result;
use lazy_static::lazy_static;
use nix::pty::Winsize;
use nix::sys::epoll::{EpollEvent, EpollFlags, EpollOp};
use nix::sys::signal::{SigHandler, Signal};
use nix::sys::{epoll, signal, wait};
use nix::unistd::{ForkResult, Pid};
use nix::{libc, pty, unistd};

use crate::display::Display;
use crate::font::FontRenderer;
use crate::input::InputTerminal;
use crate::point::Point;
use crate::terminal::lines::Lines;
use crate::terminal::renderer::TerminalRenderer;

pub mod lines;
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

#[derive(Debug)]
pub struct Terminal {
    input: InputTerminal,
    renderer: TerminalRenderer,
    lines: Lines,
    master_fd: RawFd,
    child: Pid,
}

impl Terminal {
    pub fn new(input: InputTerminal, display: Display, font: FontRenderer) -> Result<Self> {
        let display_size = display.size();
        let cell_size = font.character_size(BLOCK_CHARACTER);
        let size = display_size.fit_cells(cell_size);
        let renderer = TerminalRenderer::new(display, font, size, cell_size);
        let lines = Lines::new(size.height() as usize);

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
                unsafe {
                    signal::signal(Signal::SIGCHLD, handler)?;
                }

                Ok(Self {
                    input,
                    renderer,
                    lines,
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
        self.renderer.clear();
        self.lines.push_line("ft 0.1.0".to_owned());
        self.render();

        self.lines.push_line("".to_owned());

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

        while !SIGCHLD_FLAG.load(Ordering::Relaxed) {
            let count = epoll::epoll_wait(epoll, slice::from_mut(&mut events), -1)?;
            assert_eq!(count, 1);

            let fd = events.data();
            let mut byte = 0;
            let bytes_read = unistd::read(fd as RawFd, slice::from_mut(&mut byte))?;
            assert_eq!(bytes_read, 1);

            if fd == 0 {
                unistd::write(self.master_fd, slice::from_ref(&byte))?;
                continue;
            }

            if byte == 13 {
                self.lines.push_line("".to_owned());
                continue;
            }

            self.renderer.clear();
            self.lines.push_char(byte as char);
            self.render();
        }

        Ok(())
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
