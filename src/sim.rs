pub use self::Side::{Left, Right};
pub use self::Color::{Yellow, Blue};

use std::collections::BTreeMap;
use ::prelude::*;
use ::game;

const FIELD_LENGTH: f32    = 9.010;
const CENTER_DIAMETER: f32 = 1.000;
const DEFENSE_RADIUS: f32  = 1.000;
const ROBOT_RADIUS: f32    = 0.090;
const BALL_RADIUS: f32     = 0.023;
const BALL_FRICT_LOSS: f32 = 0.020;

#[derive(Clone, Debug, PartialEq)]
pub struct Vec2d {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Vec3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ball {
    pub pos: Vec3d,
    pub vel: Vec3d,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Robot {
    pub pos: Vec2d,
    pub vel: Vec2d,
    pub w: f32,
    pub vw: f32,
}

impl Robot {
    #[inline]
    pub fn new() -> Robot {
        Robot { pos: Vec2d { x: 0.0, y: 0.0 }, w: 0.0, vel: Vec2d { x: 0.0, y: 0.0 }, vw: 0.0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Yellow,
    Blue,
}

impl Color {
    #[inline] pub fn is_yellow(&self) -> bool { *self == Yellow }
    #[inline] pub fn is_blue(&self)   -> bool { *self == Blue }
    #[inline] pub fn yellow(is_yellow: bool) -> Color { if is_yellow { Yellow } else { Blue   } }
    #[inline] pub fn blue(is_blue: bool)     -> Color { if is_blue   { Blue   } else { Yellow } }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Side {
    Left,
    Right
}

impl Side {
    #[inline] pub fn is_right(&self) -> bool { *self == Right }
    #[inline] pub fn is_left(&self)  -> bool { *self == Left }
    #[inline] pub fn right(is_right: bool) -> Side { if is_right { Right } else { Left  } }
    #[inline] pub fn left(is_left: bool)   -> Side { if is_left  { Left  } else { Right } }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TeamSide(pub Color, pub Side);

impl TeamSide {
    #[inline]
    pub fn yellow_is_left(&self) -> bool {
        match *self {
            TeamSide(Yellow, Left)  => true,
            TeamSide(Yellow, Right) => false,
            TeamSide(Blue,   Right) => true,
            TeamSide(Blue,   Left)  => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id(pub Color, pub u8);

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub timestamp: f32,
    pub ball: Ball,
    pub robots: BTreeMap<Id, Robot>,
    pub team_side: TeamSide,
}

trait BTreeMapSimExt {
    fn initial_formation(&mut self, count: u8, color: Color, side: Side);
}

impl BTreeMapSimExt for BTreeMap<Id, Robot> {
    fn initial_formation(&mut self, count: u8, color: Color, side: Side) {
        use std::f32::consts::PI;

        let w_0 = if side.is_right() { 0.0 } else { PI };
        let w_delta = 2.0 * PI / (count as f32) * if side.is_right() { 1.0 } else { -1.0 };
        let x_offset = (CENTER_DIAMETER / 4.0 + FIELD_LENGTH / 4.0 - DEFENSE_RADIUS / 2.0) * if side.is_right() { 1.0 } else { -1.0 };
        //let radius = FIELD_LENGTH / 8.0;
        let radius = (FIELD_LENGTH / 2.0 - CENTER_DIAMETER / 2.0 - DEFENSE_RADIUS) / 3.0 - ROBOT_RADIUS;

        for i in 0..count {
            let robot_id = Id(color, i);
            let robot = self.entry(robot_id).or_insert(Robot::new());
            let w = w_0 + (i as f32) * w_delta;
            robot.pos.x = radius * w.cos() + x_offset;
            robot.pos.y = radius * w.sin();
            robot.w = w + PI;
        }
    }
}

impl State {
    pub fn new(team_side: TeamSide) -> State {
        let mut robots = BTreeMap::new();
        if team_side.yellow_is_left() {
            robots.initial_formation(6, Yellow, Left);
            robots.initial_formation(6, Blue, Right);
        } else {
            robots.initial_formation(6, Yellow, Right);
            robots.initial_formation(6, Blue, Left);
        }
        State {
            timestamp: 0.0,
            //ball: Ball { pos: Vec3d { x: 0.0, y: 0.0, z: 0.0 }, vel: Vec3d { x: 0.0, y: 0.0, z: 0.0 } },
            ball: Ball { pos: Vec3d { x: 0.0, y: 0.05, z: 0.0 }, vel: Vec3d { x: -4.0, y: 0.0, z: 0.0 } },
            robots: robots,
            team_side: team_side,
        }
    }

    pub fn update_game(&self, game_state: &mut game::State) {
        // ball
        {
            let game_ball = game_state.get_ball_mut();
            game_ball.set_x(self.ball.pos.x);
            game_ball.set_y(self.ball.pos.y);
            //game_ball.set_z(self.ball.pos.z);
            game_ball.set_vx(self.ball.vel.x);
            game_ball.set_vy(self.ball.vel.y);
            //game_ball.set_vz(self.ball.vel.z);
        }
        // robots
        for (id, ref robot) in self.robots.iter() {
            let (robot_id, ref mut robots) = match id {
                &Id(Blue, i)   => (i, game_state.get_robots_blue_mut()),
                &Id(Yellow, i) => (i, game_state.get_robots_yellow_mut()),
            };
            let game_robot = robots.entry(robot_id).or_insert(game::Robot::new(robot_id));
            game_robot.set_x(robot.pos.x);
            game_robot.set_y(robot.pos.y);
            game_robot.set_w(robot.w);
            game_robot.set_vx(robot.vel.x);
            game_robot.set_vy(robot.vel.y);
            game_robot.set_vw(robot.vw);
        }
        game_state.inc_counter();
    }

    pub fn step(&mut self, command: game::Command, timestep: f32) {
        self.timestamp += timestep;
        let color = Color::yellow(command.is_yellow);
        let &mut State { ref mut ball, ref mut robots, .. } = self;
        let mut d_time_ball = timestep;

        // XXX: overly simplified physics ahead
        for (id, robot_command) in command.robots {
            let robot_id = &Id(color, id);
            if let Some(mut robot) = robots.get_mut(robot_id) {
                let d_time = timestep;

                let v_tangent = robot_command.v_tangent;
                let v_normal  = robot_command.v_normal;
                let v_angular = robot_command.v_angular;

                let w = robot.w;

                let vx = v_normal * w.sin() + v_tangent * w.cos();
                let vy = v_normal * w.cos() - v_tangent * w.sin();
                let vw = v_angular;

                let dx = d_time * vx;
                let dy = d_time * vy;
                let dw = d_time * vw;

                // detect collision with ball
                {
                    let rr = ROBOT_RADIUS;
                    let rb = BALL_RADIUS;
                    let xr  = robot.pos.x;
                    let yr  = robot.pos.y;
                    let vrx = robot.pos.x;
                    let vry = robot.pos.y;
                    let xb  = ball.pos.x;
                    let yb  = ball.pos.y;
                    let vbx = ball.vel.x;
                    let vby = ball.vel.y;

                    // Bhāskara:
                    let a = (vry - vby) * (vry - vby) + (vrx - vbx) * (vrx - vbx);
                    let b = 2.0 * ((xr - xb) * (vrx - vbx) + (yr - yb) * (vry - vby));
                    let c = (xr - xb) * (xr - xb) + (yr - yb) * (yr - yb) - (rr + rb) * (rr + rb);

                    if c > 0.0 && a != 0.0 {
                        //debug!("there may be a collision");
                        let tc = (-b - (b * b - 4.0 * a * c).sqrt()) * (0.5 / a);
                        if tc >= 0.0 && tc <= timestep {
                            use std::f32::consts::PI;

                            let xbc = xb + tc * vbx;
                            let ybc = yb + tc * vby;
                            let xrc = xr + tc * vrx;
                            let yrc = yr + tc * vry;
                            debug!("there is a collision, with robot {:?} at {},{}_{},{}", robot_id, xrc, yrc, xbc, ybc);
                            // TODO: dummy collision
                            ball.pos.x = xbc;
                            ball.pos.y = ybc;
                            let wc = (ybc - yrc).atan2(xbc - xrc);
                            let wb = vby.atan2(vbx);
                            let v = (vbx * vbx + vby * vby).sqrt();
                            let w = PI + wc - wb;
                            ball.vel.x = v * w.cos() + vrx;
                            ball.vel.y = v * w.sin() + vry;
                            d_time_ball -= tc;
                        }
                    }
                }

                //println!("dx: {}", dx);

                robot.pos.x += dx;
                robot.pos.y += dy;
                robot.w += dw;

                // TODO: effect of robot_command.action
            }
        }

        // update ball pos
        {
            ball.pos.x += d_time_ball * ball.vel.x;
            ball.pos.y += d_time_ball * ball.vel.y;
            ball.pos.z += d_time_ball * ball.vel.z;
            let r = 1.0 - BALL_FRICT_LOSS;
            ball.vel.x *= r;
            ball.vel.y *= r;
            ball.vel.z *= r;
        }
    }
}
