use std::borrow::Cow;
use std::collections::HashMap;
use roboime_next::prelude::*;
use roboime_next::game;
use glium::program::{Program, ProgramChooserCreationError};
use glium::backend::Facade;
use glium::{texture, draw_parameters, index, uniforms};
use glium::{Surface, DrawError, Depth, DrawParameters, VertexBuffer, IndexBuffer};
use rusttype::{FontCollection, Font};
use rusttype::gpu_cache::{Cache};
pub use self::utils::*;
pub use self::models::*;

pub mod colors;
mod models;
mod utils;

#[derive(Copy, Clone)]
struct GlyphVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
    color: [f32; 4]
}
implement_vertex!(GlyphVertex, position, tex_coords, color);

pub struct Game<'a> {
    program: Program,
    simple_program: Program,
    texture_program: Program,
    params: DrawParameters<'a>,
    simple_params: DrawParameters<'a>,
    simple_alpha_params: DrawParameters<'a>,
    texture_params: DrawParameters<'a>,
    font: Font<'a>,
    dpi_factor: f32,
    cache: Cache,
    cache_tex: texture::Texture2d,
    light: [f32; 3],
    bg_color: (f32, f32, f32, f32),
    field: (VertexBuffer<Vertex>, IndexBuffer<u16>),
    exclusion_zone: (VertexBuffer<Vertex>, IndexBuffer<u16>),
    circles: [(VertexBuffer<Vertex>, IndexBuffer<u16>); 3],
    dotted_circle: (VertexBuffer<Vertex>, IndexBuffer<u16>),
    dotted_line: (VertexBuffer<Vertex>, IndexBuffer<u16>),
    goals: (VertexBuffer<RichVertex>, IndexBuffer<u16>),
    ball: (VertexBuffer<RichVertex>, IndexBuffer<u16>),
    yellow_robots: HashMap<u8, (VertexBuffer<RichVertex>, IndexBuffer<u16>)>,
    blue_robots: HashMap<u8, (VertexBuffer<RichVertex>, IndexBuffer<u16>)>,
    score_text_vertex: Option<VertexBuffer<GlyphVertex>>,
    team_side: TeamSide,
    score: (u8, u8),
}

impl<'a> Game<'a> {
    pub fn new<F: Facade>(display: &F) -> Result<Game, ProgramChooserCreationError> {
        let font_data = include_bytes!("fonts/Roboto-Light.ttf");
        let font = FontCollection::from_bytes(font_data as &[u8]).into_font().unwrap();

        //let dpi_factor = display.get_window().unwrap().hidpi_factor();
        let dpi_factor = 1.0; // FIXME
        let (cache_width, cache_height) = (2048 * dpi_factor as u32, 2048 * dpi_factor as u32);
        let cache = Cache::new(cache_width, cache_height, 0.1, 0.1);
        let cache_tex = texture::Texture2d::with_format(
            display,
            texture::RawImage2d {
                data: Cow::Owned(vec![128u8; cache_width as usize * cache_height as usize]),
                width: cache_width,
                height: cache_height,
                format: texture::ClientFormat::U8
            },
            texture::UncompressedFloatFormat::U8,
            texture::MipmapsOption::NoMipmap
        ).unwrap();

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

        let texture_program = try!(program!(display,
            150 => {
                vertex: include_str!("vertex_texture.glsl"),
                fragment: include_str!("fragment_texture.glsl"),
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

        let simple_alpha_params = {
            use glium::{Blend, BlendingFunction, LinearBlendingFactor};
            DrawParameters {
                blend: Blend {
                    color: BlendingFunction::Addition {
                        source: LinearBlendingFactor::ConstantAlpha,
                        destination: LinearBlendingFactor::OneMinusConstantAlpha,
                    },
                    alpha: BlendingFunction::AlwaysReplace,
                    constant_value: (0.0, 0.0, 0.0, 0.5),
                },
                .. simple_params.clone()
            }
        };

        let texture_params = DrawParameters {
            blend: draw_parameters::Blend::alpha_blending(),
            .. Default::default()
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
        let blue = { let (r, g, b) = colors::PATTERN_BLUE; [r, g, b] };
        let yellow = { let (r, g, b) = colors::PATTERN_YELLOW; [r, g, b] };
        let white = { let (r, g, b) = colors::WHITE; [r, g, b] };

        let team_side = Default::default();
        let field = field(display, team_side);
        let exclusion_zone = exclusion_zone(display);
        let dotted_circle = dotted_circle(display);
        let dotted_line = dotted_line(display);
        let circle_blue   = circle(display, blue);
        let circle_yellow = circle(display, yellow);
        let circle_white = circle(display, white);
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
            texture_program: texture_program,
            params: params,
            simple_params: simple_params,
            simple_alpha_params: simple_alpha_params,
            texture_params: texture_params,
            font: font,
            dpi_factor: dpi_factor,
            cache: cache,
            cache_tex: cache_tex,
            light: light,
            bg_color: bg_color,
            field: field,
            exclusion_zone: exclusion_zone,
            circles: [circle_blue, circle_yellow, circle_white],
            dotted_circle: dotted_circle,
            dotted_line: dotted_line,
            goals: goals,
            ball: ball,
            yellow_robots: yellow_robots,
            blue_robots: blue_robots,
            score_text_vertex: None,
            team_side: team_side,
            score: (0, 0),
        })
    }

    pub fn team_side<F: Facade>(&mut self, display: &F, team_side: TeamSide) {
        if team_side != self.team_side {
            let field = field(display, team_side);
            let goals = goals(display, team_side);
            self.field = field;
            self.goals = goals;
            self.team_side = team_side;
        }
    }

    pub fn update<'g, F: Facade, G: game::State<'g>>(&mut self, display: &F, game_state: &'g G) {
        let score_blue = game_state.team_info(Blue).score();
        let score_yellow = game_state.team_info(Yellow).score();
        let score = self.team_side.sort_side(score_blue, score_yellow);
        if score != self.score {
            self.score = score;
            let color = { let (r, g, b) = colors::WHITE; [r, g, b, 1.0] };
            //let color = {
            //    let (r, g, b) = if score_blue > score_yellow {
            //        colors::PATTERN_BLUE
            //    } else if score_yellow > score_blue {
            //        colors::PATTERN_YELLOW
            //    } else {
            //        colors::WHITE
            //    };
            //    [r, g, b, 1.0]
            //};
            self.set_score_text(display, score, color);
        }
    }

    pub fn set_score_text<F: Facade>(&mut self, display: &F, score: (u8, u8), color: [f32; 4]) {
        use glium::{Rect as GRect};
        use rusttype::{Rect, Scale, point, vector};

        let &mut Game {
            ref font,
            dpi_factor,
            ref mut cache,
            ref mut cache_tex,
            ref mut score_text_vertex,
            ..
        } = self;

        let (score_left, score_right) = score;
        let text_left = format!("{}", score_left);
        let text_center = format!(" ― ");
        let text_right = format!("{}", score_right);

        let zoom = 200.0;
        //let _corrected_width = (width * zoom) as u32;
        let glyphs = layout_align(font, Scale::uniform(60.0 * dpi_factor), &text_left, &text_center, &text_right);
        for glyph in &glyphs {
            cache.queue_glyph(0, glyph.clone());
        }
        cache.cache_queued(|rect, data| {
            cache_tex.main_level().write(GRect {
                left: rect.min.x,
                bottom: rect.min.y,
                width: rect.width(),
                height: rect.height()
            }, texture::RawImage2d {
                data: Cow::Borrowed(data),
                width: rect.width(),
                height: rect.height(),
                format: texture::ClientFormat::U8
            });
        }).unwrap();
        let vertex_buffer = {
            let origin = point(0.0, 0.0);
            let vertices: Vec<GlyphVertex> = glyphs.iter().flat_map(|g| {
                if let Ok(Some((uv_rect, screen_rect))) = cache.rect_for(0, g) {
                    let gl_rect = Rect {
                        min: origin + vector(screen_rect.min.x as f32, -screen_rect.min.y as f32) / zoom,
                        max: origin + vector(screen_rect.max.x as f32, -screen_rect.max.y as f32) / zoom,
                    };
                    vec![
                        GlyphVertex {
                            position: [gl_rect.min.x, gl_rect.max.y],
                            tex_coords: [uv_rect.min.x, uv_rect.max.y],
                            color: color
                        },
                        GlyphVertex {
                            position: [gl_rect.min.x,  gl_rect.min.y],
                            tex_coords: [uv_rect.min.x, uv_rect.min.y],
                            color: color
                        },
                        GlyphVertex {
                            position: [gl_rect.max.x,  gl_rect.min.y],
                            tex_coords: [uv_rect.max.x, uv_rect.min.y],
                            color: color
                        },
                        GlyphVertex {
                            position: [gl_rect.max.x,  gl_rect.min.y],
                            tex_coords: [uv_rect.max.x, uv_rect.min.y],
                            color: color },
                        GlyphVertex {
                            position: [gl_rect.max.x, gl_rect.max.y],
                            tex_coords: [uv_rect.max.x, uv_rect.max.y],
                            color: color
                        },
                        GlyphVertex {
                            position: [gl_rect.min.x, gl_rect.max.y],
                            tex_coords: [uv_rect.min.x, uv_rect.max.y],
                            color: color
                        },
                    ]
                } else {
                    vec![]
                }
            }).collect();

            VertexBuffer::new(display, &vertices).unwrap()
        };

        *score_text_vertex = Some(vertex_buffer);
    }

    pub fn draw_to<'g, S: Surface, G: game::State<'g>>(&self, target: &mut S, game_state: &'g G, view_port: (u32, u32), view: [[f32; 4]; 4]) -> Result<(), DrawError> {
        use roboime_next::prelude::{Id, Blue, Yellow, Robot, State};
        use roboime_next::game::Referee::*;

        let &Game {
            ref program,
            ref simple_program,
            ref texture_program,
            ref params,
            ref simple_params,
            ref simple_alpha_params,
            ref texture_params,
            ref cache_tex,
            light,
            bg_color,
            field: (ref field_vb, ref field_ib),
            exclusion_zone: (ref exclusion_vb, ref exclusion_ib),
            ref circles,
            dotted_circle: (ref dotted_vb, ref dotted_ib),
            dotted_line: (ref dotted_line_vb, ref dotted_line_ib),
            goals: (ref goals_vb, ref goals_ib),
            ball: (ref ball_vb, ref ball_ib),
            ref yellow_robots,
            ref blue_robots,
            ref score_text_vertex,
            ..
        } = self;

        target.clear_color_srgb_and_depth(bg_color, 1.0);

        let (width, height) = view_port;
        let perspective = perspective_matrix(width as f32, height as f32);
        let timestamp = game_state.timestamp();

        try!(target.draw(field_vb, field_ib, simple_program, &uniform! {
            model: IDENT,
            view: view,
            perspective: perspective,
        }, simple_params));

        let referee = game_state.referee();

        match referee {
            DirectFree(_) |
            IndirectFree(_) |
            Stop => {
                try!(target.draw(exclusion_vb, exclusion_ib, simple_program, &uniform! {
                    model: IDENT,
                    view: view,
                    perspective: perspective,
                }, simple_alpha_params));
                try!(target.draw(exclusion_vb, exclusion_ib, simple_program, &uniform! {
                    model: INVXY,
                    view: view,
                    perspective: perspective,
                }, simple_alpha_params));
            }
            PrePenalty(_) |
            Penalty(_) => {
                try!(target.draw(dotted_line_vb, dotted_line_ib, simple_program, &uniform! {
                    model: IDENT,
                    view: view,
                    perspective: perspective,
                }, simple_alpha_params));
                try!(target.draw(dotted_line_vb, dotted_line_ib, simple_program, &uniform! {
                    model: INVXY,
                    view: view,
                    perspective: perspective,
                }, simple_alpha_params));
            }
            _ => ()
        }

        match referee {
            PreKickoff(_) |
            Kickoff(_) |
            DirectFree(_) |
            IndirectFree(_) |
            Stop => {
                let Vec2d(x, y) = game_state.ball().pos();
                let w = timestamp as f32 * 0.5;
                try!(target.draw(dotted_vb, dotted_ib, simple_program, &uniform! {
                    model: xyzw_matrix(x, y, 0.0, w),
                    view: view,
                    perspective: perspective,
                }, simple_alpha_params));
            }
            _ => ()
        }

        if let Some((Vec2d(x, y), maybe_color)) = game_state.ball_positioning() {
            //let Vec2d(x, y) = game_state.ball().pos();
            let &(ref circle_vb, ref circle_ib) = match maybe_color {
                Some(Blue) => &circles[0],
                Some(Yellow) => &circles[1],
                None => &circles[2],
            };
            try!(target.draw(circle_vb, circle_ib, simple_program, &uniform! {
                model: xyz_matrix(x, y, 0.0),
                view: view,
                perspective: perspective,
            }, simple_alpha_params));
        }

        if let &Some(ref vertex_buffer) = score_text_vertex {
            use self::models::FIELD_WIDTH;
            try!(target.draw(
                vertex_buffer,
                index::NoIndices(index::PrimitiveType::TrianglesList),
                texture_program,
                &uniform! {
                    model: xyz_matrix(0.0, FIELD_WIDTH / 2.0 + 0.050, 0.0),
                    view: view,
                    tex: cache_tex.sampled().magnify_filter(uniforms::MagnifySamplerFilter::Nearest),
                    perspective: perspective,
                },
                texture_params
            ));
        }

        try!(target.draw(goals_vb, goals_ib, program, &uniform! {
            model: IDENT,
            view: view,
            perspective: perspective,
            u_light: light,
        }, params));

        try!(target.draw(ball_vb, ball_ib, program, &uniform! {
            model: ball_model_matrix(&game_state.ball()),
            view: view,
            perspective: perspective,
            u_light: light,
        }, params));

        for robot in game_state.robots() {
            let &(ref robot_vb, ref robot_ib) = match robot.id() {
                Id(Blue, id) => blue_robots.get(&id).unwrap(),
                Id(Yellow, id) => yellow_robots.get(&id).unwrap(),
            };
            try!(target.draw(robot_vb, robot_ib, program, &uniform! {
                model: robot_model_matrix(&robot),
                view: view,
                perspective: perspective,
                u_light: light,
            }, params));
        }

        Ok(())
    }
}
