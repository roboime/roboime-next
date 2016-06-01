use std::collections::BTreeMap;
use ::prelude::*;
use ::base::*;
use ::game;

const FIELD_LENGTH: f32    = 9.010;
const CENTER_DIAMETER: f32 = 1.000;
const DEFENSE_RADIUS: f32  = 1.000;
const ROBOT_RADIUS: f32    = 0.090;
const ROBOT_MOUTH: f32     = 0.500;
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

impl Id {
    pub fn color(self) -> Color { self.0 }
    pub fn id(self) -> u8 { self.1 }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Referee {
    Halt,
    Stop,
    Normal,
    PreKickoff(Color),
    Kickoff(Color),
    PrePenalty(Color),
    Penalty(Color),
    DirectFree(Color),
    IndirectFree(Color),
    Timeout(Color),
    Goal(Color),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RefereeState {
    Idle,
    PlacingBall(f32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TeamInfo {
    name: String,
    score: u8,
    goalie: u8,
    //yellow_card_times: BTreeMap<u8, f32>,
    //yellow_cards: BTreeMap<u8, u8>,
    //red_cards: BTreeMap<u8, u8>,
}

impl Default for TeamInfo {
    fn default() -> TeamInfo {
        TeamInfo {
            name: "".to_string(),
            score: 0,
            goalie: 0,
            //yellow_card_times: BTreeMap::new(),
            //yellow_cards: BTreeMap::new(),
            //red_cards: BTreeMap::new(),
        }
    }
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

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub timestamp: f32,
    pub ball: Ball,
    pub robots: BTreeMap<Id, Robot>,
    pub last_ball_touch: Option<Id>,
    pub team_side: TeamSide,
    pub referee: Referee,
    pub referee_state: RefereeState,
    pub info_yellow: TeamInfo,
    pub info_blue: TeamInfo,
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
            last_ball_touch: None,
            team_side: team_side,
            referee: Referee::Normal,
            referee_state: RefereeState::Idle,
            info_yellow: Default::default(),
            info_blue: Default::default(),
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
        let &mut State {
            ref mut ball,
            ref mut robots,
            ref mut last_ball_touch,
            ..
        } = self;
        let mut d_time_ball = timestep;
        let d_time = timestep;

        // XXX: overly simplified physics ahead
        for (robot_id, robot) in robots.iter_mut() {
            let robot_command = {
                if robot_id.color() == Color::yellow(command.is_yellow) {
                    command.robots.get(&robot_id.id())
                } else {
                    None
                }
            };

            if let Some(robot_command) = robot_command {
                let v_tangent = robot_command.v_tangent;
                let v_normal  = robot_command.v_normal;
                let v_angular = robot_command.v_angular;
                robot.vel = Vec2d(v_tangent, v_normal).rotate(-robot.w);
                robot.vw = v_angular;
            }

            // detect collision with ball
            let r = ROBOT_RADIUS + BALL_RADIUS;

            // Bhāskara:
            let a = (robot.vel - ball.vel).norm2();
            let b = 2.0 * ((robot.pos - ball.pos) * (robot.vel - ball.vel));
            let c = (robot.pos - ball.pos).norm2() - r * r;
            let delta = b * b - 4.0 * a * c;

            if delta >= 0.0 && a != 0.0 {
                let tc = (-b - delta.sqrt()) * 0.5 / a;
                if 0.0 <= tc && tc <= timestep {
                    use std::f32::consts::PI;

                    debug!("collision: ball and robot {:?}", robot_id);
                    *last_ball_touch = Some(*robot_id);

                    ball.pos += ball.vel * tc;
                    let robot_pos = robot.pos + robot.vel * tc;
                    let cw = (ball.pos - robot_pos).angle();
                    let vel = ball.vel - robot.vel;
                    let vw = vel.angle() - PI;
                    ball.vel = robot.vel + Vec2d(vel.norm(), 0.0).rotate(2.0 * cw - vw);

                    let rw = robot.w + robot.vw * tc;
                    if rw.abs_sub(cw) < ROBOT_MOUTH {
                        if let Some(robot_command) = robot_command {
                            use ::game::RobotAction::*;
                            if let Kick(force) = robot_command.action {
                                ball.vel += Vec2d(force, 0.0).rotate(rw);
                            }
                            if let ChipKick(force) = robot_command.action {
                                // TODO: z force
                                ball.vel += Vec2d(force, 0.0).rotate(rw);
                            }
                            // TODO: dribble effect
                        }
                    }

                    d_time_ball -= tc;
                }
            }

            robot.pos += robot.vel * d_time;
            robot.w   += robot.vw  * d_time;

            // TODO: effect of robot_command.action
        }

        // update ball pos
        ball.pos += ball.vel * d_time_ball ;
        ball.vel *= 1.0 - BALL_FRICT_LOSS * timestep;
    }
}
