extern crate roboime_next;
#[macro_use] extern crate glium;
extern crate clap;
extern crate clock_ticks;
extern crate env_logger;
#[macro_use] extern crate log;
extern crate rusttype;
extern crate unicode_normalization;

use std::error::Error;
use self::GameState::*;
use roboime_next::{sim, grsim, real, ai};

mod draw;

enum GameState<'a> {
    Sim(sim::State),
    GrSim(grsim::State),
    Real(real::State<'a>),
}

macro_rules! with_state {
    ($game_state:ident, |$state:ident| $code:expr) => (
        match $game_state {
            Sim(ref $state) => $code,
            GrSim(ref $state) => $code,
            Real(ref $state) => $code,
        }
    );
    ($game_state:ident, |$state:ident| $code:block) => (
        match $game_state {
            Sim(ref $state) => $code,
            GrSim(ref $state) => $code,
            Real(ref $state) => $code,
        }
    );
}

fn main_loop() -> Result<(), Box<Error>> {
    use std::thread;
    use std::time::Duration;
    use std::process::Command;
    use roboime_next::prelude::*;
    use ::draw::*;

    let matches = {
        use clap::{Arg, App};
        use clap::AppSettings::*;

        App::new("robime-next-gui")
            .setting(ColoredHelp)
            .setting(DeriveDisplayOrder)
            .setting(TrailingVarArg)
            .about("Connect an AI to a Robocup SSL game through an stdio interface (like codingame).")
            .arg(Arg::with_name("blue")
                 .short("b")
                 .long("blue")
                 .value_name("BLUE COMMAND")
                 .help("Command to start the blue AI, accepts arguments."))
            .arg(Arg::with_name("yellow")
                 .short("y")
                 .long("yellow")
                 .value_name("YELLOW COMMAND")
                 .help("Command to start the yellow AI, accepts arguments."))
            .arg(Arg::with_name("kickoff-yellow")
                 .short("k")
                 .long("kickoff-yellow")
                 .help("Initial kickoff for yellow (default is blue)."))
            .arg(Arg::with_name("left")
                 .short("l")
                 .long("left")
                 .help("Blue attacks the left side. (default is right)"))
            .arg(Arg::with_name("real")
                 .short("r")
                 .long("real")
                 .help("Use real tx/rx instead of the built-in simulator."))
            .arg(Arg::with_name("grsim")
                 .conflicts_with("real")
                 .short("g")
                 .long("grsim")
                 .help("Use grSim instead of the built-in simulator."))
            .arg(Arg::with_name("grsim_addr")
                 .requires("grsim")
                 .long("grsim-addr")
                 .takes_value(true)
                 .value_name("ADDRESS:PORT")
                 .help("Set the address and port where grSim is listening for commands."))
            .arg(Arg::with_name("grsim_ip")
                 .requires("grsim")
                 .conflicts_with("gsim_addr")
                 .long("grsim-ip")
                 .takes_value(true)
                 .value_name("ADDRESS")
                 .help("Set the address where grSim is listening for commands."))
            .arg(Arg::with_name("vision_addr")
                 .requires("grsim")
                 .conflicts_with("vision_port")
                 .long("vision-addr")
                 .takes_value(true)
                 .value_name("ADDRESS:PORT")
                 .help("Set the multicast address and port to receive the vision packets."))
            .arg(Arg::with_name("vision_port")
                 .requires("grsim")
                 .conflicts_with("vision_addr")
                 .long("vision-port")
                 .takes_value(true)
                 .value_name("PORT")
                 .help("Set the port to receive the vision packets. (Use default multicast address)"))
            .arg(Arg::with_name("pause")
                 .conflicts_with_all(&["real", "grsim"])
                 .short("p")
                 .long("pause")
                 .help("Start the simulator paused."))
            .arg(Arg::with_name("fast")
                 .conflicts_with_all(&["real", "grsim"])
                 .short("f")
                 .long("fast")
                 .multiple(true)
                 .help("Each f doubles the speed."))
            .arg(Arg::with_name("slow")
                 .conflicts_with_all(&["fast", "real", "grsim"])
                 .short("s")
                 .long("slow")
                 .multiple(true)
                 .help("Each s halves the speed."))
            .arg(Arg::with_name("v")
                 .short("v")
                 .multiple(true)
                 .help("Sets the level of verbosity."))
            .arg(Arg::with_name("q")
                 .conflicts_with("v")
                 .long("quiet")
                 .short("q")
                 .help("Don't log warnings or errors."))
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

    // building the display, ie. the main object
    let display = {
        use glium::{glutin, DisplayBuild};
        try!(glutin::WindowBuilder::new()
             .with_title(format!("RoboIME Next"))
             // 1 extra pixel to align the middle lines to the screen
             .with_dimensions(1041, 741)
             .with_depth_buffer(24)
             .with_gl(glutin::GlRequest::Latest)
             .with_multisampling(4)
             .with_vsync()
             .build_glium())
    };

    let team_side = {
        use roboime_next::base::TeamSide::*;
        if matches.is_present("left") {
            YellowIsLeft
        } else {
            BlueIsLeft
        }
    };

    let kickoff_color = if matches.is_present("kickoff-yellow") {
        Yellow
    } else {
        Blue
    };

    let mut is_paused = {
        let mut pause = false;
        if matches.is_present("pause") { pause = true; };
        pause
    };

    let multiplier = if matches.is_present("fast") {
        2.0f32.powi(matches.occurrences_of("fast") as i32)
    } else if matches.is_present("slow") {
        0.5f32.powi(matches.occurrences_of("slow") as i32)
    } else {
        1.0
    };

    // AI Builders

    let ai_blue_cfg = if matches.is_present("blue") {
        let ai_cmd: Vec<&str> = matches.values_of("blue").unwrap().collect::<Vec<&str>>()[0].split(" ").collect();
        let mut ai_cfg = ai::Builder::new(move || -> Command {
            let (ai_program, ai_args) = (ai_cmd[0], &ai_cmd[1..]);
            let mut ai_command = Command::new(ai_program);
            ai_command.args(ai_args);
            ai_command
        });
        ai_cfg.debugger(move |line| {
            println!("blue> {}", line);
        });
        ai_cfg.color(Color::Blue);
        Some(ai_cfg)
    } else {
        None
    };

    let ai_yellow_cfg = if matches.is_present("yellow") {
        let ai_cmd: Vec<&str> = matches.values_of("yellow").unwrap().collect::<Vec<&str>>()[0].split(" ").collect();
        let mut ai_cfg = ai::Builder::new(move || -> Command {
            let (ai_program, ai_args) = (ai_cmd[0], &ai_cmd[1..]);
            let mut ai_command = Command::new(ai_program);
            ai_command.args(ai_args);
            ai_command
        });
        ai_cfg.debugger(move |line| {
            println!("yellow> {}", line);
        });
        ai_cfg.color(Color::Yellow);
        Some(ai_cfg)
    } else {
        None
    };

    // Game/AI/Draw states

    let mut real_builder = if matches.is_present("real") {
        Some(real::Builder::new())
    } else {
        None
    };

    let mut game_state = if let Some(ref mut builder) = real_builder {
        Real(try!(builder.build()))
    } else if matches.is_present("grsim") {
        let mut grsim_cfg = grsim::Builder::new();
        if let Some(v) = matches.value_of("grsim_addr")  { try!(grsim_cfg.grsim_addr(v)); }
        if let Some(v) = matches.value_of("vision_addr") { try!(grsim_cfg.vision_addr(v)); }
        if let Some(v) = matches.value_of("vision_port") { grsim_cfg.vision_port(try!(v.parse())); }
        if let Some(v) = matches.value_of("grsim_ip")    { grsim_cfg.grsim_ip(try!(v.parse())); }
        GrSim(try!(grsim_cfg.build()))
    } else {
        Sim(sim::Builder::new()
            .initial_formation(true)
            .team_side(team_side)
            .kickoff_color(kickoff_color)
            .build())
    };
    let mut draw_game = try!(Game::new(&display)); draw_game.team_side(&display, team_side);

    let mut ai_blue = match ai_blue_cfg {
        Some(ai_cfg) => Some(try!(ai_cfg.build())),
        None => None,
    };

    let mut ai_yellow = match ai_yellow_cfg {
        Some(ai_cfg) => Some(try!(ai_cfg.build())),
        None => None,
    };

    let mut accumulator = 0.0;
    let mut previous_clock = clock_ticks::precise_time_ns();
    let mut step_clock = previous_clock;
    let mut step_counter = 0;

    let mut key_meta = false;
    let mut key_ctrl = false;

    match game_state {
        Sim(_) => {}
        GrSim(ref mut grsim_state) => {
            debug!("Wait for geometry...");
            try!(grsim_state.wait_for_geom());
        }
        Real(ref mut real_state) => {
            debug!("Wait for geometry...");
            try!(real_state.wait_for_geom());
        }
    }

    // Initialize the AIs

    debug!("Wait for AI to start...");
    let mut ai_blue_state = match ai_blue {
        Some(ref mut ai) => Some(with_state!(game_state, |state| try!(ai.init(state)))),
        None => None,
    };
    let mut ai_yellow_state = match ai_yellow {
        Some(ref mut ai) => Some(with_state!(game_state, |state| try!(ai.init(state)))),
        None => None,
    };
    debug!("AI started, going on...");

    // the main loop
    'main: loop {
        #[derive(Copy, Clone, PartialEq, Eq)] enum Shot { No, Fwd, Bkw, }
        let mut single_shot = Shot::No;

        with_state!(game_state, |state| {
            draw_game.update(&display, state);
        });

        // render the game state
        let view = view_matrix(&[0.0, 0.0, 10.0], &[0.0, 0.0, -1.0], &[0.0, 1.0, 0.0]);
        //let view = view_matrix(&[-4.0, -4.0, 4.0], &[1.0, 1.0, -1.0], &[0.0, 0.0, 1.0]);
        //let view = view_matrix(&[4.0, -4.0, 4.0], &[-1.0, 1.0, -1.0], &[0.0, 0.0, 1.0]);
        //let view = view_matrix(&[-0.2, -0.2, 0.6], &[0.2, 0.2, -0.6], &[0.0, 0.0, 1.0]);
        //let view = view_matrix(&[1.0, 2.0, 3.0], &[1.0, 1.0, -3.0], &[0.0, 1.0, 0.0]);
        //let view = view_matrix(&[-4.0, 0.7, 1.6], &[-0.2, -0.2, -0.6], &[0.0, 0.0, 1.0]);
        //let view = view_matrix(&[-5.0, -0.7, 1.6], &[0.2, 0.2, -0.6], &[0.0, 0.0, 1.0]);
        //let view = view_matrix(&[4.0, 0.7, 1.6], &[0.2, -0.2, -0.6], &[0.0, 0.0, 1.0]);
        //let view = view_matrix(&[5.0, -0.7, 1.6], &[-0.2, 0.2, -0.6], &[0.0, 0.0, 1.0]);
        //let view = view_matrix(&[0.0, 0.0, 0.6], &[0.0, 0.0, -1.0], &[0.0, 1.0, 0.0]);
        let mut target = display.draw();
        let view_port = display.get_window().unwrap().get_inner_size_points().unwrap();
        with_state!(game_state, |state| {
            try!(draw_game.draw_to(&mut target, state, view_port, view));
        });
        try!(target.finish());

        // polling and handling the events received by the window
        for event in display.poll_events() {
            use glium::glutin::Event::*;
            use glium::glutin::ElementState::*;
            use glium::glutin::VirtualKeyCode::*;

            match event {

                // close on Ctrl+Q, Cmd+Q, Win+Q, Esc, window X button
                KeyboardInput(Pressed, _, Some(Q)) if key_meta || key_ctrl => break 'main,
                KeyboardInput(Pressed, _, Some(Escape)) | Closed => break 'main,

                // modifier keys
                KeyboardInput(Pressed,  _, Some(LWin)) |
                KeyboardInput(Pressed,  _, Some(RWin)) => { key_meta = true; }
                KeyboardInput(Released, _, Some(LWin)) |
                KeyboardInput(Released, _, Some(RWin)) => { key_meta = false; }
                KeyboardInput(Pressed,  _, Some(LControl)) |
                KeyboardInput(Pressed,  _, Some(RControl)) => { key_ctrl = true; }
                KeyboardInput(Released, _, Some(LControl)) |
                KeyboardInput(Released, _, Some(RControl)) => { key_ctrl = false; }

                // some actions
                KeyboardInput(Pressed, _, Some(P)) => {
                    is_paused = !is_paused;
                }
                KeyboardInput(Pressed, _, Some(RBracket)) => {
                    single_shot = Shot::Fwd;
                }
                KeyboardInput(Pressed, _, Some(LBracket)) => {
                    single_shot = Shot::Bkw;
                }
                KeyboardInput(Pressed, _, Some(R)) => {
                    if let Sim(ref mut sim_state) = game_state {
                        // reset ball position and speed
                        sim_state.ball.pos = Vec2d(0.0, 0.0);
                        sim_state.ball.vel = Vec2d(0.0, 0.0);
                    }
                }
                KeyboardInput(..) => {
                    debug!("{:?}", event);
                }
                _ => ()
            }
        }

        //const FIXED_TIME_STEP: u64 = 16_666_667;
        const FIXED_TIME_STEP: f32 = 1_000_000_000.0 / 60.0;

        let now = clock_ticks::precise_time_ns();
        if !is_paused {
            match game_state {
                Sim(ref mut sim_state) => {
                    accumulator += (now - previous_clock) as f32 * multiplier;

                    while accumulator >= FIXED_TIME_STEP {
                        accumulator -= FIXED_TIME_STEP;
                        let now = clock_ticks::precise_time_ns();
                        let delta = now - step_clock;
                        if delta >= 1_000_000_000 {
                            // expect ~60 steps per second
                            info!("{:2} steps in {:.03} seconds", step_counter, delta as f32 * 1.0e-9);
                            step_counter = 0;
                            step_clock = now;
                        }

                        // game step
                        let mut cmds = vec![];

                        if let Some(ref mut ai_state) = ai_blue_state {
                            if let Some(cmd) = try!(ai_state.update(sim_state)) {
                                cmds.push(cmd);
                            } else {
                                warn!("blue AI did not respond");
                            }
                        }

                        if let Some(ref mut ai_state) = ai_yellow_state {
                            if let Some(cmd) = try!(ai_state.update(sim_state)) {
                                cmds.push(cmd);
                            } else {
                                warn!("yellow AI did not respond");
                            }
                        }

                        sim_state.step(&cmds, FIXED_TIME_STEP * 1.0e-9);
                        step_counter += 1;
                    }
                }
                GrSim(ref mut grsim_state) => {
                    // FIXME: think of a way to decouple this blocking call from the drawing loop
                    try!(grsim_state.recv_state());
                    let blue_cmd = match ai_blue_state {
                        Some(ref mut ai_state) => {
                            let cmd = try!(ai_state.update(grsim_state));
                            if cmd.is_none() {
                                warn!("blue AI did not respond");
                            }
                            cmd
                        }
                        None => None,
                    };
                    let yellow_cmd = match ai_yellow_state {
                        Some(ref mut ai_state) => {
                            let cmd = try!(ai_state.update(grsim_state));
                            if cmd.is_none() {
                                warn!("yellow AI did not respond");
                            }
                            cmd
                        }
                        None => None,
                    };
                    if let Some(cmd) = blue_cmd {
                        try!(grsim_state.send_command(cmd));
                    }
                    if let Some(cmd) = yellow_cmd {
                        try!(grsim_state.send_command(cmd));
                    }
                }
                Real(ref mut real_state) => {
                    // FIXME: think of a way to decouple this blocking call from the drawing loop
                    try!(real_state.recv_state());
                    let blue_cmd = match ai_blue_state {
                        Some(ref mut ai_state) => {
                            let cmd = try!(ai_state.update(real_state));
                            if cmd.is_none() {
                                warn!("blue AI did not respond");
                            }
                            cmd
                        }
                        None => None,
                    };
                    let yellow_cmd = match ai_yellow_state {
                        Some(ref mut ai_state) => {
                            let cmd = try!(ai_state.update(real_state));
                            if cmd.is_none() {
                                warn!("yellow AI did not respond");
                            }
                            cmd
                        }
                        None => None,
                    };
                    if let Some(cmd) = blue_cmd {
                        try!(real_state.send_command(cmd));
                    }
                    if let Some(cmd) = yellow_cmd {
                        try!(real_state.send_command(cmd));
                    }
                }
            }
        }
        previous_clock = now;

        if single_shot != Shot::No {
            if let Sim(ref mut sim_state) = game_state {
                // XXX: is there a simple way to no duplicade this?
                // game step
                let mut cmds = vec![];

                if let Some(ref mut ai_state) = ai_blue_state {
                    if let Some(cmd) = try!(ai_state.update(sim_state)) {
                        cmds.push(cmd);
                    } else {
                        warn!("blue AI did not respond");
                    }
                }

                if let Some(ref mut ai_state) = ai_yellow_state {
                    if let Some(cmd) = try!(ai_state.update(sim_state)) {
                        cmds.push(cmd);
                    } else {
                        warn!("yellow AI did not respond");
                    }
                }

                let mut timestep = FIXED_TIME_STEP * 1.0e-9;
                if single_shot == Shot::Bkw { timestep = -timestep; }
                sim_state.step(&cmds, timestep);
            }
        }

        if let Sim(_) = game_state {
            let real_accumulated = accumulator / multiplier;
            if FIXED_TIME_STEP > real_accumulated {
                thread::sleep(Duration::from_millis(((FIXED_TIME_STEP - real_accumulated) / 1_000_000.0) as u64));
            }
        }
    }

    info!("Bye!");
    Ok(())
}

fn main() {
    use std::process::exit;

    if let Err(err) = main_loop() {
        println!("Error: {}.", err.description());

        let mut err: &Error = &*err;
        while let Some(cause) = err.cause() {
            println!("- caused by: {}", cause.description());
            err = cause;
        }

        exit(1);
    }
}
