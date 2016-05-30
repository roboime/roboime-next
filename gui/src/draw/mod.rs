use std::collections::HashMap;
use roboime_next::game::State as GameState;
use glium::program::{Program, ProgramChooserCreationError};
use glium::backend::Facade;
use glium::draw_parameters;
use glium::{Surface, DrawError, Depth, DrawParameters, VertexBuffer, IndexBuffer};
use ::colors;
use ::utils::*;
use ::models::*;

pub struct Game<'a> {
    program: Program,
    simple_program: Program,
    params: DrawParameters<'a>,
    simple_params: DrawParameters<'a>,
    light: [f32; 3],
    bg_color: (f32, f32, f32, f32),
    field: (VertexBuffer<Vertex>, IndexBuffer<u16>),
    goals: (VertexBuffer<RichVertex>, IndexBuffer<u16>),
    ball: (VertexBuffer<RichVertex>, IndexBuffer<u16>),
    yellow_robots: HashMap<u8, (VertexBuffer<RichVertex>, IndexBuffer<u16>)>,
    blue_robots: HashMap<u8, (VertexBuffer<RichVertex>, IndexBuffer<u16>)>,
}

impl<'a> Game<'a> {
    pub fn new<F: Facade>(display: &F) -> Result<Game, ProgramChooserCreationError> {
        // compiling shaders and linking them together
        let program = try!(program!(display,
            150 => {
                vertex: include_str!("vertex.glsl"),
                fragment: include_str!("fragment.glsl"),
                outputs_srgb: true },
        ));

        let simple_program = try!(program!(display,
            150 => {
                vertex: include_str!("vertex_simple.glsl"),
                fragment: include_str!("fragment_simple.glsl"),
                outputs_srgb: true
            },
        ));

        let params = DrawParameters {
            depth: Depth {
                test: draw_parameters::DepthTest::IfLessOrEqual,
                write: true,
                .. Default::default()
            },
            backface_culling: draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            .. Default::default()
        };

        let simple_params = DrawParameters {
            depth: Depth {
                write: true,
                .. Default::default()
            },
            .. params.clone()
        };

        //let normal_map = {
        //    let image = image::load(Cursor::new(&include_bytes!("bumps.png")[..]), image::PNG).unwrap().to_rgba();
        //    let image_dimensions = image.dimensions();
        //    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
        //    glium::texture::Texture2d::new(&display, image).unwrap()
        //};

        //let light = [-1.0, 0.4, -0.9];
        let light = [-0.2, 2.0, -1.0];
        let bg_color = { let (r, g, b) = colors::DARK_GREEN; (r, g, b, 1.0) };

        let team_side = TeamSide::Undefined;
        let field = field(display, team_side);
        let goals = goals(display, team_side);
        let ball  = ball(display);
        let (yellow_robots, blue_robots) = {

            let (n, m) = (17, 11);

            let mut robots_y = HashMap::with_capacity(12);
            for i in 0..12 {
                let robot = robot(display, i, true, n, m);
                robots_y.insert(i, robot);
            }

            let mut robots_b = HashMap::with_capacity(12);
            for i in 0..12 {
                let robot = robot(display, i, false, n, m);
                robots_b.insert(i, robot);
            }

            (robots_y, robots_b)
        };

        Ok(Game {
            program: program,
            simple_program: simple_program,
            params: params,
            simple_params: simple_params,
            light: light,
            bg_color: bg_color,
            field: field,
            goals: goals,
            ball: ball,
            yellow_robots: yellow_robots,
            blue_robots: blue_robots,
        })
    }

    pub fn team_side<F: Facade>(&mut self, display: &F, team_side: TeamSide) {
        let field = field(display, team_side);
        let goals = goals(display, team_side);
        self.field = field;
        self.goals = goals;
    }

    pub fn draw_to<S: Surface>(&self, target: &mut S, game_state: &GameState, perspective: [[f32; 4]; 4], view: [[f32; 4]; 4]) -> Result<(), DrawError> {
        let &Game {
            ref program,
            ref simple_program,
            ref params,
            ref simple_params,
            light,
            bg_color,
            field: (ref field_vb, ref field_ib),
            goals: (ref goals_vb, ref goals_ib),
            ball: (ref ball_vb, ref ball_ib),
            ref yellow_robots,
            ref blue_robots,
            ..
        } = self;

        target.clear_color_srgb_and_depth(bg_color, 1.0);

        try!(target.draw(field_vb, field_ib, simple_program, &uniform! {
            model: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            view: view,
            perspective: perspective,
        }, simple_params));

        try!(target.draw(goals_vb, goals_ib, program, &uniform! {
            model: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            view: view,
            perspective: perspective,
            u_light: light,
        }, params));

        try!(target.draw(ball_vb, ball_ib, program, &uniform! {
            model: game_state.get_ball().model_matrix(),
            view: view,
            perspective: perspective,
            u_light: light,
        }, params));

        for (robot_id, robot_state) in game_state.get_robots_blue() {
            let (ref robot_vb, ref robot_ib) = blue_robots[robot_id];
            try!(target.draw(robot_vb, robot_ib, program, &uniform! {
                model: robot_state.model_matrix(),
                view: view,
                perspective: perspective,
                u_light: light,
            }, params));
        }

        for (robot_id, robot_state) in game_state.get_robots_yellow() {
            let (ref robot_vb, ref robot_ib) = yellow_robots[robot_id];
            try!(target.draw(robot_vb, robot_ib, program, &uniform! {
                model: robot_state.model_matrix(),
                view: view,
                perspective: perspective,
                u_light: light,
            }, params));
        }

        Ok(())
    }
}
