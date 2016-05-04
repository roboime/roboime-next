//!
//! Currently this is a simlpe proof of concpet.
//!

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
