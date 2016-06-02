//!
//! This module is mostly concerned with the game state.
//!

use std::collections::BTreeMap;
use ::prelude::*;

pub trait Ball {
    fn pos(&self) -> Vec2d;
    fn vel(&self) -> Vec2d;
    fn z(&self) -> Option<f32> { None }
    fn vz(&self) -> Option<f32> { None }
}

pub trait Robot {
    fn id(&self) -> Id;
    fn pos(&self) -> Vec2d;
    fn vel(&self) -> Vec2d;
    fn w(&self) -> f32;
    fn vw(&self) -> f32;
}

/// Specifications of the field geometry
///
/// This specifications intend to be minimal, some fields that are available from the vision are
/// left out on purpose.
pub trait Geometry {
    fn field_length(&self) -> f32;
    fn field_width(&self) -> f32;
    fn goal_width(&self) -> f32;
    fn center_circle_radius(&self) -> f32;
    fn defense_radius(&self) -> f32;
    fn defense_stretch(&self) -> f32;
}

/// What is needed for a game step.
pub trait State<'a> {
    type Ball: 'a + Ball;
    type Robot: 'a + Robot;
    type Robots: 'a + ExactSizeIterator<Item=Self::Robot>;
    type Geometry: 'a + Geometry;

    fn counter(&self) -> u64;
    fn timestamp(&self) -> f64;
    fn ball(&'a self) -> Self::Ball;
    fn robot(&'a self, id: Id) -> Option<Self::Robot>;
    fn robots(&'a self) -> Self::Robots;
    fn robots_team(&'a self, color: Color) -> Self::Robots;
    fn geometry(&'a self) -> Self::Geometry;
}

#[derive(Clone, Debug)]
pub struct Command {
    pub color: Color,
    pub robots: BTreeMap<u8, RobotCommand>,
}

impl Command {
    pub fn new(color: Color) -> Command {
        Command {
            color: color,
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
