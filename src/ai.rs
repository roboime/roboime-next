use std::io::prelude::*;
use std::io::{Lines, BufReader, BufWriter};
use std::process::{Stdio, Command, Child, ChildStdin, ChildStdout, ChildStderr};
use std::ffi::OsStr;
use ::prelude::*;
use ::{game, Result, Error, ErrorKind};

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
pub struct Builder {
    command: Command,
    color: Color,
}

impl Builder {
    pub fn new(command: Command) -> Builder {
        Builder {
            command: command,
            color: Default::default(),
        }
    }

    /// Whether the AI will play with the yellow team, blue otherwise.
    pub fn color(&mut self, color: Color) -> &mut Builder {
        self.color = color;
        self
    }

    pub fn build(&mut self) -> Result<InitialState> {
        debug!("AI is playing as {:?} with: {:?}", self.color, self.command);

        let mut child = try!(self.command.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn());

        info!("AI subprocess spawned");

        // XXX: should these be unwrapped? the spawn above kinda implies it's safe to unwrap
        let child_in  = try!(child.stdin .take().ok_or_else(|| Error::new(ErrorKind::Io, "missing stdin from child")));
        let child_out = try!(child.stdout.take().ok_or_else(|| Error::new(ErrorKind::Io, "missing stdout from child")));
        let child_err = try!(child.stderr.take().ok_or_else(|| Error::new(ErrorKind::Io, "missing stderr from child")));

        Ok(InitialState {
            inner: State {
                _child: child,
                color: self.color,
                input: BufWriter::new(child_in),
                output: BufReader::new(child_out).lines(),
            },
            debug: Some(BufReader::new(child_err).lines()),
        })
    }
}

// TODO: be generic over the used interfaces instead of using BufWriter, Lines, BufReader...
struct State {
    _child: Child,
    color: Color,
    input: BufWriter<ChildStdin>,
    output: Lines<BufReader<ChildStdout>>,
}

pub struct InitialState {
    inner: State,
    pub debug: Option<Lines<BufReader<ChildStderr>>>,
}

pub struct PushState {
    inner: State,
}

pub struct PullState {
    inner: State,
    counter: u64,
    ids: Vec<u8>,
}

macro_rules! throw_err {
    ( $( $tt:tt )* ) => {{
        return Err(Error::new(ErrorKind::AiProtocol, format!($($tt)*)));
    }}
}

impl InitialState {
    pub fn init<'a, S: game::State<'a>>(self, state: &'a S) -> Result<PushState> {
        let InitialState { mut inner, .. } = self;

        let version = 1;
        try!(writeln!(inner.input, "ROBOIME_AI_PROTOCOL {}", version));

        // flush to child stdin
        try!(inner.input.flush());

        {
            let line = try!(match inner.output.next() {
                Some(thing) => thing,
                None => throw_err!("expected a line"),
            });
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
        try!(writeln!(inner.input, "{:.03} {:.03} {:.03} {:.03} {:.03} {:.03}",
            geom.field_length(),
            geom.field_width(),
            geom.goal_width(),
            geom.center_circle_radius(),
            geom.defense_radius(),
            geom.defense_stretch(),
        ));

        // flush to child stdin
        try!(inner.input.flush());

        Ok(PushState { inner: inner })
    }
}

impl PushState {
    pub fn push<'a, S: game::State<'a>>(self, state: &'a S) -> Result<PullState> {
        let PushState { mut inner } = self;

        let timestamp = state.timestamp();
        let counter = state.counter();

        let color = inner.color;

        // COUNTER
        // TIMESTAMP
        // REFEREE_STATE
        // REFEREE_TIME_LEFT
        // SCORE_PLAYER
        // SCORE_OPPONENT
        // GOALIE_ID_PLAYER
        // GOALIE_ID_OPPONENT
        try!(writeln!(inner.input, "{} {} {} {} {} {} {} {}",
            counter,
            timestamp,
            'N', // TODO: REFEREE_STATE
            -1, //  TODO: REFEREE_TIME_LEFT
            0, //   TODO: SCORE_PLAYER
            0, //   TODO: SCORE_OPPONENT
            0, //   TODO: GOALIE_ID_PLAYER
            0, //   TODO: GOALIE_ID_OPPONENT
        ));

        {
            let ball = state.ball();
            let pos = ball.pos();
            let vel = ball.vel();

            // BALL_X
            // BALL_Y
            // BALL_VX
            // BALL_VY
            try!(writeln!(inner.input, "{:.04} {:.04} {:.04} {:.04}",
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
            try!(writeln!(inner.input, "{}", len));
            ids = Vec::with_capacity(len);

            // ROBOT_COUNT_PLAYER x
            for robot in robots_player {
                let robot_id = robot.id();
                let pos = robot.pos();
                let vel = robot.vel();
                // ROBOT_ID
                // ROBOT_X
                // ROBOT_Y
                // ROBOT_W
                // ROBOT_VX
                // ROBOT_VY
                // ROBOT_VW
                try!(writeln!(inner.input, "{} {:.04} {:.04} {:.04} {:.04} {:.04} {:.04}",
                    robot_id.id(),
                    pos.x(),
                    pos.y(),
                    robot.w(),
                    vel.x(),
                    vel.y(),
                    robot.vw(),
                ));
                ids.push(robot_id.id());
            }
        }

        {
            let robots_opponent = state.robots_team(!color);

            // ROBOT_COUNT_OPPONENT
            try!(writeln!(inner.input, "{}", robots_opponent.len()));

            // ROBOT_COUNT_OPPONENT x
            for robot in robots_opponent {
                let robot_id = robot.id();
                let pos = robot.pos();
                let vel = robot.vel();
                // ROBOT_ID
                // ROBOT_X
                // ROBOT_Y
                // ROBOT_W
                // ROBOT_VX
                // ROBOT_VY
                // ROBOT_VW
                try!(writeln!(inner.input, "{} {:.04} {:.04} {:.04} {:.04} {:.04} {:.04}",
                    robot_id.id(),
                    pos.x(),
                    pos.y(),
                    robot.w(),
                    vel.x(),
                    vel.y(),
                    robot.vw(),
                ));
            }
        }

        // flush to child stdin
        try!(inner.input.flush());

        Ok(PullState {
            inner: inner,
            counter: counter,
            ids: ids,
        })
    }
}

impl PullState {
    pub fn pull(self) -> Result<(PushState, game::Command)> {
        let PullState { mut inner, counter, ids } = self;

        {
            let line = try!(match inner.output.next() {
                Some(thing) => thing,
                None => throw_err!("expected a line"),
            });
            // COUNTER
            let ai_counter: u64 = try!(line.parse());
            if ai_counter != counter {
                throw_err!("wrong command counter, expected {} got {}", counter, ai_counter);
            }
        }

        let mut command = game::Command::new(inner.color);
        command.robots.clear();
        {
            let mut robot_commands = &mut command.robots;

            // ROBOT_COUNT_PLAYER x
            for robot_id in ids {
                let line = try!(match inner.output.next() {
                    Some(thing) => thing,
                    None => throw_err!("expected a line"),
                });

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
                        game::RobotAction::Kick(kick_force)
                    } else if chip_force > 0.0 {
                        game::RobotAction::ChipKick(chip_force)
                    } else if dribble {
                        game::RobotAction::Dribble
                    } else {
                        game::RobotAction::Normal
                    }
                });
            }
        }

        Ok((PushState { inner: inner }, command))
    }
}
