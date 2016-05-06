extern crate roboime_next;

use std::sync::mpsc::channel;
use std::process::{exit, Command};
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

    (child_ai, grsim).join()
}

fn dump_error(res: Result<()>) -> bool {
    if let Err(err) = res {
        println!("{}", err.description());
        while let Some(err) = err.cause() {
            println!("caused by {}", err.description());
        }
        true
    } else {
        false
    }
}

fn main() {
    if dump_error(main_loop()) {
        exit(1);
    }
}
