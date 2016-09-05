extern crate roboime_next;
#[macro_use] extern crate glium;
extern crate clap;
extern crate clock_ticks;
//extern crate image;
extern crate env_logger;
#[macro_use] extern crate log;
extern crate rusttype;
//extern crate arrayvec;
extern crate unicode_normalization;

use std::error::Error;

mod draw;

fn main_loop() -> Result<(), Box<Error>> {
    use std::env;
    use std::thread;
    //use std::io::Cursor;
    use std::time::Duration;
    use std::process::Command;
    use roboime_next::prelude::*;
    use roboime_next::{sim, ai};
    use clap::{Arg, App, AppSettings};
    use glium::{glutin, DisplayBuild};
    use log::LogLevelFilter;
    use env_logger::LogBuilder;
    //use image::image::GenericImage;
    use ::draw::*;

    let matches = App::new("robime-next-cli")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::TrailingVarArg)
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
        .arg(Arg::with_name("pause")
             .short("p")
             .long("pause")
             .help("Start the simulator paused."))
        .arg(Arg::with_name("v")
             .short("v")
             .multiple(true)
             .help("Sets the level of verbosity."))
        .arg(Arg::with_name("q")
             .conflicts_with("v")
             .long("quiet")
             .short("q")
             .help("Don't log warnings or errors."))
        //.arg(Arg::with_name("d")
        //     .long("debug")
        //     .short("d")
        //     .help("Enable debug logs."))
        .get_matches();

    {
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
        if let Ok(ref log) = env::var("RUST_LOG") {
           builder.parse(log);
        }
        try!(builder.init());
    }

    // building the display, ie. the main object
    let display = try!(glutin::WindowBuilder::new()
        .with_title(format!("RoboIME Next"))
        // 1 extra pixel to align the middle lines to the screen
        .with_dimensions(1041, 741)
        .with_depth_buffer(24)
        .with_gl(glutin::GlRequest::Latest)
        .with_multisampling(4)
        .with_vsync()
        .build_glium());

    let team_side = {
        let team_side = Default::default();
        team_side
    };

    let mut is_paused = {
        let mut pause = false;
        if matches.is_present("pause") { pause = true; };
        pause
    };

    // Configs

    let ai_blue_cfg = if matches.is_present("blue") {
        let ai_cmd: Vec<&str> = matches.values_of("blue").unwrap().collect();
        let (ai_program, ai_args) = (ai_cmd[0], &ai_cmd[1..]);
        let mut ai_command = Command::new(ai_program);
        ai_command.args(ai_args);
        let mut ai_cfg = ai::Builder::new(ai_command);
        ai_cfg.color(Color::Blue);
        Some(ai_cfg)
    } else {
        None
    };

    let ai_yellow_cfg = if matches.is_present("yellow") {
        let ai_cmd: Vec<&str> = matches.values_of("yellow").unwrap().collect();
        let (ai_program, ai_args) = (ai_cmd[0], &ai_cmd[1..]);
        let mut ai_command = Command::new(ai_program);
        ai_command.args(ai_args);
        let mut ai_cfg = ai::Builder::new(ai_command);
        ai_cfg.color(Color::Yellow);
        Some(ai_cfg)
    } else {
        None
    };

    // States

    let mut sim_state = sim::Builder::new()
        .initial_formation(true)
        .team_side(team_side)
        .build();
    let mut draw_game = try!(Game::new(&display)); draw_game.team_side(&display, team_side);

    let mut ai_blue = match ai_blue_cfg {
        Some(mut ai_cfg) => Some(try!(ai_cfg.build())),
        None => None,
    };

    let mut ai_yellow = match ai_yellow_cfg {
        Some(mut ai_cfg) => Some(try!(ai_cfg.build())),
        None => None,
    };

    let mut accumulator = 0;
    let mut previous_clock = clock_ticks::precise_time_ns();
    let mut step_clock = previous_clock;
    let mut step_counter = 0;

    let mut key_meta = false;
    let mut key_ctrl = false;

    // debugger thread
    if let Some(ref mut ai) = ai_blue {
        let debug = ai.debug.take().unwrap();
        thread::spawn(move || {
            for line in debug {
                println!("Blue> {}", line.unwrap());
            }
        });
    }

    if let Some(ref mut ai) = ai_yellow {
        let debug = ai.debug.take().unwrap();
        thread::spawn(move || {
            for line in debug {
                println!("Blue> {}", line.unwrap());
            }
        });
    }

    // init the AI
    debug!("Wait for AI to start...");
    let mut ai_blue_state = match ai_blue {
        Some(ref mut ai) => Some(try!(ai.init(&sim_state))),
        None => None,
    };
    let mut ai_yellow_state = match ai_yellow {
        Some(ref mut ai) => Some(try!(ai.init(&sim_state))),
        None => None,
    };

    //drop(try!(rx.recv()));
    debug!("AI started, going on...");

    // the main loop
    'main: loop {
        #[derive(Copy, Clone, PartialEq, Eq)] enum Shot { No, Fwd, Bkw, }
        let mut single_shot = Shot::No;

        draw_game.update(&display, &sim_state);

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
        try!(draw_game.draw_to(&mut target, &sim_state, view_port, view));
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
                    // reset ball position and speed
                    sim_state.ball.pos = Vec2d(0.0, 0.0);
                    sim_state.ball.vel = Vec2d(0.0, 0.0);
                }
                KeyboardInput(..) => {
                    debug!("{:?}", event);
                }
                _ => ()
            }
        }

        const FIXED_TIME_STEP: u64 = 16_666_667;

        let now = clock_ticks::precise_time_ns();
        if !is_paused {
            accumulator += now - previous_clock;

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
                    cmds.push(try!(ai_state.update(&sim_state)));
                }

                if let Some(ref mut ai_state) = ai_yellow_state {
                    cmds.push(try!(ai_state.update(&sim_state)));
                }

                sim_state.step(&cmds, FIXED_TIME_STEP as f32 * 1.0e-9);
                step_counter += 1;
            }
        }
        previous_clock = now;

        if single_shot != Shot::No {
            // XXX: is there a simple way to no duplicade this?
            // game step
            let mut cmds = vec![];

            if let Some(ref mut ai_state) = ai_blue_state {
                cmds.push(try!(ai_state.update(&sim_state)));
            }

            if let Some(ref mut ai_state) = ai_yellow_state {
                cmds.push(try!(ai_state.update(&sim_state)));
            }

            let mut timestep = FIXED_TIME_STEP as f32 * 1.0e-9;
            if single_shot == Shot::Bkw { timestep = -timestep; }
            sim_state.step(&cmds, timestep);
        }

        thread::sleep(Duration::from_millis(((FIXED_TIME_STEP - accumulator) / 1_000_000) as u64));
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
