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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Referee {
    /// All robots must stay away from the ball by at least 0.5m.
    Stop,
    /// This is the normal game play.
    Normal,
    /// After some types of kicks the kicker robot cannot touch the ball before it touches another
    /// robot, this state indicates which robot is it.  The state will change to normal when this
    /// restriction is lifted.
    Avoid(Id),
    /// All robots must go to their respective side of the field, while also staying away from the
    /// ball (by at least 0.5m). A kickoff for the team with the given color will follow soon.
    PreKickoff(Color),
    /// The team with the given color is allowed to kickoff.
    Kickoff(Color),
    /// All robots must go to the penalty positions (better described by the rules). A penalty for
    /// the team with the given color will follow.
    PrePenalty(Color),
    /// The team with the given color is allowed to kick the penalty.
    Penalty(Color),
    /// The team with the given color is allowed to kick the free direct kick.
    DirectFree(Color),
    /// The team with the given color is allowed to kick the free indirect kick.
    IndirectFree(Color),
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

pub trait TeamInfo {
    fn name<'a>(&'a self) -> &'a str { "" }
    fn score(&self) -> u8 { 0 }
    fn goalie(&self) -> u8 { 0 }
}

impl TeamInfo for () {}

/// What is needed for a game step.
pub trait State<'a> {
    type Ball: 'a + Ball;
    type Robot: 'a + Robot;
    type Robots: 'a + ExactSizeIterator<Item=Self::Robot>;
    type Geometry: 'a + Geometry;
    type TeamInfo: 'a + TeamInfo;

    fn counter(&self) -> u64;
    fn timestamp(&self) -> f64;
    fn ball(&'a self) -> Self::Ball;
    fn robot(&'a self, id: Id) -> Option<Self::Robot>;
    fn robots(&'a self) -> Self::Robots;
    fn robots_team(&'a self, color: Color) -> Self::Robots;
    fn geometry(&'a self) -> Self::Geometry;
    fn referee(&self) -> Referee { Referee::Normal }
    fn team_info(&'a self, color: Color) -> Self::TeamInfo;
    fn team_side(&self) -> TeamSide;
    fn ball_positioning(&self) -> Option<(Vec2d, Option<Color>)> { None }
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
