// XXX: temporarily allow dead code
#![allow(dead_code)]
use std::io::prelude::*;
use std::io::{Lines, BufReader, BufWriter};
use std::process::{Stdio, Command, Child, ChildStdin, ChildStdout};
use std::ffi::OsStr;
use std::sync::{Arc, Mutex};
use std::thread;
use ::prelude::*;
use ::{game, Result, Error, ErrorKind};

const MAX_REBOOTS: u8 = 10;

macro_rules! err {
    ($( $tt:tt )*) => { Error::new(ErrorKind::AiProtocol, format!($($tt)*)) }
}

macro_rules! throw_err {
    ($( $tt:tt )*) => {{ return Err(err!($($tt)*)); }}
}

pub trait CommandAiExt {
    fn from_args<S: AsRef<OsStr>>(args: &[S]) -> Command;
}

impl CommandAiExt for Command {
    fn from_args<S: AsRef<OsStr>>(program_with_args: &[S]) -> Command {
        assert!(program_with_args.len() > 0);
        let (program, args) = program_with_args.split_first().unwrap();//(program_with_args[0], &program_with_args[1..]);
        let mut command = Command::new(program);
        command.args(args);
        command
    }
}

/// Builder for a subprocess (child) AI
pub struct Builder<S: Fn() -> Command, D: 'static + Fn(&str) + Send + Sync> {
    color: Color,
    spawner: S,
    debugger: Option<D>,
}

impl<S: Fn() -> Command, D: 'static + Fn(&str) + Send + Sync> Builder<S, D> {
    pub fn new(spawner: S) -> Builder<S, D> {
        Builder {
            color: Default::default(),
            spawner: spawner,
            debugger: None,
        }
    }

    /// Whether the AI will play with the yellow team, blue otherwise.
    pub fn color(&mut self, color: Color) -> &mut Builder<S, D> {
        self.color = color;
        self
    }

    /// What to do with each line of the AI's stderr
    pub fn debugger(&mut self, debugger: D) -> &mut Builder<S, D> {
        self.debugger = Some(debugger);
        self
    }

    /// Consume the builder to spawn an AI
    pub fn build(self) -> Result<InitialState<S, D>> {
        let command = (self.spawner)();
        debug!("{:?} AI is playing with: {:?}", self.color, command);

        let mut ai_state = State {
            color: self.color,
            spawner: self.spawner,
            // unwrap is OK because of the type system
            debugger: Arc::new(Mutex::new(self.debugger.unwrap())),
            child: None,
            reboots: 0,
        };

        // this is not technically required, but it's best to error soon
        try!(ai_state.boot(false));

        Ok(InitialState {
            inner: ai_state,
            //debug: Some(BufReader::new(child_err).lines()),
        })
    }
}

struct ChildFields {
    subproc: Child,
    input: BufWriter<ChildStdin>,
    output: Lines<BufReader<ChildStdout>>,
    //error: Lines<BufReader<ChildStderr>>,
    debug: Option<thread::JoinHandle<Result<()>>>,
}

impl Drop for ChildFields {
    fn drop(&mut self) {
        let &mut ChildFields { ref mut subproc, ref mut debug, .. } = self;
        if let Err(err) = subproc.kill() {
            warn!("error killing AI subproc: {}", err);
        }
        if let Some(thread) = debug.take() {
            let _ = thread.join();
        }
    }
}

struct State<S: Fn() -> Command, D: 'static + Fn(&str) + Send + Sync> {
    color: Color,
    spawner: S,
    debugger: Arc<Mutex<D>>,
    child: Option<ChildFields>,
    reboots: u8,
}

impl<S: Fn() -> Command, D: 'static + Fn(&str) + Send + Sync> State<S, D> {
    fn boot(&mut self, reboot: bool) -> Result<()> {
        let mut command = (self.spawner)();

        let mut child = try!(command.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn());
        if reboot {
            info!("{:?} AI rebooted", self.color);
        } else {
            info!("{:?} AI booted", self.color);
        }

        // XXX: should these be unwrapped? the spawn above kinda implies it's safe to unwrap
        let child_in  = try!(child.stdin .take().ok_or(Error::new(ErrorKind::Io, "missing stdin from child")));
        let child_out = try!(child.stdout.take().ok_or(Error::new(ErrorKind::Io, "missing stdout from child")));
        let child_err = try!(child.stderr.take().ok_or(Error::new(ErrorKind::Io, "missing stderr from child")));

        let error = BufReader::new(child_err).lines();
        let debugger = self.debugger.clone();
        let color = self.color;

        let debug_thread = thread::spawn(move || {
            let debugger = try!(debugger.lock());
            for line_res in error {
                match line_res {
                    Ok(line) => (debugger)(&line),
                    Err(err) => warn!("{:?} AI I/O error: {}", color, err),
                }
            }
            Ok(())
        });

        self.child = Some(ChildFields {
            subproc: child,
            input: BufWriter::new(child_in),
            output: BufReader::new(child_out).lines(),
            //error: error,
            debug: Some(debug_thread),
        });

        Ok(())
    }

    fn reboot(&mut self) -> Result<()> {
        if let Some(ref mut child) = self.child {
            try!(child.subproc.kill());
            let _status = try!(child.subproc.wait());
        }
        try!(self.boot(true));
        self.reboots += 1;
        Ok(())
    }

    fn init<'g, G: game::State<'g>>(&mut self, state: &'g G) -> Result<()> {
        if self.child.is_none() { try!(self.boot(false)); }
        let &mut ChildFields { ref mut input, ref mut output, .. } = try!(self.child.as_mut().ok_or(err!("child not booted")));

        let version = 1;
        try!(writeln!(input, "ROBOIME_AI_PROTOCOL {}", version));

        // flush to child stdin
        try!(input.flush());

        {
            let line = try!(match output.next() {
                Some(thing) => thing,
                None => throw_err!("expected a line"),
            });
            //debug!("{}", line);
            match line.as_ref() {
                "COMPATIBLE 1" => {}
                s if s.starts_with("COMPATIBLE") => throw_err!("AI not protocol compatible (implicit)"),
                s if s.starts_with("NOT_COMPATIBLE") => throw_err!("AI not protocol compatible (explicit)"),
                s => throw_err!("AI protocol compatibility error, output was '{}'", s),
            }
        }

        let geom = state.geometry();

        // FIELD_LENGTH
        // FIELD_WIDTH
        // GOAL_WIDTH
        // CENTER_CIRCLE_RADIUS
        // DEFENSE_RADIUS
        // DEFENSE_STRETCH
        try!(writeln!(input, "{:.03} {:.03} {:.03} {:.03} {:.03} {:.03}",
            geom.field_length(),
            geom.field_width(),
            geom.goal_width(),
            geom.center_circle_radius(),
            geom.defense_radius(),
            geom.defense_stretch(),
        ));

        // flush to child stdin
        try!(input.flush());

        Ok(())
    }

    fn push<'g, G: game::State<'g>>(&mut self, state: &'g G) -> Result<(u64, Vec<u8>)> {
        if self.child.is_none() { try!(self.init(state)); }
        let &mut ChildFields { ref mut input, .. } = try!(self.child.as_mut().ok_or(err!("child not booted")));

        let timestamp = state.timestamp();
        let counter = state.counter();
        let referee = state.referee();

        let color = self.color;
        let side = state.team_side().side(color);

        let (score_player, goalie_player) = {
            let team_info = state.team_info(color);
            (team_info.score(), team_info.goalie())
        };
        let (score_opponent, goalie_opponent) = {
            let team_info = state.team_info(!color);
            (team_info.score(), team_info.goalie())
        };

        // COUNTER
        // TIMESTAMP
        // REFEREE_STATE
        // REFEREE_MORE_INFO
        // SCORE_PLAYER
        // SCORE_OPPONENT
        // GOALIE_ID_PLAYER
        // GOALIE_ID_OPPONENT
        try!(writeln!(input, "{} {} {} {} {} {} {} {}",
            counter,
            timestamp,
            referee.to_char(color),
            referee.more_info(color),
            score_player,
            score_opponent,
            goalie_player,
            goalie_opponent,
        ));

        {
            let ball = state.ball();
            let pos = side.rel_vec(ball.pos());
            let vel = side.rel_vec(ball.vel());

            // BALL_X
            // BALL_Y
            // BALL_VX
            // BALL_VY
            try!(writeln!(input, "{:.04} {:.04} {:.04} {:.04}",
                pos.x(),
                pos.y(),
                vel.x(),
                vel.y(),
            ));
        }

        let mut ids;
        {
            let robots_player = state.robots_team(color);

            // ROBOT_COUNT_PLAYER
            let len = robots_player.len();
            try!(writeln!(input, "{}", len));
            ids = Vec::with_capacity(len);

            // ROBOT_COUNT_PLAYER x
            for robot in robots_player {
                let robot_id = robot.id();
                let pos = side.rel_vec(robot.pos());
                let vel = side.rel_vec(robot.vel());
                let w = side.rel_w(robot.w());
                let vw = robot.vw();
                // ROBOT_ID
                // ROBOT_X
                // ROBOT_Y
                // ROBOT_W
                // ROBOT_VX
                // ROBOT_VY
                // ROBOT_VW
                //try!(writeln!(input, "{} {:.04} {:.04} {:.04} {:.04} {:.04} {:.04}",
                try!(writeln!(input, "{} {:.04} {:.04} {} {:.04} {:.04} {}",
                    robot_id.id(),
                    pos.x(),
                    pos.y(),
                    w,
                    vel.x(),
                    vel.y(),
                    vw,
                ));
                ids.push(robot_id.id());
            }
        }

        {
            let robots_opponent = state.robots_team(!color);

            // ROBOT_COUNT_OPPONENT
            try!(writeln!(input, "{}", robots_opponent.len()));

            // ROBOT_COUNT_OPPONENT x
            for robot in robots_opponent {
                let robot_id = robot.id();
                let pos = side.rel_vec(robot.pos());
                let vel = side.rel_vec(robot.vel());
                let w = side.rel_w(robot.w());
                let vw = robot.vw();
                // ROBOT_ID
                // ROBOT_X
                // ROBOT_Y
                // ROBOT_W
                // ROBOT_VX
                // ROBOT_VY
                // ROBOT_VW
                //try!(writeln!(input, "{} {:.04} {:.04} {:.04} {:.04} {:.04} {:.04}",
                try!(writeln!(input, "{} {:.04} {:.04} {} {:.04} {:.04} {}",
                    robot_id.id(),
                    pos.x(),
                    pos.y(),
                    w,
                    vel.x(),
                    vel.y(),
                    vw,
                ));
            }
        }

        // flush to child stdin
        try!(input.flush());

        Ok((counter, ids))
    }

    fn pull(&mut self, counter: u64, ids: Vec<u8>) -> Result<game::Command> {
        let &mut ChildFields { ref mut output, .. } = try!(self.child.as_mut().ok_or(err!("child not booted")));

        {
            let line = try!(match output.next() {
                Some(thing) => thing,
                None => throw_err!("expected a line"),
            });
            //debug!("{}", line);

            // COUNTER
            let ai_counter: u64 = try!(line.parse());
            if ai_counter != counter {
                throw_err!("wrong command counter, expected {} got {}", counter, ai_counter);
            }
        }

        let mut command = game::Command::new(self.color);
        command.robots.clear();
        {
            let mut robot_commands = &mut command.robots;

            // ROBOT_COUNT_PLAYER x
            for robot_id in ids {
                let line = try!(match output.next() {
                    Some(thing) => thing,
                    None => throw_err!("expected a line"),
                });
                //debug!("{}", line);

                let vars: Vec<_> = line.split(' ').collect();
                let vars_len = vars.len();
                if vars_len != 6 {
                    throw_err!("expected 6 values for robot command, got {}", vars_len);
                }

                // V_TANGENT
                // V_NORMAL
                // V_ANGULAR
                // KICK_FORCE
                // CHIP_FORCE
                // DRIBBLE
                let v_tangent:  f32 = try!(vars[0].parse());
                let v_normal:   f32 = try!(vars[1].parse());
                let v_angular:  f32 = try!(vars[2].parse());
                let kick_force: f32 = try!(vars[3].parse());
                let chip_force: f32 = try!(vars[4].parse());
                let dribble:   bool = try!(vars[5].parse::<i32>()) == 1;

                robot_commands.insert(robot_id, game::RobotCommand {
                    v_tangent: v_tangent,
                    v_normal: v_normal,
                    v_angular: v_angular,
                    action: if kick_force > 0.0 {
                        if chip_force > 0.0 { warn!("CHIP_FORCE shadowed by KICK_FORCE, please specify only one action"); }
                        if dribble { warn!("DRIBBLE shadowed by KICK_FORCE, please specify only one action"); }
                        game::RobotAction::Kick(kick_force)
                    } else if chip_force > 0.0 {
                        if dribble { warn!("DRIBBLE shadowed by CHIP_FORCE, please specify only one action"); }
                        game::RobotAction::ChipKick(chip_force)
                    } else if dribble {
                        game::RobotAction::Dribble
                    } else {
                        game::RobotAction::Normal
                    }
                });
            }
        }

        Ok(command)
    }
}

pub struct InitialState<S: Fn() -> Command, D: 'static + Fn(&str) + Send + Sync> {
    inner: State<S, D>,
    //pub debug: Option<Lines<BufReader<ChildStderr>>>,
}

pub struct PushState<'a, S: 'a + Fn() -> Command, D: 'a + 'static + Fn(&str) + Send + Sync> {
    inner: &'a mut State<S, D>,
}

pub struct PullState<'a, S: 'a + Fn() -> Command, D: 'a + 'static + Fn(&str) + Send + Sync> {
    inner: &'a mut State<S, D>,
    counter: u64,
    ids: Vec<u8>,
}

impl<S: Fn() -> Command, D: 'static + Fn(&str) + Send + Sync> InitialState<S, D> {
    pub fn init<'a, 'g, G: game::State<'g>>(&'a mut self, state: &'g G) -> Result<PushState<'a, S, D>> {
        let &mut InitialState { ref mut inner, .. } = self;
        try!(inner.init(state));
        Ok(PushState { inner: inner })
    }
}

impl game::Referee {
    fn to_char(self, team: Color) -> char {
        use game::Referee::*;
        match self {
            Normal => 'N',
            Avoid(Id(color, _)) if color == team => 'A',
            Avoid(..) => 'N',
            PreKickoff(color) if color == team => 'p',
            Kickoff(color) if color == team => 'k',
            PrePenalty(color) if color == team => 'x',
            Penalty(color) if color == team => 'y',
            DirectFree(color) if color == team => 'd',
            IndirectFree(color) if color == team => 'i',
            PreKickoff(color) if color != team => 'P',
            Kickoff(color) if color != team => 'K',
            PrePenalty(color) if color != team => 'X',
            Penalty(color) if color != team => 'Y',
            DirectFree(color) if color != team => 'D',
            IndirectFree(color) if color != team => 'I',
            _ => 'S',
        }
    }
    fn more_info(self, team: Color) -> i32 {
        use game::Referee::*;
        match self {
            Avoid(Id(color, id)) if color == team => id as i32,
            _ => -1,
        }
    }
}

impl Side {
    fn rel_vec(self, pos: Vec2d) -> Vec2d {
        if self.is_left() { pos } else { -pos }
    }
    fn rel_w(self, w: f32) -> f32 {
        use std::f32::consts::PI;
        if self.is_left() { w } else { (w + 2.0 * PI) % (2.0 * PI) - PI }
    }
}

impl<'a, S: 'a + Fn() -> Command, D: 'a + 'static + Fn(&str) + Send + Sync> PushState<'a, S, D> {
    pub fn update<'g, G: game::State<'g>>(&mut self, state: &'g G) -> Result<Option<game::Command>> {
        let &mut PushState { ref mut inner } = self;
        let s = PushState { inner: inner };
        let s = try!(s.push(state));
        let (_, cmd) = try!(s.pull());
        Ok(cmd)
    }

    pub fn push<'g, G: game::State<'g>>(self, state: &'g G) -> Result<PullState<'a, S, D>> {
        let PushState { mut inner } = self;
        let (counter, ids) = try!(inner.push(state));
        Ok(PullState {
            inner: inner,
            counter: counter,
            ids: ids,
        })
    }
}

impl<'a, S: 'a + Fn() -> Command, D: 'a + 'static + Fn(&str) + Send + Sync> PullState<'a, S, D> {
    pub fn pull(self) -> Result<(PushState<'a, S, D>, Option<game::Command>)> {
        let PullState { mut inner, counter, ids } = self;
        let command = try!(inner.pull(counter, ids));
        Ok((PushState { inner: inner }, Some(command)))
    }
}
