extern crate roboime_next;
extern crate clap;

use std::sync::mpsc::channel;
use std::process::{exit, Command};
use std::error::Error;
use roboime_next::{Result, SharedGameState, ChildAi, GrSimInterface, InterfaceHandle};
use clap::{Arg, App, AppSettings};

fn main_loop() -> Result<()> {
    let matches = App::new("robime-next-cli")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::TrailingVarArg)
        .about("Connect an AI to a Robocup SSL game through an stdio interface (like codingame).")
        .arg(Arg::with_name("yellow")
             .conflicts_with("blue")
             .short("y")
             .long("yellow")
             .help("Play as the yellow team (default)"))
        .arg(Arg::with_name("blue")
             .short("b")
             .long("blue")
             .help("Play as the blue team"))
        //.arg(Arg::with_name("v")
        //     .short("v")
        //     .multiple(true)
        //     .help("Sets the level of verbosity"))
        .arg(Arg::with_name("grsim_addr")
             .long("grsim")
             .takes_value(true)
             .value_name("ADDRESS")
             .help("Set the address and port where grSim is listening for commands."))
        .arg(Arg::with_name("vision_addr")
             .conflicts_with("vision_port")
             .long("vision")
             .takes_value(true)
             .value_name("ADDRESS")
             .help("Set the multicast address and port to receive the vision packets."))
        .arg(Arg::with_name("vision_port")
             .long("vision-port")
             .takes_value(true)
             .value_name("PORT")
             .help("Set the port to receive the vision packets. (Use default multicast address)"))
        .arg(Arg::with_name("AI")
             .required(true)
             .multiple(true)
             .value_name("COMMAND")
             .help("How to start the AI"))
        .get_matches();

    // TODO: use this or get rid of it
    //let verbosity = matches.occurrences_of("v");

    let ai_cmd: Vec<&str> = matches.values_of("AI").unwrap().collect();
    let (ai_program, ai_args) = (ai_cmd[0], &ai_cmd[1..]);
    let mut ai_command = Command::new(ai_program);
    ai_command.args(ai_args);
    let mut ai_cfg = ChildAi::new(ai_command);
    ai_cfg.is_yellow(matches.is_present("yellow"));

    let mut grsim_cfg = GrSimInterface::new();
    try!(grsim_cfg.grsim_addr(matches.value_of("grsim_addr").unwrap_or("127.0.0.1:20011")));
    try!(grsim_cfg.vision_addr(matches.value_of("vision_addr").unwrap_or("224.5.23.2:10002")));
    if matches.is_present("vision_port") {
        grsim_cfg.vision_port(try!(matches.value_of("vision_port").unwrap().parse()));
    }

    let game_state = SharedGameState::new();
    let (tx, rx) = channel();

    let ai = try!(ai_cfg.spawn(game_state.clone(), tx));
    let grsim = try!(grsim_cfg.spawn(game_state.clone(), rx));

    (ai, grsim).join()
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
