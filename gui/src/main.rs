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

pub mod models;
pub mod colors;

enum Action {
    Stop,
    Continue,
}

fn start_loop<F>(mut callback: F) where F: FnMut(&game::State) -> Action {
    use std::thread;
    use std::time::Duration;

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
    use glium::glutin::GlRequest;

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .with_title(format!("RoboIME Next"))
        .with_dimensions(1040, 740)
        .with_depth_buffer(24)
        .with_gl(GlRequest::Latest)
        .with_srgb(Some(true))
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

    let light = [-1.0, 0.4, -0.9f32];

    let field = models::field(&display);
    //let ball = models::uv_sphere(&display, 0.023, 12, 24, colors::ORANGE);
    //let ball = models::uv_sphere(&display, 0.023, 6, 12, colors::ORANGE);
    //let ball = models::sub_icosahedron(&display, 0.023, 3, colors::ORANGE);
    let ball = models::sub_icosahedron(&display, 0.023, 2, colors::ORANGE);

    //let normal_map = {
    //    let image = image::load(Cursor::new(&include_bytes!("bumps.png")[..]), image::PNG).unwrap().to_rgba();
    //    let image_dimensions = image.dimensions();
    //    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    //    glium::texture::Texture2d::new(&display, image).unwrap()
    //};

    // the main loop
    start_loop(|_game_state| {

        let mut target = display.draw();

        let default_model = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        let model = {
            let c = clock_ticks::precise_time_ns();
            let t = (c as f32) / 0.5e9;
            //let s = 1.000;
            //let r = 0.023;
            //let s = 46.511627f32;
            let s = 1.0;
            let r = 1.0;
            let (x, y, z) = (2.000, 1.000, 0.023);
            [
                [ r * t.cos(),  r * t.sin(), 0.0, 0.0],
                [-r * t.sin(),  r * t.cos(), 0.0, 0.0],
                [     0.0    ,      0.0    ,  r , 0.0],
                [      x     ,       y     ,  z ,  s ]
            ]
        };

        let view = view_matrix(&[0.0, 0.0, 10.0], &[0.0, 0.0, -1.0], &[0.0, 1.0, 0.0]);
        //let view = view_matrix(&[-4.0, -4.0, 4.0], &[1.0, 1.0, -1.0], &[0.0, 0.0, 1.0]);

        let perspective = {
            let (width, height) = target.get_dimensions();

            let scale = 2000.0;
            let x_scale = -scale / width as f32;
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
            perspective: perspective,
            u_light: light,
            //normal_tex: &normal_map
        };
        let simple_uniforms = uniform! {
            model: default_model,
            view: view,
            perspective: perspective,
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        // drawing a frame
        let color = { let (r, g, b) = colors::DARK_GREEN; (r, g, b, 1.0) };
        target.clear_color_srgb_and_depth(color, 1.0);
        target.draw(&field.0, &field.1, &simple_program, &simple_uniforms, &params).unwrap();
        target.draw(&field.0, &field.1, &simple_program, &simple_uniforms, &Default::default()).unwrap();
        target.draw(&ball.0, &ball.1, &program, &uniforms, &params).unwrap();
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
