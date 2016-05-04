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
pub struct BallState {
    p: Pos,
    v: Pos,
}

impl Position for BallState {
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
pub struct RobotState {
    i: u8,
    p: Pos,
    v: Pos,
    w: f32,
    vw: f32,
}

impl RobotState {
    pub fn new(id: u8) -> RobotState {
        RobotState {
            i: id,
            p: Pos { x: 0.0, y: 0.0 },
            v: Pos { x: 0.0, y: 0.0 },
            w: 0.0,
            vw: 0.0,
        }
    }
    pub fn get_id(&self) -> u8 { self.i }
}

impl Position for RobotState {
    fn get_x(&self) -> f32 { self.p.x }
    fn get_y(&self) -> f32 { self.p.y }
    fn get_vx(&self) -> f32 { self.v.x }
    fn get_vy(&self) -> f32 { self.v.y }
    fn set_x(&mut self, x: f32) { self.p.x = x; }
    fn set_y(&mut self, y: f32) { self.p.y = y; }
    fn set_vx(&mut self, vx: f32) { self.v.x = vx; }
    fn set_vy(&mut self, vy: f32) { self.v.y = vy; }
}

impl Pose for RobotState {
    fn get_w(&self) -> f32 { self.w }
    fn get_vw(&self) -> f32 { self.vw }
    fn set_w(&mut self, w: f32) { self.w = w; }
    fn set_vw(&mut self, vw: f32) { self.vw = vw; }
}

//#[derive(Debug, Clone)]
//pub struct FieldSpecs {
//}

/// Carries everything needed for a game step.
#[derive(Debug, Clone)]
pub struct GameState {
    counter: u64,
    timestamp: f64,
    ball: BallState,
    robots_blue: BTreeMap<u8, RobotState>,
    robots_yellow: BTreeMap<u8, RobotState>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            counter: 0,
            timestamp: 0.0,
            ball: BallState {
                p: Pos { x: 0.0, y: 0.0 },
                v: Pos { x: 0.0, y: 0.0 },
            },
            robots_blue: BTreeMap::new(),
            robots_yellow: BTreeMap::new(),
        }
    }
    pub fn get_counter(&self) -> u64 { self.counter }
    pub fn inc_counter(&mut self) { self.counter += 1; }
    pub fn get_timestamp(&self) -> f64 { self.timestamp }
    pub fn set_timestamp(&mut self, t: f64) { self.timestamp = t; }
    pub fn get_ball(&self) -> &BallState { &self.ball }
    pub fn get_ball_mut(&mut self) -> &mut BallState { &mut self.ball }
    pub fn get_robots_blue(&self) -> &BTreeMap<u8, RobotState> { &self.robots_blue }
    pub fn get_robots_yellow(&self) -> &BTreeMap<u8, RobotState> { &self.robots_yellow }
    pub fn get_robots_blue_mut(&mut self) -> &mut BTreeMap<u8, RobotState> { &mut self.robots_blue }
    pub fn get_robots_yellow_mut(&mut self) -> &mut BTreeMap<u8, RobotState> { &mut self.robots_yellow }
}

struct InnerSharedGameState {
    state: RwLock<GameState>,
    condvar: Condvar,
    mutex: Mutex<()>,
}

/// A type for sharing a `GameState`, commiting, signaling and waiting for changes.
// TODO: explain more, explain better
#[derive(Clone)]
pub struct SharedGameState {
    inner: Arc<InnerSharedGameState>,
}

impl SharedGameState {
    /// Initialize and return a `SharedGameState`.
    pub fn new() -> SharedGameState {
        SharedGameState {
            inner: Arc::new(InnerSharedGameState {
                state: RwLock::new(GameState::new()),
                condvar: Condvar::new(),
                mutex: Mutex::new(()),
            })
        }
    }

    /// Gives read access, only committed changes are visible
    pub fn read(&self) -> Result<RwLockReadGuard<GameState>> {
        Ok(try!(self.inner.state.read()))
    }

    /// Gives write access, will be commited when the returned `AutoCommitGameState` is dropped
    pub fn write(&self) -> Result<AutoCommitGameState> {
        Ok(AutoCommitGameState {
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
}

/// Helper type that commits changes when dropped.
#[derive(Clone)]
pub struct AutoCommitGameState {
    shared_game_state: SharedGameState,
    state: GameState,
}

impl Drop for AutoCommitGameState {
    fn drop(&mut self) {
        let mut old_state = self.shared_game_state.inner.state.write().unwrap();
        old_state.clone_from(&self.state);
        self.shared_game_state.notify();
    }
}

impl Deref for AutoCommitGameState {
    type Target = GameState;

    fn deref(&self) -> &GameState { &self.state }
}

impl DerefMut for AutoCommitGameState {
    fn deref_mut(&mut self) -> &mut GameState { &mut self.state }
}
