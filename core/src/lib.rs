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
//! use roboime_next_core::{SharedGameState, ChildAi, GrSimInterface, InterfaceHandle};
//!
//! let game_state = SharedGameState::new();
//! let (tx, rx) = channel();
//!
//! let grsim = GrSimInterface::new()
//!     .vision_addr("224.5.23.2:11002").unwrap()
//!     .grsim_addr("127.0.0.1:20011").unwrap()
//!     .spawn(game_state.clone(), rx).unwrap();
//!
//! let ai= ChildAi::new(Command::new("./demo-ai"))
//!     .is_yellow(true)
//!     .spawn(game_state.clone(), tx).unwrap();
//!
//! (ai, grsim).join().unwrap();
//! ```
//!
//! To see that in action you will need a __demo-ai__ (there's one in this project which needs
//! Python 3 to run), and a running __grSim__ with matching settings (which could very well be
//! running on another machine, just change the `grsim_addr`).

extern crate roboime_next_protocol as protocol;
extern crate net2;
extern crate time;

pub use error::{Result, Error, ErrorKind};
pub use state::{GameState, RobotState, BallState, Position, Pose, SharedGameState};
pub use interface::InterfaceHandle;
pub use child_ai::ChildAi;
pub use grsim::GrSimInterface;

mod error;
mod state;
mod interface;
mod child_ai;
mod grsim;
