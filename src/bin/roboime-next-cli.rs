extern crate roboime_next;
extern crate clap;
extern crate clock_ticks;
extern crate env_logger;
#[macro_use] extern crate log;

use std::error::Error;

fn run() -> Result<(), Box<Error>> {
    let matches = {
        use clap::{Arg, App};
        use clap::AppSettings::*;

        App::new("robime-next-cli")
            .setting(ColoredHelp)
            .setting(DeriveDisplayOrder)
            .setting(TrailingVarArg)
            .about("Connect an AI to a Robocup SSL game through an stdio interface (like codingame).")
            .arg(Arg::with_name("blue")
                 .short("b")
                 .long("blue")
                 .help("Play as the blue team (default)"))
            .arg(Arg::with_name("yellow")
                 .conflicts_with("blue")
                 .short("y")
                 .long("yellow")
                 .help("Play as the yellow team"))
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
            .get_matches()
    };


    {
        use std::env;
        use std::env::VarError;
        use env_logger::LogBuilder;
        use log::LogLevelFilter;

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
        let mut invalid_filter = None;
        match env::var("RUST_LOG") {
            Ok(ref filter) => { builder.parse(filter); }
            Err(VarError::NotUnicode(filter)) => { invalid_filter = Some(filter); }
            Err(VarError::NotPresent) => {}
        }
        try!(builder.init());
        if let Some(filter) = invalid_filter {
            warn!("invalid unicode {:?}, ignoring it", filter);
        }
    }

    {
        use std::process::Command;
        use std::thread;
        use roboime_next::prelude::*;
        use roboime_next::{ai, grsim};
        //use roboime_next::sim;

        let mut ai_cfg = ai::Builder::new(Command::from_args(&matches.values_of_os("AI").unwrap().collect::<Vec<_>>()));
        if matches.is_present("blue")   { ai_cfg.color(Blue); }
        if matches.is_present("yellow") { ai_cfg.color(Yellow); }

        let mut grsim_cfg = grsim::Builder::new();
        if let Some(v) = matches.value_of("grsim_addr")  { try!(grsim_cfg.grsim_addr(v)); }
        if let Some(v) = matches.value_of("vision_addr") { try!(grsim_cfg.vision_addr(v)); }
        if let Some(v) = matches.value_of("vision_port") { grsim_cfg.vision_port(try!(v.parse())); }
        if let Some(v) = matches.value_of("grsim_ip")    { grsim_cfg.grsim_ip(try!(v.parse())); }

        //let mut sim_cfg = sim::Builder::new();
        //sim_cfg.initial_formation(true);

        let mut step_clock = clock_ticks::precise_time_ns();
        let mut step_counter = 0;
        let mut ai = try!(ai_cfg.build());
        let mut grsim = try!(grsim_cfg.build());
        //let mut sim = sim_cfg.build();

        debug!("Wait for grSim geometry...");
        try!(grsim.wait_for_geom());

        let debug = ai.debug.take().unwrap();
        thread::spawn(move || {
            for line in debug {
                println!("AI> {}", line.unwrap());
            }
        });

        debug!("Wait for AI to start...");
        let mut ai = try!(ai.init(&grsim));
        //let mut ai = try!(ai.init(&sim));

        debug!("AI started, going on...");

        loop {
            let now = clock_ticks::precise_time_ns();
            let delta = now - step_clock;
            if delta >= 1_000_000_000 {
                // expect ~60 steps per second
                info!("{:2} steps in {:.03} seconds", step_counter, delta as f32 * 1.0e-9);
                step_counter = 0;
                step_clock = now;
            }
            step_counter += 1;

            try!(grsim.recv_state());
            let ai2 = try!(ai.push(&grsim));
            let (ai3, cmd) = try!(ai2.pull());
            try!(grsim.send_command(cmd));
            //let ai2 = try!(ai.push(&sim));
            //let (ai3, cmd) = try!(ai2.pull());
            //sim.step(cmd, 0.016_666_667);
            ai = ai3;
        }
    }

    Ok(())
}

fn main() {
    use std::process::exit;

    if let Err(err) = run() {
        let mut err: &Error = &*err;

        println!("Error: {}.", err.description());
        while let Some(cause) = err.cause() {
            println!("- caused by: {}", cause.description());
            err = cause;
        }

        exit(1);
    }
}
