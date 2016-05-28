//!
//! This module is mostly concerned with the game state.
//!

use std::collections::BTreeMap;

pub trait Position {
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_vx(&self) -> f32;
    fn get_vy(&self) -> f32;
    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);
    fn set_vx(&mut self, vx: f32);
    fn set_vy(&mut self, vy: f32);
    fn update_position(&mut self, x: f32, y: f32, dt: f64) {
        let px = self.get_x();
        let py = self.get_y();
        let vx = (((x - px) as f64) / dt) as f32;
        let vy = (((y - py) as f64) / dt) as f32;
        self.set_x(x);
        self.set_y(y);
        self.set_vx(vx);
        self.set_vy(vy);
    }
}

pub trait Pose : Position {
    fn get_w(&self) -> f32;
    fn get_vw(&self) -> f32;
    fn set_w(&mut self, w: f32);
    fn set_vw(&mut self, vw: f32);
    fn update_pose(&mut self, x: f32, y: f32, w: f32, dt: f64) {
        let px = self.get_x();
        let py = self.get_y();
        let pw = self.get_w();
        let vx = (((x - px) as f64) / dt) as f32;
        let vy = (((y - py) as f64) / dt) as f32;
        let vw = (((w - pw) as f64) / dt) as f32;
        self.set_x(x);
        self.set_y(y);
        self.set_w(w);
        self.set_vx(vx);
        self.set_vy(vy);
        self.set_vw(vw);
    }
}

#[derive(Clone, Debug)]
struct Pos {
    x: f32,
    y: f32,
}

/// Carries mainly x, y, vx, vy
#[derive(Clone, Debug)]
pub struct Ball {
    p: Pos,
    v: Pos,
}

impl Position for Ball {
    fn get_x(&self) -> f32 { self.p.x }
    fn get_y(&self) -> f32 { self.p.y }
    fn get_vx(&self) -> f32 { self.v.x }
    fn get_vy(&self) -> f32 { self.v.y }
    fn set_x(&mut self, x: f32) { self.p.x = x; }
    fn set_y(&mut self, y: f32) { self.p.y = y; }
    fn set_vx(&mut self, vx: f32) { self.v.x = vx; }
    fn set_vy(&mut self, vy: f32) { self.v.y = vy; }
}

/// Carries mainly x, y, w, vx, vy, vw
#[derive(Clone, Debug)]
pub struct Robot {
    i: u8,
    p: Pos,
    v: Pos,
    w: f32,
    vw: f32,
}

impl Robot {
    pub fn new(id: u8) -> Robot {
        Robot {
            i: id,
            p: Pos { x: 0.0, y: 0.0 },
            v: Pos { x: 0.0, y: 0.0 },
            w: 0.0,
            vw: 0.0,
        }
    }
    pub fn get_id(&self) -> u8 { self.i }
}

impl Position for Robot {
    fn get_x(&self) -> f32 { self.p.x }
    fn get_y(&self) -> f32 { self.p.y }
    fn get_vx(&self) -> f32 { self.v.x }
    fn get_vy(&self) -> f32 { self.v.y }
    fn set_x(&mut self, x: f32) { self.p.x = x; }
    fn set_y(&mut self, y: f32) { self.p.y = y; }
    fn set_vx(&mut self, vx: f32) { self.v.x = vx; }
    fn set_vy(&mut self, vy: f32) { self.v.y = vy; }
}

impl Pose for Robot {
    fn get_w(&self) -> f32 { self.w }
    fn get_vw(&self) -> f32 { self.vw }
    fn set_w(&mut self, w: f32) { self.w = w; }
    fn set_vw(&mut self, vw: f32) { self.vw = vw; }
}

/// Specifications of the field geometry
///
/// This specifications intend to be minimal, some fields that we have from the vision are left
/// out on purpose.
///
#[derive(Clone, Debug)]
pub struct FieldGeom {
    pub field_length: f32,
    pub field_width: f32,
    pub goal_width: f32,
    pub center_circle_radius: f32,
    pub defense_radius: f32,
    pub defense_stretch: f32,
    //pub free_kick_from_defense_dist: f32,
    pub penalty_spot_from_field_line_dist: f32,
    pub penalty_line_from_spot_dist: f32,
}

/// Carries everything needed for a game step.
#[derive(Clone, Debug)]
pub struct State {
    counter: u64,
    timestamp: f64,
    ball: Ball,
    robots_blue: BTreeMap<u8, Robot>,
    robots_yellow: BTreeMap<u8, Robot>,
    field_geom: FieldGeom,
}

impl State {
    pub fn new() -> State {
        State {
            counter: 0,
            timestamp: 0.0,
            ball: Ball {
                p: Pos { x: 0.0, y: 0.0 },
                v: Pos { x: 0.0, y: 0.0 },
            },
            robots_blue: BTreeMap::new(),
            robots_yellow: BTreeMap::new(),
            field_geom: FieldGeom {
                field_length: 9.0,
                field_width: 6.0,
                goal_width: 1.0,
                center_circle_radius: 1.0,
                defense_radius: 1.0,
                defense_stretch: 0.5,
                //free_kick_from_defense_dist: 0.7,
                penalty_spot_from_field_line_dist: 1.0,
                penalty_line_from_spot_dist: 0.4,
            },
        }
    }
    pub fn get_counter(&self) -> u64 { self.counter }
    pub fn inc_counter(&mut self) { self.counter += 1; }
    pub fn get_timestamp(&self) -> f64 { self.timestamp }
    pub fn set_timestamp(&mut self, t: f64) { self.timestamp = t; }
    pub fn get_ball(&self) -> &Ball { &self.ball }
    pub fn get_ball_mut(&mut self) -> &mut Ball { &mut self.ball }
    pub fn get_robots_blue(&self) -> &BTreeMap<u8, Robot> { &self.robots_blue }
    pub fn get_robots_yellow(&self) -> &BTreeMap<u8, Robot> { &self.robots_yellow }
    pub fn get_robots_blue_mut(&mut self) -> &mut BTreeMap<u8, Robot> { &mut self.robots_blue }
    pub fn get_robots_yellow_mut(&mut self) -> &mut BTreeMap<u8, Robot> { &mut self.robots_yellow }
    pub fn get_field_geom(&self) -> &FieldGeom { &self.field_geom }
    pub fn get_field_geom_mut(&mut self) -> &mut FieldGeom { &mut self.field_geom }
}

#[derive(Clone, Debug)]
pub struct Command {
    pub is_yellow: bool,
    pub robots: BTreeMap<u8, RobotCommand>,
}

impl Command {
    pub fn new(is_yellow: bool) -> Command {
        Command {
            is_yellow: is_yellow,
            robots: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RobotCommand {
    pub v_tangent: f32,
    pub v_normal: f32,
    pub v_angular: f32,
    pub action: RobotAction,
}

#[derive(Clone, Debug)]
pub enum RobotAction {
    Normal,
    Dribble,
    Kick(f32),
    ChipKick(f32),
}
