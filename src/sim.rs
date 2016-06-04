//! A built-in simulator, with referee support.
//!
//! ```no_run
//! use std::process::Command;
//! use roboime_next::prelude::*;
//! use roboime_next::{ai, sim};
//!
//! let mut sim = sim::Builder::new()
//!     .initial_formation(true)
//!     .build();
//!
//! let mut ai = ai::Builder::new(Command::new("./demo-ai"))
//!     .color(Yellow)
//!     .build().unwrap();
//!
//! let mut ai = ai.init(&sim).unwrap();
//!
//! loop {
//!     let cmd = ai.update(&sim).unwrap();
//!     sim.step(cmd, 0.016_666_667);
//!     // sleep maybe
//! }
//! ```
use std::collections::BTreeMap;
use std::collections::btree_map;
use ::prelude::*;
use ::game;
use self::RefereeState::*;
use self::RefereeAction::*;
use self::BallInPlay::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Geometry {}

const FIELD_LENGTH: f32    = 9.010;
const FIELD_WIDTH: f32     = 6.010;
const FIELD_MARGIN: f32    = 0.300;
const CENTER_DIAMETER: f32 = 1.000;
const DEFENSE_RADIUS: f32  = 1.000;
const DEFENSE_STRETCH: f32 = 0.500;
const GOAL_WIDTH: f32      = 1.000;
const ROBOT_RADIUS: f32    = 0.090;
const ROBOT_MOUTH: f32     = 0.700;
const ROBOT_MAX_SPEED: f32 = 4.000;
const BALL_RADIUS: f32     = 0.023;
const BALL_FRICT_LOSS: f32 = 1.000;
const REACTION_TIME: f32   = 3.000;
const THROW_IN_MARGIN: f32 = 0.100;

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

#[derive(Clone, Debug, PartialEq)]
pub enum RefereeState {
    Idle,
    WaitingToAct(f32, RefereeAction),
    PlacingBall(f32),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RefereeAction {
    PlaceBall(Vec2d),
}

impl RefereeState {
    fn ignore_ball(&self) -> bool {
        match *self {
            PlacingBall(_) => true,
            _ => false,
        }
    }
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

impl<'t> game::TeamInfo for &'t TeamInfo {
    fn name<'a>(&'a self) -> &'a str { &self.name }
    fn score(&self) -> u8 { self.score }
    fn goalie(&self) -> u8 { self.goalie }
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

#[derive(Debug, Clone)]
pub struct Builder {
    team_side: TeamSide,
    initial_formation: bool,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            team_side: Default::default(),
            initial_formation: false,
        }
    }

    pub fn team_side(&mut self, team_side: TeamSide) -> &mut Builder {
        self.team_side = team_side;
        self
    }

    pub fn initial_formation(&mut self, initial_formation: bool) -> &mut Builder {
        self.initial_formation = initial_formation;
        self
    }

    pub fn build(&self) -> State {
        let mut robots = BTreeMap::new();
        if self.initial_formation {
            if self.team_side.yellow_is_left() {
                robots.initial_formation(6, Yellow, Left);
                robots.initial_formation(6, Blue, Right);
            } else {
                robots.initial_formation(6, Yellow, Right);
                robots.initial_formation(6, Blue, Left);
            }
        }
        State {
            initial_timestamp: 0.0, // TODO
            counter: 0,
            timestamp: 0.0,
            //ball: Ball { pos: Vec2d(0.0, 0.0), vel: Vec2d(0.0, 0.0) },
            //ball: Ball { pos: Vec2d(0.0, 0.05), vel: Vec2d(-4.0, 0.0) },
            ball: Ball { pos: Vec2d(0.0, -0.05), vel: Vec2d(-4.0, 0.0) },
            robots: robots,
            last_ball_touch: None,
            team_side: self.team_side,
            referee: game::Referee::Normal,
            referee_state: RefereeState::Idle,
            info_yellow: Default::default(),
            info_blue: Default::default(),
            geometry: Geometry {}
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub initial_timestamp: f64,
    pub counter: u64,
    pub timestamp: f32,
    pub ball: Ball,
    pub robots: BTreeMap<Id, Robot>,
    pub last_ball_touch: Option<Id>,
    pub team_side: TeamSide,
    pub referee: game::Referee,
    pub referee_state: RefereeState,
    pub info_blue: TeamInfo,
    pub info_yellow: TeamInfo,
    pub geometry: Geometry,
}

impl State {
    pub fn step(&mut self, command: game::Command, timestep: f32) {
        use std::f32::consts::PI;

        self.counter += 1;
        self.timestamp += timestep;
        let &mut State {
            ref mut ball,
            ref mut robots,
            ref mut last_ball_touch,
            ref mut referee_state,
            ref team_side,
            ref mut info_blue,
            ref mut info_yellow,
            ref mut referee,
            ..
        } = self;
        let mut d_time_ball = timestep;
        let d_time = timestep;
        let initial_ball_pos = ball.pos;

        // XXX: overly simplified physics ahead
        for (robot_id, robot) in robots.iter_mut() {
            let robot_command = {
                if robot_id.color() == command.color {
                    command.robots.get(&robot_id.id())
                } else {
                    None
                }
            };

            if let Some(robot_command) = robot_command {
                let v_tangent = robot_command.v_tangent;
                let v_normal  = robot_command.v_normal;
                let v_angular = robot_command.v_angular;
                robot.vel = Vec2d(v_tangent, v_normal).rotate(robot.w);
                robot.vw = v_angular;
            }

            // detect collision with ball
            if !referee_state.ignore_ball() {
                let r = ROBOT_RADIUS + BALL_RADIUS;

                // Bhāskara:
                let a = (robot.vel - ball.vel).norm2();
                let b = 2.0 * ((robot.pos - ball.pos) * (robot.vel - ball.vel));
                let c = (robot.pos - ball.pos).norm2() - r * r;
                let delta = b * b - 4.0 * a * c;

                if delta >= 0.0 && a != 0.0 {
                    let tc = (-b - delta.sqrt()) * 0.5 / a;
                    if 0.0 <= tc && tc <= timestep {

                        trace!("collision: ball and robot {:?}", robot_id);
                        *last_ball_touch = Some(*robot_id);

                        ball.pos += ball.vel * tc;
                        let robot_pos = robot.pos + robot.vel * tc;
                        let cw = (ball.pos - robot_pos).angle();
                        let vel = ball.vel - robot.vel;
                        let vw = vel.angle() - PI;
                        ball.vel = robot.vel + Vec2d(vel.norm(), 0.0).rotate(2.0 * cw - vw);

                        fn angle_abs_sub(a: f32, b: f32) -> f32 {
                            ((a - b + PI) % (2.0 * PI) - PI).abs()
                        }

                        let rw = robot.w + robot.vw * tc;
                        if angle_abs_sub(rw, cw) < ROBOT_MOUTH / 2.0 {
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
            }

            const ROBOT_MAX_SPEED2: f32 = ROBOT_MAX_SPEED * ROBOT_MAX_SPEED;
            if robot.vel.norm2() > ROBOT_MAX_SPEED2 {
                robot.vel = robot.vel * ROBOT_MAX_SPEED / robot.vel.norm();
            }
            robot.pos += robot.vel * d_time;
            robot.w   += robot.vw  * d_time;
            robot.w   %= 2.0 * PI;

            // limit robot to inside the robot area:
            const MAX_X: f32 = FIELD_LENGTH * 0.5 + FIELD_MARGIN - ROBOT_RADIUS;
            const MAX_Y: f32 = FIELD_WIDTH  * 0.5 + FIELD_MARGIN - ROBOT_RADIUS;
            if robot.pos.x() >  MAX_X { robot.pos = Vec2d( MAX_X, robot.pos.y()); }
            if robot.pos.x() < -MAX_X { robot.pos = Vec2d(-MAX_X, robot.pos.y()); }
            if robot.pos.y() >  MAX_Y { robot.pos = Vec2d(robot.pos.x(),  MAX_Y); }
            if robot.pos.y() < -MAX_Y { robot.pos = Vec2d(robot.pos.x(), -MAX_Y); }
        }

        // update ball pos
        ball.pos += ball.vel * d_time_ball ;
        ball.vel *= 1.0 - BALL_FRICT_LOSS * timestep;

        // referee checks
        let mut next_referee_state = None;
        match *referee_state {
            Idle => {
                match ball_in_play(initial_ball_pos, ball.pos) {
                    MaybeGoal(side) => {
                        let scoring_team = !team_side.color(side);
                        match scoring_team {
                            Blue => { info_blue.score += 1; }
                            Yellow => { info_yellow.score += 1; }
                        }
                        let score_blue = info_blue.score;
                        let score_yellow = info_yellow.score;
                        debug!("Goal! {:?} team scores!! (B: {}, Y: {})", scoring_team, score_blue, score_yellow);
                        *referee = game::Referee::Goal(scoring_team);
                        next_referee_state = Some(WaitingToAct(REACTION_TIME, PlaceBall(Vec2d(0.0, 0.0))));
                    }
                    GoalLine(place, side) => {
                        // NOTE: last_ball_touch may be None!!
                        let team = last_ball_touch.map(|id| !id.color());
                        if team == Some(team_side.color(side)) {
                            debug!("Out! Goal kick {:?}", team);
                        } else {
                            debug!("Out! Corner kick {:?}", team);
                        }
                        *referee = game::Referee::Stop;
                        next_referee_state = Some(WaitingToAct(REACTION_TIME, PlaceBall(place)));
                    }
                    ThrowIn(place) => {
                        let team = last_ball_touch.map(|id| !id.color());
                        debug!("Out! Throw in {:?}", team);
                        *referee = game::Referee::Stop;
                        next_referee_state = Some(WaitingToAct(REACTION_TIME, PlaceBall(place)));
                    }
                    InPlay => (),
                }
            },
            WaitingToAct(ref mut time_left, ref action) => {
                if *time_left <= 0.0 {
                    match *action {
                        PlaceBall(place) => {
                            ball.pos = place;
                            ball.vel = Vec2d(0.0, 0.0);
                            next_referee_state = Some(PlacingBall(REACTION_TIME));
                        }
                    }
                } else {
                    *time_left -= timestep;
                }
            }
            PlacingBall(ref mut time_left) => {
                if *time_left <= 0.0 {
                    next_referee_state = Some(Idle);
                    *referee = game::Referee::Normal;
                    debug!("Go!");
                } else {
                    *time_left -= timestep;
                }
            }
        }

        if let Some(state) = next_referee_state {
            *referee_state = state;
        }
    }
}

impl<'a> game::State<'a> for State {
    type Ball = &'a Ball;
    type Robot = (Id, &'a Robot);
    type Robots = Iter<'a>;
    type Geometry = &'a Geometry;
    type TeamInfo = &'a TeamInfo;

    fn counter(&self) -> u64 {
        self.counter
    }

    fn timestamp(&self) -> f64 {
        self.initial_timestamp + self.timestamp as f64
    }

    fn ball(&'a self) -> Self::Ball {
        &self.ball
    }

    fn robot(&'a self, id: Id) -> Option<Self::Robot> {
        self.robots.get(&id).map(|r| (id, r))
    }

    fn robots(&'a self) -> Self::Robots {
        Iter {
            filter: None,
            robots: self.robots.iter()
        }
    }

    fn robots_team(&'a self, color: Color) -> Self::Robots {
        Iter {
            filter: Some(color),
            robots: self.robots.iter()
        }
    }

    fn geometry(&'a self) -> Self::Geometry {
        &self.geometry
    }

    fn referee(&self) -> game::Referee {
        self.referee
    }

    fn team_info(&'a self, color: Color) -> Self::TeamInfo {
        match color {
            Yellow => &self.info_yellow,
            Blue => &self.info_blue,
        }
    }
}

pub struct Iter<'a> {
    filter: Option<Color>,
    robots: btree_map::Iter<'a, Id, Robot>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (Id, &'a Robot);

    fn next(&mut self) -> Option<Self::Item> {
        (match self.filter {
            None => self.robots.next(),
            Some(color) => self.robots.find(|&(&id, _)| id.color() == color),
        }).map(|(id, robot)| (*id, robot))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = match self.filter {
            None => self.robots.len(),
            Some(color) => self.robots.clone().filter(|&(&id, _)| id.color() == color).count(),
        };
        (len, Some(len))
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}

impl<'a> game::Ball for &'a Ball {
    fn pos(&self) -> Vec2d { self.pos }
    fn vel(&self) -> Vec2d { self.vel }
}

impl<'a> game::Robot for (Id, &'a Robot) {
    fn id(&self) -> Id { self.0 }
    fn pos(&self) -> Vec2d { self.1.pos }
    fn vel(&self) -> Vec2d { self.1.vel }
    fn w(&self) -> f32 { self.1.w }
    fn vw(&self) -> f32 { self.1.vw }
}

impl<'a> game::Geometry for &'a Geometry {
    fn field_length(&self) -> f32 { FIELD_LENGTH }
    fn field_width(&self) -> f32 { FIELD_WIDTH }
    fn goal_width(&self) -> f32 { GOAL_WIDTH }
    fn center_circle_radius(&self) -> f32 { CENTER_DIAMETER / 2.0 }
    fn defense_radius(&self) -> f32 { DEFENSE_RADIUS }
    fn defense_stretch(&self) -> f32 { DEFENSE_STRETCH }
}

fn line_cross_x(p0: Vec2d, p1: Vec2d, x: f32) -> Option<Vec2d> {
    if p0.x() <= x && x < p1.x() || p0.x() > x && x >= p1.x() {
        Some(p0 + (p1 - p0) * (x - p0.x()) / (p1.x() - p0.x()))
    } else {
        None
    }
}

#[test]
fn test_line_cross_x() {
    assert_eq!(line_cross_x(Vec2d( 0.0,  0.0), Vec2d( 2.0, 0.0),  1.0), Some(Vec2d( 1.0, 0.0)));
    assert_eq!(line_cross_x(Vec2d(-1.0,  0.0), Vec2d( 2.0, 0.0),  1.0), Some(Vec2d( 1.0, 0.0)));
    assert_eq!(line_cross_x(Vec2d( 0.0,  0.0), Vec2d( 1.0, 0.0),  2.0), None);
    assert_eq!(line_cross_x(Vec2d(-1.0, -0.5), Vec2d( 2.0, 1.0),  1.0), Some(Vec2d( 1.0, 0.5)));
    assert_eq!(line_cross_x(Vec2d(-0.0,  0.0), Vec2d(-2.0, 0.0), -1.0), Some(Vec2d(-1.0, 0.0)));
    assert_eq!(line_cross_x(Vec2d( 1.0,  0.0), Vec2d(-2.0, 0.0), -1.0), Some(Vec2d(-1.0, 0.0)));
    assert_eq!(line_cross_x(Vec2d( 0.0,  0.0), Vec2d(-1.0, 0.0), -2.0), None);
    assert_eq!(line_cross_x(Vec2d( 1.0, -0.5), Vec2d(-2.0, 1.0), -1.0), Some(Vec2d(-1.0, 0.5)));
}

fn line_cross_y(p0: Vec2d, p1: Vec2d, y: f32) -> Option<Vec2d> {
    if p0.y() <= y && y < p1.y() || p0.y() > y && y >= p1.y() {
        Some(p0 + (p1 - p0) * (y - p0.y()) / (p1.y() - p0.y()))
    } else {
        None
    }
}

#[test]
fn test_line_cross_y() {
    assert_eq!(line_cross_y(Vec2d( 0.0,  0.0), Vec2d(0.0,  2.0),  1.0), Some(Vec2d(0.0,  1.0)));
    assert_eq!(line_cross_y(Vec2d( 0.0, -1.0), Vec2d(0.0,  2.0),  1.0), Some(Vec2d(0.0,  1.0)));
    assert_eq!(line_cross_y(Vec2d( 0.0,  0.0), Vec2d(0.0,  1.0),  2.0), None);
    assert_eq!(line_cross_y(Vec2d(-0.5, -1.0), Vec2d(1.0,  2.0),  1.0), Some(Vec2d(0.5,  1.0)));
    assert_eq!(line_cross_y(Vec2d( 0.0, -0.0), Vec2d(0.0, -2.0), -1.0), Some(Vec2d(0.0, -1.0)));
    assert_eq!(line_cross_y(Vec2d( 0.0,  1.0), Vec2d(0.0, -2.0), -1.0), Some(Vec2d(0.0, -1.0)));
    assert_eq!(line_cross_y(Vec2d( 0.0,  0.0), Vec2d(0.0, -1.0), -2.0), None);
    assert_eq!(line_cross_y(Vec2d(-0.5,  1.0), Vec2d(1.0, -2.0), -1.0), Some(Vec2d(0.5, -1.0)));
}

enum BallInPlay {
    InPlay,
    ThrowIn(Vec2d),
    GoalLine(Vec2d, Side),
    MaybeGoal(Side),
}

// XXX: the rules define a ball out of play when the "whole" ball crosses the touch
// line, from our understanding that means when the top view of the ball does not
// touch the line anymore, and it's position is outside the field.
fn ball_in_play(p0: Vec2d, p1: Vec2d) -> BallInPlay {
    if let Some(point) = line_cross_x(p0, p1, -FIELD_LENGTH * 0.5 - BALL_RADIUS) {
        if point.y().abs() <= GOAL_WIDTH * 0.5 - BALL_RADIUS {
            return MaybeGoal(Left);
        } else if point.y().abs() <= FIELD_WIDTH * 0.5 + BALL_RADIUS {
            return GoalLine(point + Vec2d(BALL_RADIUS * 2.0 + THROW_IN_MARGIN, 0.0), Left);
        }
    }
    if let Some(point) = line_cross_x(p0, p1, FIELD_LENGTH * 0.5 + BALL_RADIUS) {
        if point.y().abs() <= GOAL_WIDTH * 0.5 - BALL_RADIUS {
            return MaybeGoal(Right);
        } else if point.y().abs() <= FIELD_WIDTH * 0.5 + BALL_RADIUS {
            return GoalLine(point - Vec2d(BALL_RADIUS * 2.0 + THROW_IN_MARGIN, 0.0), Right);
        }
    }
    if let Some(point) = line_cross_y(p0, p1, FIELD_WIDTH * 0.5 + BALL_RADIUS) {
        ThrowIn(point - Vec2d(0.0, BALL_RADIUS * 2.0 + THROW_IN_MARGIN))
    } else if let Some(point) = line_cross_y(p0, p1, -FIELD_WIDTH * 0.5 - BALL_RADIUS) {
        ThrowIn(point + Vec2d(0.0, BALL_RADIUS * 2.0 + THROW_IN_MARGIN))
    } else {
        InPlay
    }
}
