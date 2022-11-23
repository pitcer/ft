use std::io;
use std::io::Read;
use std::os::unix::io::RawFd;

use anyhow::Result;
use framebuffer::{Framebuffer, KdMode};
use nix::libc;
use nix::sys::termios;
use nix::sys::termios::{SetArg, Termios};

const TERMINAL_FD: RawFd = libc::STDIN_FILENO;

#[derive(Debug)]
pub struct InputTerminal {
    old_terminal_attributes: Termios,
}

impl InputTerminal {
    pub fn initialize() -> Result<Self> {
        Framebuffer::set_kd_mode(KdMode::Graphics)?;
        let old_terminal_attributes = Self::set_raw_mode()?;
        Ok(Self {
            old_terminal_attributes,
        })
    }

    fn set_raw_mode() -> Result<Termios> {
        let mut attributes = termios::tcgetattr(TERMINAL_FD)?;
        let old_attributes = attributes.clone();
        termios::cfmakeraw(&mut attributes);
        Self::set_attributes(&attributes)?;
        Ok(old_attributes)
    }

    pub fn read_byte(&self) -> Option<io::Result<u8>> {
        io::stdin().bytes().next()
    }

    pub fn finish(self) -> Result<()> {
        Self::set_attributes(&self.old_terminal_attributes)?;
        Framebuffer::set_kd_mode(KdMode::Text)?;
        Ok(())
    }

    fn set_attributes(attributes: &Termios) -> Result<()> {
        termios::tcsetattr(TERMINAL_FD, SetArg::TCSAFLUSH, attributes)?;
        Ok(())
    }
}
