extern crate roboime_next;

use std::net::Ipv4Addr;
use std::sync::mpsc::channel;
use std::process::Command;
use std::error::Error;
use roboime_next::{Result, SharedGameState, GrSimUpdaterBuilder, GrSimCommanderBuilder, ChildAiBuilder};

fn main_loop() -> Result<()> {
    let game_state = SharedGameState::new();
    let (tx, rx) = channel();

    let grsim_updater = try!(GrSimUpdaterBuilder::new().port(10002).spawn(game_state.clone()));
    let grsim_commander = try!(GrSimCommanderBuilder::new().grsim_addr(Ipv4Addr::new(127, 0, 0, 1)).spawn(rx));
    let child_ai = try!(ChildAiBuilder::new(Command::new("./demo-ai")).is_yellow(true).spawn(game_state.clone(), tx));

    // FIXME: ignoring the any reason why a join may have failed
    let _ = child_ai.join();
    let _ = grsim_commander.join();
    let _ = grsim_updater.join();

    Ok(())
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
