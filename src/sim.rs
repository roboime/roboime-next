use std::collections::BTreeMap;
use ::prelude::*;
use ::base::*;
use ::game;

const FIELD_LENGTH: f32    = 9.010;
const CENTER_DIAMETER: f32 = 1.000;
const DEFENSE_RADIUS: f32  = 1.000;
const ROBOT_RADIUS: f32    = 0.090;
const BALL_RADIUS: f32     = 0.023;
const BALL_FRICT_LOSS: f32 = 1.000;

#[derive(Clone, Debug, PartialEq)]
pub struct Ball {
    //pub pos: Vec3d,
    //pub vel: Vec3d,
    pub pos: Vec2d,
    pub vel: Vec2d,
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
        Robot { pos: Vec2d(0.0, 0.0), w: 0.0, vel: Vec2d(0.0, 0.0), vw: 0.0 }
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
            robot.pos.0 = radius * w.cos() + x_offset;
            robot.pos.1 = radius * w.sin();
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
            //ball: Ball { pos: Vec2d(0.0, 0.0), vel: Vec2d(0.0, 0.0) },
            ball: Ball { pos: Vec2d(0.0, 0.05), vel: Vec2d(-4.0, 0.0) },
            robots: robots,
            team_side: team_side,
        }
    }

    pub fn update_game(&self, game_state: &mut game::State) {
        // ball
        {
            let game_ball = game_state.get_ball_mut();
            game_ball.set_x(self.ball.pos.0);
            game_ball.set_y(self.ball.pos.1);
            //game_ball.set_z(self.ball.pos.z);
            game_ball.set_vx(self.ball.vel.0);
            game_ball.set_vy(self.ball.vel.1);
            //game_ball.set_vz(self.ball.vel.z);
        }
        // robots
        for (id, ref robot) in self.robots.iter() {
            let (robot_id, ref mut robots) = match id {
                &Id(Blue, i)   => (i, game_state.get_robots_blue_mut()),
                &Id(Yellow, i) => (i, game_state.get_robots_yellow_mut()),
            };
            let game_robot = robots.entry(robot_id).or_insert(game::Robot::new(robot_id));
            game_robot.set_x(robot.pos.0);
            game_robot.set_y(robot.pos.1);
            game_robot.set_w(robot.w);
            game_robot.set_vx(robot.vel.0);
            game_robot.set_vy(robot.vel.1);
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

                robot.vel = Vec2d(v_tangent, v_normal).rotate(-robot.w);
                robot.vw = v_angular;

                // detect collision with ball
                {
                    let r = ROBOT_RADIUS + BALL_RADIUS;

                    // Bhāskara:
                    let a = (robot.vel - ball.vel).norm2();
                    let b = 2.0 * ((robot.pos - ball.pos) * (robot.vel - ball.vel));
                    let c = (robot.pos - ball.pos).norm2() - r * r;
                    let delta = b * b - 4.0 * a * c;

                    if delta >= 0.0 && a != 0.0 {
                        let tc = (-b - delta.sqrt()) * 0.5 / a;
                        if tc >= 0.0 && tc <= timestep {
                            use std::f32::consts::PI;

                            debug!("collision: ball and robot {:?}", robot_id);

                            ball.pos += ball.vel * tc;
                            let robot_pos = robot.pos + robot.vel * tc;
                            let w = (ball.pos - robot_pos).angle();
                            ball.vel = robot.vel + (ball.vel - robot.vel).rotate(2.0 * w - PI);

                            d_time_ball -= tc;
                        }
                    }
                }

                robot.pos += robot.vel * d_time;
                robot.w   += robot.vw  * d_time;

                // TODO: effect of robot_command.action
            }
        }

        // update ball pos
        ball.pos += ball.vel * d_time_ball ;
        ball.vel *= 1.0 - BALL_FRICT_LOSS * timestep;
    }
}
