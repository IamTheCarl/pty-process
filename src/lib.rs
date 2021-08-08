//! This crate adds a helper method to
//! [`std::process::Command`](::std::process::Command) (and optionally its
//! equivalent in various async frameworks) to allocate a pty to spawn the
//! process into. This allows for manipulation of interactive programs.
//!
//! The basic functionality is provided by the [`Command`](Command) trait in
//! this crate:
//!
//! ```no_run
//! use pty_process::Command as _;
//!
//! let mut cmd = std::process::Command::new("nethack");
//! let child = cmd.spawn_pty(Some(&pty_process::Size::new(24, 80))).unwrap();
//! ```
//!
//! The `child` instance returned by the call to
//! [`spawn_pty`](Command::spawn_pty) is a thin wrapper around the `Child`
//! struct associated with the `Command` module used. You can use it
//! identically to how you would use the normal
//! [`std::process::Child`](::std::process::Child) instance, but it also
//! provides additional methods for interacting with the pty:
//!
//! ```no_run
//! # use pty_process::{Command as _, Pty};
//! #
//! # let mut cmd = std::process::Command::new("nethack");
//! # let (mut child, mut pty) = cmd
//! #   .spawn_pty(Some(&pty_process::Size::new(24, 80))).unwrap();
//! use std::io::Write as _;
//!
//! pty.write_all(b"foo\n").unwrap();
//! pty.resize(&pty_process::Size::new(30, 100)).unwrap();
//! let status = child.wait().unwrap();
//! ```
//!
//! The available implementations are gated by features:
//! * `backend-std`: Add an implementation for
//!   [`std::process::Command`](::std::process::Command). Enabled by default.
//! * `backend-smol`: Add an implementation for
//!   [`smol::process::Command`](::smol::process::Command).
//! * `backend-async-std`: Add an implementation for
//!   [`async_std::process::Command`](::async_std::process::Command).
//! * `backend-tokio`: Add an implementation for
//!   [`tokio::process::Command`](::tokio::process::Command).
//!
//! Any number of backends may be enabled, depending on your needs.

mod command;
pub use command::Command;
mod error;
pub use error::{Error, Result};
mod pty;
pub use pty::Pty;
pub use pty::Size;

#[cfg(feature = "backend-async-std")]
pub mod async_io {
    pub use super::pty::async_io::Pty;
}
#[cfg(feature = "backend-smol")]
pub mod smol {
    pub use super::pty::smol::Pty;
}
#[cfg(feature = "backend-std")]
pub mod std {
    pub use super::pty::std::Pty;
}
#[cfg(feature = "backend-tokio")]
pub mod tokio {
    pub use super::pty::tokio::Pty;
}
