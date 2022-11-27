use anyhow::Result;
use nix::sys::epoll;
use nix::sys::epoll::{EpollEvent, EpollFlags, EpollOp};
use std::os::unix::io::RawFd;

#[derive(Debug)]
pub struct Events {
    epoll: RawFd,
}

impl Events {
    pub fn new() -> Result<Self> {
        let epoll = epoll::epoll_create()?;
        Ok(Self { epoll })
    }

    pub fn register_read_event(&mut self, fd: RawFd) -> Result<()> {
        let mut event = EpollEvent::new(EpollFlags::EPOLLIN, fd as u64);
        epoll::epoll_ctl(self.epoll, EpollOp::EpollCtlAdd, fd, Some(&mut event))?;
        Ok(())
    }

    pub fn wait<'a>(&mut self, buffer: &'a mut [EpollEvent]) -> Result<&'a mut [EpollEvent]> {
        let count = epoll::epoll_wait(self.epoll, buffer, -1)?;
        Ok(&mut buffer[0..count])
    }
}
