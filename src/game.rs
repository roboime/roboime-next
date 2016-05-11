//!
//! This module is mostly concerned with the game state.
//!

use std::ops::{Deref, DerefMut};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, RwLockReadGuard, Condvar, Mutex};
use ::Result;

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

#[derive(Debug, Clone)]
struct Pos {
    x: f32,
    y: f32,
}

/// Carries mainly x, y, vx, vy
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct FieldGeom {
    pub field_length: f32,
    pub field_width: f32,
    pub goal_width: f32,
    pub center_circle_radius: f32,
    pub defense_radius: f32,
    pub defense_stretch: f32,
    pub free_kick_from_defense_dist: f32,
    pub penalty_spot_from_field_line_dist: f32,
    pub penalty_line_from_spot_dist: f32,
}

/// Carries everything needed for a game step.
#[derive(Debug, Clone)]
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
                field_length: 0.0,
                field_width: 0.0,
                goal_width: 0.0,
                center_circle_radius: 0.0,
                defense_radius: 0.0,
                defense_stretch: 0.0,
                free_kick_from_defense_dist: 0.0,
                penalty_spot_from_field_line_dist: 0.0,
                penalty_line_from_spot_dist: 0.0,
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

struct InnerSharedState {
    state: RwLock<State>,
    condvar: Condvar,
    mutex: Mutex<()>,
}

/// A type for sharing a `State`, commiting, signaling and waiting for changes.
// TODO: explain more, explain better
#[derive(Clone)]
pub struct SharedState {
    inner: Arc<InnerSharedState>,
}

impl SharedState {
    /// Initialize and return a `SharedState`.
    pub fn new() -> SharedState {
        SharedState {
            inner: Arc::new(InnerSharedState {
                state: RwLock::new(State::new()),
                condvar: Condvar::new(),
                mutex: Mutex::new(()),
            })
        }
    }

    /// Gives read access, only committed changes are visible
    pub fn read(&self) -> Result<RwLockReadGuard<State>> {
        Ok(try!(self.inner.state.read()))
    }

    /// Gives write access, will be commited when the returned `AutoCommitState` is dropped
    pub fn write(&self) -> Result<AutoCommitState> {
        Ok(AutoCommitState {
            shared_game_state: self.clone(),
            state: try!(self.inner.state.read()).clone(),
        })
    }

    fn notify(&self) {
        self.inner.condvar.notify_all();
    }

    /// Blocks the current thread until there's a commit
    pub fn wait(&self) -> Result<()> {
        let mutex = try!(self.inner.mutex.lock());
        // XXX: should this be used?
        let _ = try!(self.inner.condvar.wait(mutex));
        Ok(())
    }

    /// Shortcut for `.wait()` followed by `.read()`
    pub fn wait_and_read(&self) -> Result<RwLockReadGuard<State>> {
        self.wait().and(self.read())
    }
}

/// Helper type that commits changes when dropped.
#[derive(Clone)]
pub struct AutoCommitState {
    shared_game_state: SharedState,
    state: State,
}

impl Drop for AutoCommitState {
    fn drop(&mut self) {
        let mut old_state = self.shared_game_state.inner.state.write().unwrap();
        old_state.clone_from(&self.state);
        self.shared_game_state.notify();
    }
}

impl Deref for AutoCommitState {
    type Target = State;

    fn deref(&self) -> &State { &self.state }
}

impl DerefMut for AutoCommitState {
    fn deref_mut(&mut self) -> &mut State { &mut self.state }
}
