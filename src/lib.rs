//!
//! Currently this is a simlpe proof of concpet.
//!

extern crate roboime_next_protocol as protocol;
extern crate net2;
extern crate time;

use std::net::{UdpSocket, Ipv4Addr, SocketAddrV4};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::spawn;
use protocol::{parse_from_bytes, Message};
use protocol::messages_robocup_ssl_wrapper::SSL_WrapperPacket;
use protocol::grSim_Packet::grSim_Packet;
use protocol::grSim_Commands::grSim_Robot_Command;
use net2::UdpSocketExt;
//use time::{Duration, SteadyTime, precise_time_s};
use time::{Duration, SteadyTime};

pub use self::error::{Result, Error, ErrorKind};
pub use subproc::{CommandExt, ChildExt};

mod error;
mod subproc;

/// - One thread will bind to 0.0.0.0 and join the multicast to receive and dispatch vision packets
///   in a loop;
/// - Another thread will buffer the dispatched packets and if more than 1s passes after the last
///   tick it will print the size of the buffer, clear it, and reset the tick;
/// - Wait for both looping threads to join.
///
pub fn demo() {
    let (tx, rx) = channel();
    let recv_thread = spawn(move || {
        println!("recv thread started");

        let iface = Ipv4Addr::new(0, 0, 0, 0);
        let addr = SocketAddrV4::new(iface, 10002);
        //let socket = match UdpSocket::bind("0.0.0.0:10005") {
        let socket = match UdpSocket::bind(addr) {
            Ok(s) => s,
            Err(e) => panic!("couldn't bind socket: {}", e),
        };
        println!("bound to {}", addr);

        let mcast_addr = Ipv4Addr::new(224, 5, 23, 2);
        match socket.join_multicast_v4(&mcast_addr, &iface) {
            Ok(()) => (),
            Err(e) => panic!("couldn't join multicast: {}", e),
        }
        println!("joined multicast {}", mcast_addr);

        // 1KB buffer
        let buf = &mut [0u8; 1024];
        loop {
            match socket.recv_from(buf) {
                Ok((size, _)) => {
                    //match parse_from_bytes::<SSL_WrapperPacket>(&buf[0..size]) {
                    match parse_from_bytes(&buf[0..size]) {
                        Ok(packet) => match tx.send(packet) {
                            Ok(()) => {}
                            Err(e) => {
                                println!("couldn't send datagram to other thread: {}", e);
                                break;
                            }
                        },
                        Err(e) => {
                            println!("couldn't parse datagram: {}", e);
                        }
                    }
                },
                Err(e) => {
                    println!("couldn't receive datagram: {}", e);
                }
            }
        }
    });

    let (tx2, rx2) = channel();
    let work_thread = spawn(move || {
        match child_proc(rx, tx2) {
            Ok(()) => {}
            Err(e) => {
                println!("child subproc errored: {}", e);
            }
        }
    });

    let send_thread = spawn(move || {
        let iface = Ipv4Addr::new(0, 0, 0, 0);
        let addr = SocketAddrV4::new(iface, 0);
        let socket = match UdpSocket::bind(addr) {
            Ok(s) => s,
            Err(e) => panic!("couldn't bind socket: {}", e),
        };
        println!("send socket bound to {}", addr);

        let grsim_ip = Ipv4Addr::new(192, 168, 91, 92);
        let grsim_addr = SocketAddrV4::new(grsim_ip, 20011);

        let mut last_time = SteadyTime::now();
        let mut counter = 0;

        loop {
            match rx2.recv() {
                Ok(ref bytes) => {
                    match socket.send_to(bytes, grsim_addr) {
                        //Ok(size) => { println!("{} bytes sent", size); }
                        Ok(_) => { counter += 1; },
                        Err(e) => { println!("error sending bytes to grSim: {}", e); }
                    }
                }
                Err(e) => {
                    println!("error receiving from work_thread: {}", e);
                    break;
                }
            }

            let next_time = SteadyTime::now();
            let delta = next_time - last_time;
            if delta >= Duration::seconds(1) {
                println!("sent packets: {}, delta: {}", counter, delta);
                counter = 0;
                last_time = next_time;
            }
        }
    });

    recv_thread.join().unwrap();
    work_thread.join().unwrap();
    send_thread.join().unwrap();
}

fn child_proc(rx: Receiver<SSL_WrapperPacket>, tx: Sender<Vec<u8>>) -> Result<()> {
    use std::process::Command;
    use std::io::prelude::*;
    use std::io::BufReader;

    println!("work thread started");

    let mut child = try!(Command::new("./demo-ai").piped_spawn());
    println!("running subproc");

    try!(child.map_all_pipes(|child_in, child_out, child_err| {
        let mut lines = BufReader::new(child_out).lines();
        //TODO: setup phase

        //writeln!(child_in, "bar").unwrap();
        //println!("got: {:?}", lines.next());

        //writeln!(child_in, "enough").unwrap();
        //println!("got: {:?}", lines.next());

        //for line in BufReader::new(child_err).lines() {
        //    println!("got: {:?}", line);
        //}

        loop {
            let packet = rx.recv().unwrap();
            if !packet.has_detection() {
                println!("geometry packet");
                continue;
            }
            let detection = packet.get_detection();

            //let timestamp = precise_time_s();
            let timestamp = detection.get_t_capture();

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
                assert!(line.starts_with("command"));
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

    // unreachable:
    //Ok(try!(child.wait()))
    Ok(())
}
