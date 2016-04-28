//!
//! Currently this is a simlpe proof of concpet.
//!

extern crate roboime_next_protocol as protocol;
extern crate net2;
extern crate time;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::spawn;
use std::process::ExitStatus;
use protocol::{Message};
use protocol::messages_robocup_ssl_wrapper_legacy::SSL_WrapperPacket;
use protocol::grSim_Packet::grSim_Packet;
use protocol::grSim_Commands::grSim_Robot_Command;
//use time::{Duration, SteadyTime, precise_time_s};

pub use self::error::{Result, Error, ErrorKind};
pub use subproc::{CommandExt, ChildExt};

mod error;
mod subproc;
pub mod interface;
use interface::Interface;

/// - One thread will bind to 0.0.0.0 and join the multicast to receive and dispatch vision packets
///   in a loop;
/// - Another thread will buffer the dispatched packets and if more than 1s passes after the last
///   tick it will print the size of the buffer, clear it, and reset the tick;
/// - Wait for both looping threads to join.
///
pub fn demo() {
    let (tx, rx) = channel();
    let recv_thread = spawn(move || {
        let mut interface: Interface = Interface::gr_sim_receiver();
        interface.receive_from_gr_sim(tx);
    });

    let (tx2, rx2) = channel();
    let work_thread = spawn(move || {
        match child_proc(rx, tx2) {
            Ok(s) => {
                println!("child subproc exited with: {}", s);
            }
            Err(e) => {
                println!("child subproc errored: {}", e);
            }
        }
    });

    let send_thread = spawn(move || {
        let mut interface: Interface = Interface::gr_sim_sender();
        interface.send_to_gr_sim(rx2);
    });

    recv_thread.join().unwrap();
    work_thread.join().unwrap();
    send_thread.join().unwrap();
}

fn child_proc(rx: Receiver<SSL_WrapperPacket>, tx: Sender<Vec<u8>>) -> Result<ExitStatus> {
    use std::process::Command;
    use std::io::prelude::*;
    use std::io::BufReader;

    println!("work thread started");

    let mut child = try!(Command::new("./demo-ai").piped_spawn());
    println!("running subproc");

    fn new_err(msg: &str) -> error::Error {
        error::Error::new(error::ErrorKind::AiProtocol, msg.to_string())
    }

    try!(child.map_all_pipes(|child_in, child_out, child_err| {
        let mut lines = BufReader::new(child_out).lines();

        try!(writeln!(child_in, "ROBOIME_INTEL_PROTOCOL_VERSION 1"));
        {
            let line = try!(match lines.next() {
                Some(thing) => thing,
                None => {
                    return Err(new_err("no output"));
                }
            });
            match line.as_ref() {
                "COMPATIBLE 1" => {
                    println!("AI is compatible");
                },
                s if s.starts_with("NOT_COMPATIBLE") => {
                    println!("AI is not compatible");
                    return Ok(());
                },
                _ => {
                    return Err(new_err("invalid version confirmation"));
                }
            }
        }

        // TODO: get these from the geometry, or better, from a game state

        // FIELD_WIDTH
        try!(writeln!(child_in, "4.000"));
        // FIELD_HEIGHT
        try!(writeln!(child_in, "6.000"));
        // GOAL_WIDTH
        try!(writeln!(child_in, "0.700"));
        // CENTER_CIRCLE_RADIUS
        try!(writeln!(child_in, "0.500"));
        // DEFENSE_RADIUS
        try!(writeln!(child_in, "0.500"));
        // DEFENSE_STRETCH
        try!(writeln!(child_in, "0.350"));
        // FREE_KICK_FROM_DEFENSE_DIST
        try!(writeln!(child_in, "0.700"));
        // PENALTY_SPOT_FROM_FIELD_LINE_DIST
        try!(writeln!(child_in, "0.450"));
        // PENALTY_LINE_FROM_SPOT_DIST
        try!(writeln!(child_in, "0.350"));

        //for line in BufReader::new(child_err).lines() {
        //    println!("got: {:?}", line);
        //}

        let mut counter: u64 = 0;
        loop {
            let packet = rx.recv().unwrap();
            if !packet.has_detection() {
                println!("geometry packet");
                continue;
            }
            let detection = packet.get_detection();

            //let timestamp = precise_time_s();
            let timestamp = detection.get_t_capture();

            // COUNTER
            try!(writeln!(child_in, "{}", counter));

            // TIMESTAMP
            try!(writeln!(child_in, "{}", timestamp));

            // OUR_SCORE OPPONENT_SCORE
            try!(writeln!(child_in, "0 0"));

            // REF_STATE <REF_TIME_LEFT|-1>
            try!(writeln!(child_in, "N -1"));

            let balls = detection.get_balls();
            let ball = &balls[0]; // TODO: get best ball

            // BALL_X BALL_Y BALL_VX BALL_VY
            try!(writeln!(child_in, "{} {} 0 0", ball.get_x(), ball.get_y()));

            // GOALKEEPER_ID
            try!(writeln!(child_in, "0"));

            let robots_yellow = detection.get_robots_yellow();
            let robots_blue = detection.get_robots_blue();

            // NUM_ROBOTS
            try!(writeln!(child_in, "{}", robots_yellow.len()));

            // OPPONENT_NUM_ROBOTS
            try!(writeln!(child_in, "{}", robots_blue.len()));

            for robot in robots_yellow {
                // [ROBOT_ID ROBOT_X ROBOT_Y ROBOT_W ROBOT_VX ROBOT_VY ROBOT_VW] x NUM_ROBOTS
                try!(writeln!(child_in,
                         "{} {} {} {} 0 0 0",
                         robot.get_robot_id(),
                         robot.get_x(),
                         robot.get_y(),
                         robot.get_orientation()
                        ));
            }

            for robot in robots_blue {
                // [ROBOT_ID ROBOT_X ROBOT_Y ROBOT_W ROBOT_VX ROBOT_VY ROBOT_VW] x OPPONENT_NUM_ROBOTS
                try!(writeln!(child_in,
                         "{} {} {} {} 0 0 0",
                         robot.get_robot_id(),
                         robot.get_x(),
                         robot.get_y(),
                         robot.get_orientation()
                        ));
            }

            {
                let line = try!(match lines.next() {
                    Some(thing) => thing,
                    None => { break; }
                });
                if line != format!("C {}", counter) {
                    return Err(new_err(format!("wrong command counter, expected 'C {}' got {}", counter, line).as_ref()));
                }
            }

            let mut packet = grSim_Packet::new();
            {
                let commands = packet.mut_commands();
                commands.set_timestamp(timestamp);
                commands.set_isteamyellow(true);
                let robot_commands = commands.mut_robot_commands();

                for robot in robots_yellow {
                    let mut robot_command = grSim_Robot_Command::new();
                    let robot_id = robot.get_robot_id();

                    let line = try!(match lines.next() {
                        Some(thing) => thing,
                        None => { break; }
                    });

                    let vars: Vec<_> = line.split(' ').collect();
                    assert_eq!(vars.len(), 6);

                    let v_tan:  f32 = try!(vars[0].parse());
                    let v_norm: f32 = try!(vars[1].parse());
                    let v_ang:  f32 = try!(vars[2].parse());
                    let kick_x: f32 = try!(vars[3].parse());
                    let kick_z: f32 = try!(vars[4].parse());
                    let spin = try!(vars[5].parse::<i32>()) == 1;

                    robot_command.set_id(robot_id);
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

            // update the counter after receiving a command
            counter += 1;

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
}
