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
//! Note that there is no referee in this example.
//!
//! ```no_run
//! use std::process::Command;
//! use roboime_next::prelude::*;
//! use roboime_next::{ai, grsim};
//!
//! let mut grsim = grsim::Builder::new()
//!     .vision_addr("224.5.23.2:11002").unwrap()
//!     .grsim_addr("127.0.0.1:20011").unwrap()
//!     .build().unwrap();
//!
//! let mut ai = ai::Builder::new(Command::new("./demo-ai"))
//!     .color(Yellow)
//!     .build().unwrap();
//!
//! grsim.wait_for_geom().unwrap();
//! let mut ai = ai.init(&grsim).unwrap();
//!
//! loop {
//!     grsim.recv_state().unwrap();
//!     let cmd = ai.update(&grsim).unwrap();
//!     grsim.send_command(cmd).unwrap();
//! }
//! ```
//!
//! To see that in action you will need a __demo-ai__, and a running __grSim__ with matching
//! settings (which could very well be running on another machine, just change the `grsim_addr`).

extern crate roboime_next_protocol as protocol;
extern crate net2;
#[macro_use] extern crate log;

pub use error::{Result, Error, ErrorKind};

mod error;
pub mod game;
pub mod ai;
pub mod base;
pub mod grsim;
pub mod sim;
pub mod prelude {
    pub use ::game::{Ball, Robot, Geometry, State, TeamInfo};
    pub use ::base::*;
    pub use ::ai::CommandAiExt;
}
