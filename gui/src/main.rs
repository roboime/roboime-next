extern crate roboime_next;
#[macro_use] extern crate glium;
extern crate clock_ticks;
//extern crate image;

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

fn main() {
    use std::thread;
    use std::time::Duration;
    use glium::DisplayBuild;
    use glium::glutin::GlRequest;

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .with_title(format!("RoboIME Next"))
        // 1 extra pixel to align the middle lines to the screen
        .with_dimensions(1041, 741)
        .with_depth_buffer(32)
        .with_gl(GlRequest::Latest)
        .with_srgb(Some(true))
        .with_multisampling(4)
        //.with_multisampling(8)
        .with_vsync()
        .build_glium()
        .unwrap();

    let team_side = TeamSide::BlueIsLeft;
    let mut draw_game = draw::Game::new(&display).unwrap();
    draw_game.team_side(&display, team_side);

    let mut accumulator = 0;
    let mut previous_clock = clock_ticks::precise_time_ns();
    let game_state = game::SharedState::new();

    // add some robots
    {
        let mut game_state = game_state.write().unwrap();
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
            let state = game_state.read().unwrap();
            draw_game.draw_to(&mut target, &state, view).unwrap();
        }

        target.finish().unwrap();

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
}
