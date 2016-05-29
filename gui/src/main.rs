extern crate roboime_next;
#[macro_use] extern crate glium;
extern crate clap;
extern crate clock_ticks;
//extern crate image;
extern crate env_logger;
#[macro_use] extern crate log;

use std::error::Error;

pub mod models;
pub mod colors;
mod utils;
mod draw;

fn main_loop() -> Result<(), Box<Error>> {
    use std::env;
    use std::thread;
    //use std::io::Cursor;
    use std::time::Duration;
    use std::process::Command;
    use roboime_next::prelude::*;
    use roboime_next::{game, ai};
    use clap::{Arg, App, AppSettings};
    use glium::{glutin, DisplayBuild, Surface};
    use log::LogLevelFilter;
    use env_logger::LogBuilder;
    //use image::image::GenericImage;
    use ::utils::*;

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
             .help("Play on the right side."))
        .arg(Arg::with_name("left")
             .short("l")
             .long("left")
             .help("Play on the left side. (default)"))
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

    let is_yellow = {
        let mut yellow = true;
        if matches.is_present("blue")   { yellow = false; };
        if matches.is_present("yellow") { yellow = true; };
        yellow
    };

    let is_left = {
        let mut left = true;
        if matches.is_present("right") { left = false; };
        if matches.is_present("left")  { left = true; };
        left
    };

    let mut ai_cfg = {
        let ai_cmd: Vec<&str> = matches.values_of("AI").unwrap().collect();
        let (ai_program, ai_args) = (ai_cmd[0], &ai_cmd[1..]);
        let mut ai_command = Command::new(ai_program);
        ai_command.args(ai_args);
        let mut ai_cfg = ai::Interface::new(ai_command);
        ai_cfg.is_yellow(is_yellow);
        ai_cfg
    };

    let team_side = {
        use ::models::TeamSide::*;
        match (is_yellow, is_left) {
            (true,  true)  => YellowIsLeft,
            (true,  false) => BlueIsLeft,
            (false, false) => YellowIsLeft,
            (false, true)  => BlueIsLeft,
        }
    };

    let mut draw_game = try!(draw::Game::new(&display));
    draw_game.team_side(&display, team_side);

    let mut accumulator = 0;
    let mut previous_clock = clock_ticks::precise_time_ns();
    let mut step_clock = previous_clock;
    let mut step_counter = 0;
    let mut game_state = game::State::new();

    let mut ai = try!(ai_cfg.spawn());

    // add some robots
    {
        //use std::f32::consts::FRAC_PI_4;
        use ::models::TeamSide::*;

        //let mut state = try!(game_state.write());
        let mut state = &mut game_state;
        let blue_is_right = team_side == YellowIsLeft;
        add_initial_robots(state.get_robots_blue_mut(), 6, blue_is_right);
        add_initial_robots(state.get_robots_yellow_mut(), 6, !blue_is_right);
        //let ball = state.get_ball_mut();
        //ball.set_x(3.5);
        //ball.set_y(3.0);
        //let mut robot = state.get_robots_yellow_mut().get_mut(&0).unwrap();
        //robot.set_w(FRAC_PI_4);
        //robot.set_x(-2.5);
        //robot.set_y(-1.5);
    }

    // init the AI
    debug!("Wait for AI to start...");
    try!(ai.push_init(&game_state));

    //drop(try!(rx.recv()));
    debug!("AI started, going on...");

    let debug = ai.debug.take().unwrap();
    let debugger = thread::spawn(move || {
        for line in debug {
            println!("AI> {}", line.unwrap());
        }
    });

    // the main loop
    'main: loop {

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
        {
            //let state = try!(game_state.read());
            let state = &game_state;
            try!(draw_game.draw_to(&mut target, &state, view));
        }
        try!(target.finish());

        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,
                _ => ()
            }
        }

        let now = clock_ticks::precise_time_ns();
        accumulator += now - previous_clock;
        previous_clock = now;

        const FIXED_TIME_STAMP: u64 = 16666667;
        while accumulator >= FIXED_TIME_STAMP {
            accumulator -= FIXED_TIME_STAMP;
            let now = clock_ticks::precise_time_ns();
            let delta = now - step_clock;
            if delta >= 1_000_000_000 {
                // expect ~60 steps per second
                info!("{:2} steps in {:.03} seconds", step_counter, delta as f32 * 1.0e-9);
                step_counter = 0;
                step_clock = now;
            }
            step_counter += 1;

            // game step
            //game_state.notify();
            //let cmd = try!(rx.recv());
            //let mut state = try!(game_state.write());
            let mut state = &mut game_state;
            try!(ai.push_state(state));
            let cmd = try!(ai.read_command(state));
            {
                let robots = if is_yellow {
                    state.get_robots_yellow_mut()
                } else {
                    state.get_robots_blue_mut()
                };

                // XXX: overly simplified physics ahead
                for (ref robot_id, ref mut robot_command) in cmd.robots.iter() {
                    let mut robot = robots.get_mut(robot_id).unwrap();
                    let d_time = FIXED_TIME_STAMP as f32 * 1.0e-9;

                    let d_tangent = d_time * robot_command.v_tangent;
                    let d_normal  = d_time * robot_command.v_normal;
                    let d_angular = d_time * robot_command.v_angular;

                    let x = robot.get_x();
                    let y = robot.get_y();
                    let w = robot.get_w();

                    let dx = d_normal * w.sin() + d_tangent * w.cos();
                    let dy = d_normal * w.cos() - d_tangent * w.sin();
                    let dw = d_angular;

                    //println!("dx: {}", dx);

                    robot.set_x(x + dx);
                    robot.set_y(y + dy);
                    robot.set_w(w + dw);

                    // TODO: effect of robot_command.action
                }
            }
            state.inc_counter();

            //debug!("{:#?} <- {:#?}", state, cmd);
            //drop(state);
        }

        thread::sleep(Duration::from_millis(((FIXED_TIME_STAMP - accumulator) / 1_000_000) as u64));
    }

    try!(ai.quit());
    debug!("Waiting for AI to exit");
    // FIXME: error after a quit is a bug, silence it for now
    let _ = ai.join();
    let _ = debugger.join();
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
