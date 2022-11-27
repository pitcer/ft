use std::ffi::CString;
use std::os::unix::io::RawFd;

use anyhow::Result;
use nix::errno::Errno;
use nix::unistd::{ForkResult, Pid};
use nix::{libc, pty, unistd};

use crate::spatial::dimension::Dimensions;
use crate::spatial::CellsUnit;

#[derive(Debug)]
pub struct Shell {
    master_fd: RawFd,
    pid_fd: RawFd,
}

impl Shell {
    pub fn spawn(
        terminal_size: Dimensions<CellsUnit>,
        shell_path: &str,
        fds_to_close: &[RawFd],
    ) -> Result<Self> {
        let size = terminal_size.into();
        // SAFETY: In the child branch we only call `execv` and `close`, which are
        // async-signal-safe functions.
        let result = unsafe { pty::forkpty(Some(&size), None)? };
        match result.fork_result {
            ForkResult::Parent { child } => {
                let master_fd = result.master;
                let pid_fd = Self::pidfd_open(child)?;
                Ok(Self { master_fd, pid_fd })
            }
            ForkResult::Child => {
                for fd in fds_to_close {
                    unistd::close(*fd)?;
                }
                unistd::execv(&CString::new(shell_path)?, &[CString::new(shell_path)?])?;
                unreachable!()
            }
        }
    }

    fn pidfd_open(pid: Pid) -> nix::Result<RawFd> {
        let result = unsafe { libc::syscall(libc::SYS_pidfd_open, pid, 0) };
        Errno::result(result).map(|result| result as RawFd)
    }

    pub fn master_fd(&self) -> RawFd {
        self.master_fd
    }

    pub fn pid_fd(&self) -> RawFd {
        self.pid_fd
    }
}
