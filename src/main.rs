extern crate roboime_next;

use std::sync::mpsc::channel;
use std::process::Command;
use std::error::Error;
use roboime_next::{Result, SharedGameState, ChildAi, GrSimInterface, InterfaceHandle};

fn main_loop() -> Result<()> {
    let game_state = SharedGameState::new();
    let (tx, rx) = channel();

    let mut grsim_cfg = GrSimInterface::new();
    try!(grsim_cfg.vision_addr("224.5.23.2:10002"));
    try!(grsim_cfg.grsim_addr("127.0.0.1:20011"));
    let grsim = try!(grsim_cfg.spawn(game_state.clone(), rx));

    let child_ai = try!(ChildAi::new(Command::new("./demo-ai")).is_yellow(true).spawn(game_state.clone(), tx));

    Ok(try!((child_ai, grsim).join()))
}

trait ResultExt {
    fn dump_error(&self);
}

impl ResultExt for Result<()> {
    fn dump_error(&self) {
        if let &Err(ref err) = self {
            println!("{}", err.description());
            while let Some(err) = err.cause() {
                println!("caused by {}", err.description());
            }
        }
    }
}

fn main() {
    main_loop().dump_error();
}
