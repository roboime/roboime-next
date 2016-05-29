//! A library for abstracting the most common interfaces and filtering for making an AI for the
//! RoboCup's SSL robot soccer game.
//!
//! The main purpose is to provide an _stdio_ based interface for building an AI.  The usage of
//! this library is useful for building or customizing a user interface around that concept.
//!
//! # Examples
//!
//! The following will make a CLI program that will run an executable called `demo-ai` for it to
//! play on a running __grSim__ (or any simulator protocol compatible).
//!
//! Note that there is no referee state in this example.
//!
//! ```no_run
//! use std::sync::mpsc::channel;
//! use std::process::Command;
//! use roboime_next::prelude::*;
//! use roboime_next::{game, ai, grsim};
//!
//! let state = game::State::new();
//!
//! let mut grsim = grsim::Interface::new()
//!     .vision_addr("224.5.23.2:11002").unwrap()
//!     .grsim_addr("127.0.0.1:20011").unwrap()
//!     .spawn().unwrap();
//!
//! let mut ai = ai::Interface::new(Command::new("./demo-ai"))
//!     .is_yellow(true)
//!     .spawn().unwrap();
//!
//! grsim.wait_for_geom(&mut state).unwrap();
//! ai.push_init(&state).unwrap();
//!
//! loop {
//!     grsim.recv_to_state(&mut state).unwrap();
//!     ai.push_state(&state).unwrap();
//!     let cmd = ai.read_command(&state).unwrap();
//!     grsim.send_command(cmd).unwrap();
//! }
//!
//! ```
//!
//! To see that in action you will need a __demo-ai__ (there's one in this project which needs
//! Python 3 to run), and a running __grSim__ with matching settings (which could very well be
//! running on another machine, just change the `grsim_addr`).

extern crate roboime_next_protocol as protocol;
extern crate net2;
#[macro_use] extern crate log;

pub use error::{Result, Error, ErrorKind};
pub use interface::InterfaceHandle;

mod error;
mod interface;
pub mod game;
pub mod ai;
pub mod grsim;
pub mod sim;
pub mod prelude {
    pub use ::InterfaceHandle;
    pub use ::game::{Position, Pose};
    pub use ::ai::CommandAiExt;
}
