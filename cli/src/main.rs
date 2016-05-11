extern crate roboime_next;
extern crate clap;
extern crate env_logger;
extern crate log;

use std::env;
use std::sync::mpsc::channel;
use std::process::{exit, Command};
use std::error::Error;
use log::LogLevelFilter;
use env_logger::LogBuilder;
use roboime_next::prelude::*;
use roboime_next::{game, ai, grsim, Result};
use clap::{Arg, App, AppSettings};

fn main_loop() -> Result<()> {
    let matches = App::new("robime-next-cli")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::TrailingVarArg)
        .about("Connect an AI to a Robocup SSL game through an stdio interface (like codingame).")
        .arg(Arg::with_name("blue")
             .short("b")
             .long("blue")
             .help("Play as the blue team"))
        .arg(Arg::with_name("yellow")
             .conflicts_with("blue")
             .short("y")
             .long("yellow")
             .help("Play as the yellow team (default)"))
        .arg(Arg::with_name("v")
             .short("v")
             .multiple(true)
             .help("Sets the level of verbosity"))
        .arg(Arg::with_name("q")
             .conflicts_with("v")
             .long("quiet")
             .short("q")
             .help("Don't log warnings or errors."))
        .arg(Arg::with_name("grsim_addr")
             .long("grsim")
             .takes_value(true)
             .value_name("ADDRESS:PORT")
             .help("Set the address and port where grSim is listening for commands."))
        .arg(Arg::with_name("grsim_ip")
             .conflicts_with("gsim_addr")
             .long("grsim-addr")
             .takes_value(true)
             .value_name("ADDRESS")
             .help("Set the address where grSim is listening for commands."))
        .arg(Arg::with_name("vision_addr")
             .conflicts_with("vision_port")
             .long("vision")
             .takes_value(true)
             .value_name("ADDRESS:PORT")
             .help("Set the multicast address and port to receive the vision packets."))
        .arg(Arg::with_name("vision_port")
             .conflicts_with("vision_addr")
             .long("vision-port")
             .takes_value(true)
             .value_name("PORT")
             .help("Set the port to receive the vision packets. (Use default multicast address)"))
        .arg(Arg::with_name("AI")
             .required(true)
             .multiple(true)
             .value_name("COMMAND")
             .help("Command to start the AI, accepts arguments."))
        .get_matches();


    let mut builder = LogBuilder::new();
    builder.format(|record| {
        format!("{} [{}] {}", record.level(), record.location().module_path(), record.args())
    });
    let verbosity = matches.occurrences_of("v");
    builder.filter(Some("roboime"), match verbosity {
        0 => LogLevelFilter::Warn,
        1 => LogLevelFilter::Info,
        2 => LogLevelFilter::Debug,
        _ => LogLevelFilter::Trace,
    });
    if matches.is_present("q") {
        builder.filter(Some("roboime"), LogLevelFilter::Off);
    }
    if env::var("RUST_LOG").is_ok() {
       builder.parse(&env::var("RUST_LOG").unwrap());
    }
    try!(builder.init());

    let ai_cmd: Vec<&str> = matches.values_of("AI").unwrap().collect();
    let (ai_program, ai_args) = (ai_cmd[0], &ai_cmd[1..]);
    let mut ai_command = Command::new(ai_program);
    ai_command.args(ai_args);
    let mut ai_cfg = ai::Interface::new(ai_command);
    if matches.is_present("blue") {
        ai_cfg.is_yellow(false);
    }
    if matches.is_present("yellow") {
        ai_cfg.is_yellow(true);
    }

    let mut grsim_cfg = grsim::Interface::new();
    try!(grsim_cfg.grsim_addr(matches.value_of("grsim_addr").unwrap_or("127.0.0.1:20011")));
    try!(grsim_cfg.vision_addr(matches.value_of("vision_addr").unwrap_or("224.5.23.2:10002")));
    if matches.is_present("vision_port") {
        grsim_cfg.vision_port(try!(matches.value_of("vision_port").unwrap().parse()));
    }
    if matches.is_present("grsim_ip") {
        grsim_cfg.grsim_ip(try!(matches.value_of("grsim_ip").unwrap().parse()));
    }

    let state = game::SharedState::new();
    let (tx, rx) = channel();

    let ai = try!(ai_cfg.spawn(state.clone(), tx));
    let grsim = try!(grsim_cfg.spawn(state.clone(), rx));

    (ai, grsim).join()
}

fn dump_error(res: Result<()>) -> bool {
    if let Err(err) = res {
        println!("Error: {}.", err.description());
        while let Some(err) = err.cause() {
            println!("- caused by {}", err.description());
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
