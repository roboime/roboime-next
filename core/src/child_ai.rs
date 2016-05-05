use std::process::{Stdio, Command, Child};
use std::sync::mpsc::Sender;
use std::thread::{Builder, JoinHandle};
use ::{Result, Error, ErrorKind, SharedGameState, Position, Pose, InterfaceHandle};

/// Builder for a subprocess (child) AI
pub struct ChildAi {
    command: Command,
    is_yellow: bool,
}

impl ChildAi {
    pub fn new(command: Command) -> ChildAi {
        ChildAi {
            command: command,
            is_yellow: true,
        }
    }

    /// Whether the AI will play with the yellow team, blue otherwise.
    pub fn is_yellow(&mut self, is_yellow: bool) -> &mut ChildAi {
        self.is_yellow = is_yellow;
        self
    }

    /// Spawn a thread which spawns a child subprocess and feeds it the game_state at 60Hz and
    /// sends commands through tx.
    pub fn spawn(&mut self, game_state: SharedGameState, tx: Sender<Vec<u8>>) -> Result<ChildAiHandle> {
        use std::io::prelude::*;
        use std::io::{BufReader, BufWriter};
        use protocol::Message;
        use protocol::grSim_Packet::grSim_Packet;
        use protocol::grSim_Commands::grSim_Robot_Command;

        let is_yellow = self.is_yellow;
        let mut child = try!(self.command.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn());

        println!("child AI started");

        let child_in = try!(child.stdin.take().ok_or_else(|| Error::new(ErrorKind::Io, "missing stdin from child")));
        let child_out = try!(child.stdout.take().ok_or_else(|| Error::new(ErrorKind::Io, "missing stdout from child")));
        let child_err = try!(child.stderr.take().ok_or_else(|| Error::new(ErrorKind::Io, "missing stderr from child")));

        let thread_builder = Builder::new().name("child AI Debug".to_string());
        let debug_thread = try!(thread_builder.spawn(move || -> Result<()> {
            for line in BufReader::new(child_err).lines() {
                println!("debug: {}", try!(line));
            }
            Ok(())
        }));

        let thread_builder = Builder::new().name("child AI Game I/O".to_string());
        let child_thread = try!(thread_builder.spawn(move || -> Result<()> {

            macro_rules! throw_err {
                ( $( $tt:tt )* ) => {{
                    return Err(Error::new(ErrorKind::AiProtocol, format!($($tt)*)));
                }}
            }

            // TODO: optimize by using `with_capacity` after calculating a low but enough estimate
            let mut output = BufReader::new(child_out).lines();
            let mut input = BufWriter::new(child_in);

            try!(writeln!(input, "ROBOIME_INTEL_PROTOCOL_VERSION 1"));

            // flush to child stdin
            try!(input.flush());

            {
                let line = try!(match output.next() {
                    Some(thing) => thing,
                    None => throw_err!("expected a line"),
                });
                match line.as_ref() {
                    "COMPATIBLE 1" => {}
                    s if s.starts_with("COMPATIBLE") => throw_err!("child protocol not compatible (implicit)"),
                    s if s.starts_with("NOT_COMPATIBLE") => throw_err!("child protocol not compatible (explicit)"),
                    _ => throw_err!("child compatibility negotiation failed"),
                }
            }

            {
                let state = try!(game_state.wait_and_read());
                let geom = state.get_field_geom();
                // FIELD_LENGTH FIELD_WIDTH GOAL_WIDTH CENTER_CIRCLE_RADIUS DEFENSE_RADIUS DEFENSE_STRETCH FREE_KICK_FROM_DEFENSE_DIST
                // PENALTY_SPOT_FROM_FIELD_LINE_DIST PENALTY_LINE_FROM_SPOT_DIST
                try!(writeln!(input, "{:.03} {:.03} {:.03} {:.03} {:.03} {:.03} {:.03} {:.03} {:.03}",
                    geom.field_length,
                    geom.field_width,
                    geom.goal_width,
                    geom.center_circle_radius,
                    geom.defense_radius,
                    geom.defense_stretch,
                    geom.free_kick_from_defense_dist,
                    geom.penalty_spot_from_field_line_dist,
                    geom.penalty_line_from_spot_dist,
                ));
            }

            // flush to child stdin
            try!(input.flush());

            loop {
                let state = try!(game_state.wait_and_read());
                let timestamp = state.get_timestamp();
                let counter = state.get_counter();

                let (robots_our, robots_opponent) = {
                    let robots_yellow = state.get_robots_yellow();
                    let robots_blue = state.get_robots_blue();
                    if is_yellow {
                        (robots_yellow, robots_blue)
                    } else {
                        (robots_blue, robots_yellow)
                    }
                };

                // COUNTER TIMESTAMP OUR_SCORE OPPONENT_SCORE REF_STATE <REF_TIME_LEFT|-1> GOALKEEPER_ID NUM_ROBOTS OPPONENT_NUM_ROBOTS
                try!(writeln!(input, "{} {} {} {} {} {} {} {} {}",
                    counter,
                    timestamp,
                    0, // TODO: OUR_SCORE
                    0, // TODO: OPPONENT_SCORE
                    'N', // TODO: REFEREE_STATE
                    -1, // TODO: REFEREE_TIME_LEFT
                    0, // TODO: GOALKEEPER_ID
                    robots_our.len(),
                    robots_opponent.len(),
                ));

                let ball = state.get_ball();

                // BALL_X BALL_Y BALL_VX BALL_VY
                try!(writeln!(input, "{:.04} {:.04} {:.04} {:.04}",
                    ball.get_x(),
                    ball.get_y(),
                    ball.get_vx(),
                    ball.get_vy(),
                ));

                for (robot_id, robot) in robots_our {
                    // [ROBOT_ID ROBOT_X ROBOT_Y ROBOT_W ROBOT_VX ROBOT_VY ROBOT_VW] x NUM_ROBOTS
                    try!(writeln!(input, "{} {:.04} {:.04} {:.04} {:.04} {:.04} {:.04}",
                        robot_id,
                        robot.get_x(),
                        robot.get_y(),
                        robot.get_w(),
                        robot.get_vx(),
                        robot.get_vy(),
                        robot.get_vw(),
                    ));
                }

                for (robot_id, robot) in robots_opponent {
                    // [ROBOT_ID ROBOT_X ROBOT_Y ROBOT_W ROBOT_VX ROBOT_VY ROBOT_VW] x OPPONENT_NUM_ROBOTS
                    try!(writeln!(input, "{} {:.04} {:.04} {:.04} {:.04} {:.04} {:.04}",
                        robot_id,
                        robot.get_x(),
                        robot.get_y(),
                        robot.get_w(),
                        robot.get_vx(),
                        robot.get_vy(),
                        robot.get_vw(),
                     ));
                }

                // flush to child stdin
                try!(input.flush());

                {
                    let line = try!(match output.next() {
                        Some(thing) => thing,
                        None => throw_err!("expected a line"),
                    });
                    let ai_counter: u64 = try!(line.parse());
                    if ai_counter != counter {
                        throw_err!("wrong command counter, expected {} got {}", counter, ai_counter);
                    }
                }

                let mut packet = grSim_Packet::new();
                {
                    let commands = packet.mut_commands();
                    commands.set_timestamp(timestamp);
                    commands.set_isteamyellow(is_yellow);
                    let robot_commands = commands.mut_robot_commands();

                    for (robot_id, _) in robots_our {
                        let mut robot_command = grSim_Robot_Command::new();

                        let line = try!(match output.next() {
                            Some(thing) => thing,
                            None => throw_err!("expected a line"),
                        });

                        let vars: Vec<_> = line.split(' ').collect();
                        let vars_len = vars.len();
                        if vars_len != 6 {
                            throw_err!("expected 6 values for robot command, got {}", vars_len);
                        }

                        let v_tan:  f32 = try!(vars[0].parse());
                        let v_norm: f32 = try!(vars[1].parse());
                        let v_ang:  f32 = try!(vars[2].parse());
                        let kick_x: f32 = try!(vars[3].parse());
                        let kick_z: f32 = try!(vars[4].parse());
                        let spin = try!(vars[5].parse::<i32>()) == 1;

                        robot_command.set_id(*robot_id as u32);
                        robot_command.set_kickspeedx(kick_x);
                        robot_command.set_kickspeedz(kick_z);
                        robot_command.set_veltangent(v_tan);
                        robot_command.set_velnormal(v_norm);
                        robot_command.set_velangular(v_ang);
                        robot_command.set_spinner(spin);
                        robot_command.set_wheelsspeed(false);
                        robot_commands.push(robot_command);
                    }
                }

                match packet.write_to_bytes() {
                    Ok(bytes) => {
                        match tx.send(bytes) {
                            Ok(_) => (),
                            Err(e) => {
                                println!("error sending to send_thread: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("error writing protobuf packet to byes: {}", e);
                    }
                }
            }
        }));

        Ok(ChildAiHandle {
            subproc_handle: child,
            debug_handle: debug_thread,
            child_handle: child_thread,
        })
    }
}

pub struct ChildAiHandle {
    subproc_handle: Child,
    debug_handle: JoinHandle<Result<()>>,
    child_handle: JoinHandle<Result<()>>,
}

impl InterfaceHandle for ChildAiHandle {
    fn join(self) -> Result<()> {
        let ChildAiHandle {
            subproc_handle: mut child,
            debug_handle: debug_thread,
            child_handle: child_thread,
        } = self;
        let child_result = try!(child_thread.join());
        let debug_result = try!(debug_thread.join());
        let exit_status = try!(child.wait());
        try!(child_result);
        try!(debug_result);
        if exit_status.success() {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Io, "child AI exited with failure"))
        }
    }
}
