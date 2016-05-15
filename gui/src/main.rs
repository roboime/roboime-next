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

enum Action {
    Stop,
    Continue,
}

fn start_loop<F>(mut callback: F) where F: FnMut(&game::State) -> Action {
    use std::thread;
    use std::time::Duration;

    let mut accumulator = 0;
    let mut previous_clock = clock_ticks::precise_time_ns();
    let mut game_state = game::State::new();

    // add some robots
    add_initial_robots(game_state.get_robots_blue_mut(), 6, false);
    add_initial_robots(game_state.get_robots_yellow_mut(), 6, true);

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
    use glium::glutin::GlRequest;

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .with_title(format!("RoboIME Next"))
        .with_dimensions(1040, 740)
        .with_depth_buffer(30)
        .with_gl(GlRequest::Latest)
        .with_srgb(Some(true))
        .with_multisampling(8)
        .build_glium()
        .unwrap();

    // compiling shaders and linking them together
    let program = program!(&display,
        150 => { vertex: include_str!("vertex.glsl"), fragment: include_str!("fragment.glsl"), outputs_srgb: true },
    ).unwrap();

    let simple_program = program!(&display,
        150 => {
            vertex: r#"
                #version 150

                uniform mat4 perspective;
                uniform mat4 view;
                uniform mat4 model;
                in vec3 position;
                in vec3 color;
                out vec3 v_color;

                void main() {
                    mat4 modelview = view * model;
                    gl_Position = perspective * modelview * vec4(position, 1.0);
                    v_color = color;
                }
            "#,

            fragment: r#"
                #version 150

                in vec3 v_color;
                out vec4 f_color;
                //const float gamma = 1.5;

                void main() {
                    //vec3 c_color = v_color * 0.9;
                    //vec3 color = vec3(pow(c_color.x, gamma), pow(c_color.y, gamma), pow(c_color.z, gamma));
                    //f_color = vec4(color, 1.0);
                    f_color = vec4(v_color, 1.0);
                }
            "#,

            outputs_srgb: true
        },
    ).unwrap();

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLessOrEqual,
            write: true,
            .. Default::default()
        },
        backface_culling:  glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
        .. Default::default()
    };

    let params_pre = glium::DrawParameters {
        depth: glium::Depth {
            write: true,
            .. Default::default()
        },
        .. params.clone()
    };

    let light = [-1.0, 0.4, -0.9f32];
    let bg_color = { let (r, g, b) = colors::DARK_GREEN; (r, g, b, 1.0) };

    let field = models::field(&display, TeamSide::BlueIsLeft);
    let ball  = models::ball(&display);
    let (yellow_robots, blue_robots) = {
        use std::collections::HashMap;

        let (n, m) = (17, 11);

        let mut robots_y = HashMap::with_capacity(12);
        for i in 0..12 {
            let robot = models::robot(&display, i, true, n, m);
            robots_y.insert(i, robot);
        }

        let mut robots_b = HashMap::with_capacity(12);
        for i in 0..12 {
            let robot = models::robot(&display, i, false, n, m);
            robots_b.insert(i, robot);
        }

        (robots_y, robots_b)
    };

    //let normal_map = {
    //    let image = image::load(Cursor::new(&include_bytes!("bumps.png")[..]), image::PNG).unwrap().to_rgba();
    //    let image_dimensions = image.dimensions();
    //    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    //    glium::texture::Texture2d::new(&display, image).unwrap()
    //};

    // the main loop
    start_loop(|game_state| {

        let mut target = display.draw();

        let view = view_matrix(&[0.0, 0.0, 10.0], &[0.0, 0.0, -1.0], &[0.0, 1.0, 0.0]);
        //let view = view_matrix(&[-4.0, -4.0, 4.0], &[1.0, 1.0, -1.0], &[0.0, 0.0, 1.0]);
        //let view = view_matrix(&[4.0, -4.0, 4.0], &[-1.0, 1.0, -1.0], &[0.0, 0.0, 1.0]);
        //let view = view_matrix(&[-0.2, -0.2, 0.6], &[0.2, 0.2, -0.6], &[0.0, 0.0, 1.0]);

        let perspective = {
            let (width, height) = target.get_dimensions();
            perspective_matrix(width as f32, height as f32)
        };

        target.clear_color_srgb_and_depth(bg_color, 1.0);

        target.draw(&field.0, &field.1, &simple_program, &uniform! {
            model: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            view: view,
            perspective: perspective,
        }, &params_pre).unwrap();

        target.draw(&ball.0, &ball.1, &program, &uniform! {
            model: game_state.get_ball().model_matrix(),
            view: view,
            perspective: perspective,
            u_light: light,
        }, &params).unwrap();

        for (robot_id, robot_state) in game_state.get_robots_blue() {
            let ref robot = blue_robots[robot_id];
            target.draw(&robot.0, &robot.1, &program, &uniform! {
                model: robot_state.model_matrix(),
                view: view,
                perspective: perspective,
                u_light: light,
            }, &params).unwrap();
        }

        for (robot_id, robot_state) in game_state.get_robots_yellow() {
            let ref robot = yellow_robots[robot_id];
            target.draw(&robot.0, &robot.1, &program, &uniform! {
                model: robot_state.model_matrix(),
                view: view,
                perspective: perspective,
                u_light: light,
            }, &params).unwrap();
        }

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
