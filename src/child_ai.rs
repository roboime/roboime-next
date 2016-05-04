use std::process::{Stdio, Command, Child, ChildStdin, ChildStdout, ChildStderr, ExitStatus};
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;
use ::{Result, Error, ErrorKind, SharedGameState, Position, Pose, InterfaceHandle};

/// Extension methods for the standard [`Command` type][link] in `std::process`.
///
/// [link]: https://doc.rust-lang.org/std/process/struct.Command.html
pub trait CommandExt {
    fn piped_spawn(&mut self) -> Result<Child>;
}

impl CommandExt for Command {
    fn piped_spawn(&mut self) -> Result<Child> {
        self.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| err.into())
    }
}

/// Extension methods for the standard [`Child` type][link] in `std::process`.
///
/// [link]: https://doc.rust-lang.org/std/process/struct.Child.html
pub trait ChildExt {
    fn map_all_pipes<U, F>(&mut self, op: F) -> Result<U>
        where F: FnOnce(&mut ChildStdin, &mut ChildStdout, &mut ChildStderr) -> Result<U>;
}

impl ChildExt for Child {
    fn map_all_pipes<U, F>(&mut self, op: F) -> Result<U>
        where F: FnOnce(&mut ChildStdin, &mut ChildStdout, &mut ChildStderr) -> Result<U>
    {
        if let Child { stdin: Some(ref mut child_in), stdout: Some(ref mut child_out), stderr: Some(ref mut child_err), .. } = *self {
            op(child_in, child_out, child_err)
        } else {
            Err(Error::new(ErrorKind::Io, "some pipes were missing from the child process"))
        }
    }
}

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
        use std::io::BufReader;
        use protocol::Message;
        use protocol::grSim_Packet::grSim_Packet;
        use protocol::grSim_Commands::grSim_Robot_Command;

        let is_yellow = self.is_yellow;
        let mut child = try!(self.command.piped_spawn());

        let thread_builder = thread::Builder::new().name("Child AI".to_string());
        let child_thread = try!(thread_builder.spawn(move || -> Result<ExitStatus> {

            macro_rules! throw_err {
                ( $( $tt:tt )* ) => {{
                    return Err(Error::new(ErrorKind::AiProtocol, format!($($tt)*)));
                }}
            }

            try!(child.map_all_pipes(|child_in, child_out, child_err| {
                let mut lines = BufReader::new(child_out).lines();

                // TODO: capture child_err on another thread
                //for line in BufReader::new(child_err).lines() {
                //    println!("got: {:?}", line);
                //}

                try!(writeln!(child_in, "ROBOIME_INTEL_PROTOCOL_VERSION 1"));
                {
                    let line = try!(match lines.next() {
                        Some(thing) => thing,
                        None => throw_err!("no output"),
                    });
                    match line.as_ref() {
                        "COMPATIBLE 1" => {
                            println!("child AI started");
                        },
                        s if s.starts_with("NOT_COMPATIBLE") => {
                            println!("child AI is not compatible, aborting...");
                            return Ok(());
                        },
                        _ => throw_err!("invalid version confirmation"),
                    }
                }

                {
                    let state = try!(game_state.wait_and_read());
                    let geom = state.get_field_geom();
                    // FIELD_LENGTH FIELD_WIDTH GOAL_WIDTH CENTER_CIRCLE_RADIUS DEFENSE_RADIUS DEFENSE_STRETCH FREE_KICK_FROM_DEFENSE_DIST PENALTY_SPOT_FROM_FIELD_LINE_DIST PENALTY_LINE_FROM_SPOT_DIST
                    try!(writeln!(child_in, "{:.03} {:.03} {:.03} {:.03} {:.03} {:.03} {:.03} {:.03} {:.03}",
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

                loop {
                    let state = try!(game_state.wait_and_read());
                    let timestamp = state.get_timestamp();
                    let counter = state.get_counter();

                    // COUNTER
                    try!(writeln!(child_in, "{}", counter));

                    // TIMESTAMP
                    try!(writeln!(child_in, "{}", timestamp));

                    // OUR_SCORE OPPONENT_SCORE
                    // TODO
                    try!(writeln!(child_in, "0 0"));

                    // REF_STATE <REF_TIME_LEFT|-1>
                    // TODO
                    try!(writeln!(child_in, "N -1"));

                    let ball = state.get_ball();

                    // BALL_X BALL_Y BALL_VX BALL_VY
                    try!(writeln!(child_in, "{:.04} {:.04} {:.04} {:.04}",
                        ball.get_x(),
                        ball.get_y(),
                        ball.get_vx(),
                        ball.get_vy(),
                    ));

                    // GOALKEEPER_ID
                    // TODO
                    try!(writeln!(child_in, "0"));

                    let (robots_our, robots_opponent) = {
                        let robots_yellow = state.get_robots_yellow();
                        let robots_blue = state.get_robots_blue();
                        if is_yellow {
                            (robots_yellow, robots_blue)
                        } else {
                            (robots_blue, robots_yellow)
                        }
                    };

                    // NUM_ROBOTS
                    try!(writeln!(child_in, "{}", robots_our.len()));

                    // OPPONENT_NUM_ROBOTS
                    try!(writeln!(child_in, "{}", robots_opponent.len()));

                    for (robot_id, robot) in robots_our {
                        // [ROBOT_ID ROBOT_X ROBOT_Y ROBOT_W ROBOT_VX ROBOT_VY ROBOT_VW] x NUM_ROBOTS
                        try!(writeln!(child_in, "{} {:.04} {:.04} {:.04} {:.04} {:.04} {:.04}",
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
                        try!(writeln!(child_in, "{} {:.04} {:.04} {:.04} {:.04} {:.04} {:.04}",
                            robot_id,
                            robot.get_x(),
                            robot.get_y(),
                            robot.get_w(),
                            robot.get_vx(),
                            robot.get_vy(),
                            robot.get_vw(),
                         ));
                    }

                    {
                        let line = try!(match lines.next() {
                            Some(thing) => thing,
                            None => { break; }
                        });
                        if line != format!("C {}", counter) {
                            throw_err!("wrong command counter, expected 'C {}' got {}", counter, line);
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

                            let line = try!(match lines.next() {
                                Some(thing) => thing,
                                None => { break; }
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

                for line in BufReader::new(child_err).lines() {
                    println!("{}", try!(line));
                }

                Ok(())
            }));

            Ok(try!(child.wait()))
        }));

        Ok(ChildAiHandle {
            child_handle: child_thread
        })
    }
}

pub struct ChildAiHandle {
    child_handle: JoinHandle<Result<ExitStatus>>,
}

impl InterfaceHandle for ChildAiHandle {
    fn join(self) -> Result<()> {
        let exit_status = try!(try!(self.child_handle.join()));
        if exit_status.success() {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Io, "child AI exited with failure"))
        }
    }
}
