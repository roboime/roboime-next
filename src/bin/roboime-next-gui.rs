extern crate roboime_next;
#[macro_use] extern crate glium;
extern crate clap;
extern crate clock_ticks;
//extern crate image;
extern crate env_logger;
#[macro_use] extern crate log;

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
    use glium::{glutin, DisplayBuild, Surface};
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
             .help("Play as the blue team."))
        .arg(Arg::with_name("yellow")
             .conflicts_with("blue")
             .short("y")
             .long("yellow")
             .help("Play as the yellow team. (default)"))
        .arg(Arg::with_name("right")
             .short("r")
             .long("right")
             .help("Play on the right side. (default for blue)"))
        .arg(Arg::with_name("left")
             .conflicts_with("right")
             .short("l")
             .long("left")
             .help("Play on the left side. (default for yellow)"))
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
        .arg(Arg::with_name("AI")
             .required(true)
             .multiple(true)
             .value_name("COMMAND")
             .help("Command to start the AI, accepts arguments."))
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
        .with_multisampling(2)
        .with_vsync()
        .build_glium());

    let perspective = {
        let (width, height) = display.get_window().unwrap().get_inner_size_points().unwrap();
        perspective_matrix(width as f32, height as f32)
    };

    let color = {
        let mut color = Default::default();
        if matches.is_present("blue")   { color = Blue; };
        if matches.is_present("yellow") { color = Yellow; };
        color
    };

    let team_side = {
        let mut team_side = Default::default();
        if matches.is_present("right") { team_side = TeamSide::new(color, Right); };
        if matches.is_present("left")  { team_side = TeamSide::new(color, Left); };
        team_side
    };

    let mut is_paused = {
        let mut pause = false;
        if matches.is_present("pause") { pause = true; };
        pause
    };

    let mut ai_cfg = {
        let ai_cmd: Vec<&str> = matches.values_of("AI").unwrap().collect();
        let (ai_program, ai_args) = (ai_cmd[0], &ai_cmd[1..]);
        let mut ai_command = Command::new(ai_program);
        ai_command.args(ai_args);
        let mut ai_cfg = ai::Builder::new(ai_command);
        ai_cfg.color(color);
        ai_cfg
    };

    let mut sim_state = sim::Builder::new()
        .initial_formation(true)
        .team_side(team_side)
        .build();
    let mut draw_game = try!(Game::new(&display)); draw_game.team_side(&display, team_side);
    let mut ai = try!(ai_cfg.build());

    let mut accumulator = 0;
    let mut previous_clock = clock_ticks::precise_time_ns();
    let mut step_clock = previous_clock;
    let mut step_counter = 0;

    let mut key_meta = false;
    let mut key_ctrl = false;

    // debugger thread
    let debug = ai.debug.take().unwrap();
    thread::spawn(move || {
        for line in debug {
            println!("AI> {}", line.unwrap());
        }
    });

    // init the AI
    debug!("Wait for AI to start...");
    let mut ai_state = try!(ai.init(&sim_state));

    //drop(try!(rx.recv()));
    debug!("AI started, going on...");

    // the main loop
    'main: loop {
        #[derive(Copy, Clone, PartialEq, Eq)] enum Shot { No, Fwd, Bkw, }
        let mut single_shot = Shot::No;

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
        let mut target = display.draw();
        try!(draw_game.draw_to(&mut target, &sim_state, perspective, view));
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
                let cmd = try!(ai_state.update(&sim_state));
                sim_state.step(cmd, FIXED_TIME_STEP as f32 * 1.0e-9);
                step_counter += 1;
            }
        }
        previous_clock = now;

        if single_shot != Shot::No {
            // XXX: is there a simple way to no duplicade this?
            // game step
            let cmd = try!(ai_state.update(&sim_state));
            let mut timestep = FIXED_TIME_STEP as f32 * 1.0e-9;
            if single_shot == Shot::Bkw { timestep = -timestep; }
            sim_state.step(cmd, timestep);
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
