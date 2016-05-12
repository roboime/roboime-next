extern crate roboime_next;
extern crate roboime_next_sim;
#[macro_use] extern crate glium;
extern crate clock_ticks;

use std::thread;
use std::time::Duration;
use glium::Surface;
use glium::glutin;
//use roboime_next::prelude::*;
use roboime_next::game;

pub mod models;
pub mod colors;

enum Action {
    Stop,
    Continue,
}

fn start_loop<F>(mut callback: F) where F: FnMut(&game::State) -> Action {
    let mut accumulator = 0;
    let mut previous_clock = clock_ticks::precise_time_ns();
    let game_state = game::State::new();

    loop {
        match callback(&game_state) {
            Action::Stop => break,
            Action::Continue => ()
        };

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

fn main() {
    use glium::DisplayBuild;

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .with_dimensions(1040, 740)
        .with_title(format!("RoboIME Next"))
        .build_glium()
        .unwrap();

    // compiling shaders and linking them together
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140

                uniform mat4 perspective;
                uniform mat4 view;
                uniform mat4 model;

                in vec3 position;
                in vec3 color;

                out vec3 vColor;

                void main() {
                    mat4 modelview = view * model;
                    gl_Position = perspective * modelview * vec4(position, 1.0);
                    vColor = color;
                }
            ",

            fragment: "
                #version 140
                in vec3 vColor;
                out vec4 f_color;

                void main() {
                    f_color = vec4(vColor, 1.0);
                }
            "
        },

        110 => {
            vertex: "
                #version 110

                uniform mat4 perspective;
                uniform mat4 view;
                uniform mat4 model;

                attribute vec3 position;
                attribute vec3 color;

                varying vec3 vColor;

                void main() {
                    mat4 modelview = view * model;
                    gl_Position = perspective * modelview * vec4(position, 1.0);
                    vColor = color;
                }
            ",

            fragment: "
                #version 110
                varying vec3 vColor;

                void main() {
                    gl_FragColor = vec4(vColor, 1.0);
                }
            ",
        },

        100 => {
            vertex: "
                #version 100

                uniform lowp mat4 perspective;
                uniform lowp mat4 view;
                uniform lowp mat4 model;

                attribute lowp vec3 position;
                attribute lowp vec3 color;

                varying lowp vec3 vColor;

                void main() {
                    mat4 lowp modelview = view * model;
                    gl_Position = perspective * modelview * vec4(position, 1.0);
                    vColor = color;
                }
            ",

            fragment: "
                #version 100
                varying lowp vec3 vColor;

                void main() {
                    gl_FragColor = vec4(vColor, 1.0);
                }
            ",
        },
    ).unwrap();

    let (vertex_buffer, index_buffer) = models::field(&display);

    // the main loop
    start_loop(|_game_state| {

        let mut target = display.draw();

        let model = {
            //let c = clock_ticks::precise_time_ns();
            //let t = (c as f32) / 1e9;
            [
                //[t.cos(), t.sin(), 0.0, 0.0],
                //[-t.sin(), t.cos(), 0.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };

        let view = view_matrix(&[0.0, 0.0, 10.0], &[0.0, 0.0, -1.0], &[0.0, 1.0, 0.0]);

        let perspective = {
            let (width, height) = target.get_dimensions();

            let scale = 2000.0;
            let x_scale = scale / width as f32;
            let y_scale = scale / height as f32;

            let zfar = 1024.0;
            let znear = 0.1;

            //let fov: f32 = 3.141592 / 3.0;
            //let f = 1.0 / (fov / 2.0).tan();
            //let aspect_ratio = height as f32 / width as f32;
            //let x_scale = f * aspect_ratio;
            //let y_scale = f;

            [
                [x_scale,   0.0  ,              0.0              ,   0.0],
                [  0.0  , y_scale,              0.0              ,   0.0],
                [  0.0  ,   0.0  ,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [  0.0  ,   0.0  , -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        };

        // building the uniforms
        let uniforms = uniform! {
            model: model,
            view: view,
            perspective: perspective
        };

        // drawing a frame
        let (r, g, b) = colors::DARK_GREEN;
        target.clear_color(r, g, b, 1.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return Action::Stop,
                _ => ()
            }
        }

        Action::Continue
    });
}

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}
