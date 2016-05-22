extern crate roboime_next;
#[macro_use] extern crate glium;
extern crate clock_ticks;
//extern crate image;
extern crate env_logger;
extern crate log;

use std::error::Error;
use std::process::exit;
//use std::io::Cursor;
use glium::Surface;
use glium::glutin;
//use image::image::GenericImage;
//use roboime_next::prelude::*;
use roboime_next::game;
use ::utils::*;
use ::models::TeamSide;

pub mod models;
pub mod colors;
mod utils;
mod draw;

fn main_loop() -> Result<(), Box<Error>> {
    use std::env;
    use std::thread;
    use std::time::Duration;
    use glium::DisplayBuild;
    use glium::glutin::GlRequest;
    use log::LogLevelFilter;
    use env_logger::LogBuilder;

    {
        let mut builder = LogBuilder::new();
        builder.format(|record| {
            format!("{} [{}] {}", record.level(), record.location().module_path(), record.args())
        });
        builder.filter(Some("roboime"), LogLevelFilter::Info);
        if env::var("RUST_LOG").is_ok() {
           builder.parse(&env::var("RUST_LOG").unwrap());
        }
        try!(builder.init());
    }

    // building the display, ie. the main object
    let display = {
        // 1 extra pixel to align the middle lines to the screen
        // TODO: refactor: deduplicate options
        if let Ok(display) = glutin::WindowBuilder::new()
            .with_title(format!("RoboIME Next"))
            .with_dimensions(1041, 741)
            .with_depth_buffer(24)
            .with_gl(GlRequest::Latest)
            .with_multisampling(4)
            with_srgb(Some(true))
            .with_vsync()
            .build_glium() {
            display
        } else {
            try!(glutin::WindowBuilder::new()
                .with_title(format!("RoboIME Next"))
                .with_dimensions(1041, 741)
                .with_depth_buffer(24)
                .with_gl(GlRequest::Latest)
                .with_multisampling(4)
                .with_vsync()
                .build_glium())
        }
    };

    let team_side = TeamSide::BlueIsLeft;
    let mut draw_game = try!(draw::Game::new(&display));
    draw_game.team_side(&display, team_side);

    let mut accumulator = 0;
    let mut previous_clock = clock_ticks::precise_time_ns();
    let game_state = game::SharedState::new();

    // add some robots
    {
        let mut game_state = try!(game_state.write());
        add_initial_robots(game_state.get_robots_blue_mut(), 6, false);
        add_initial_robots(game_state.get_robots_yellow_mut(), 6, true);
    }

    // the main loop
    'main: loop {

        let mut target = display.draw();

        let view = view_matrix(&[0.0, 0.0, 10.0], &[0.0, 0.0, -1.0], &[0.0, 1.0, 0.0]);
        //let view = view_matrix(&[-4.0, -4.0, 4.0], &[1.0, 1.0, -1.0], &[0.0, 0.0, 1.0]);
        //let view = view_matrix(&[4.0, -4.0, 4.0], &[-1.0, 1.0, -1.0], &[0.0, 0.0, 1.0]);
        //let view = view_matrix(&[-0.2, -0.2, 0.6], &[0.2, 0.2, -0.6], &[0.0, 0.0, 1.0]);
        //let view = view_matrix(&[1.0, 2.0, 3.0], &[1.0, 1.0, -3.0], &[0.0, 1.0, 0.0]);
        //let view = view_matrix(&[-4.0, 0.7, 1.6], &[-0.2, -0.2, -0.6], &[0.0, 0.0, 1.0]);
        //let view = view_matrix(&[-5.0, -0.7, 1.6], &[0.2, 0.2, -0.6], &[0.0, 0.0, 1.0]);
        //let view = view_matrix(&[4.0, 0.7, 1.6], &[0.2, -0.2, -0.6], &[0.0, 0.0, 1.0]);
        //let view = view_matrix(&[5.0, -0.7, 1.6], &[-0.2, 0.2, -0.6], &[0.0, 0.0, 1.0]);

        {
            let state = try!(game_state.read());
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

            // if you have a game, update the state here
        }

        thread::sleep(Duration::from_millis(((FIXED_TIME_STAMP - accumulator) / 1000000) as u64));
    }

    Ok(())
}

fn main() {
    if let Err(err) = main_loop() {
        println!("Error: {}.", err.description());
        while let Some(err) = err.cause() {
            println!("- caused by {}", err.description());
            // XXX: hack to avoid infinite loop, also because &std::error:Error doesn't impl PartialEq
            if err.cause().is_some() && err.description() == err.cause().unwrap().description() {
                break;
            }
        }
        exit(1);
    }
}
