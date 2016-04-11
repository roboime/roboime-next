//!
//! Currently this is a simlpe proof of concpet.
//!

extern crate roboime_next_protocol as protocol;
extern crate net2;
extern crate time;

use std::net::{UdpSocket, Ipv4Addr, SocketAddrV4};
use std::sync::mpsc::channel;
use std::thread::spawn;
use protocol::{parse_from_bytes, Message};
use protocol::messages_robocup_ssl_wrapper::SSL_WrapperPacket;
use protocol::grSim_Packet::grSim_Packet;
use protocol::grSim_Commands::grSim_Robot_Command;
use protocol::messages_robocup_ssl_detection::SSL_DetectionFrame;
use net2::UdpSocketExt;
use time::{Duration, SteadyTime, precise_time_s};

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
        println!("work thread started");

        let mut packets: Vec<SSL_WrapperPacket> = Vec::new();
        let mut last_time = SteadyTime::now();
        struct Position{x: f32, y: f32, orientation: f32};
        impl Copy for Position {}
	impl Clone for Position { fn clone(&self) -> Position { *self } }
	impl std::ops::Sub<Position> for Position {
            type Output = Position;

            fn sub(self, _rhs: Position) -> Position {
                let result = Position{x: self.x - _rhs.x, y: self.y - _rhs.y, orientation: self.orientation - _rhs.orientation};
		//let result = Position{x:0.0, y: 0.0, orientation:0.0};
		result
            }
        }
        struct Velocity{normal: f32, tangent: f32, angular: f32};
        let hold_pos = Position{x: 2000.0, y: 40.0, orientation: 0.0};
        let mut pos = Position{x: 0.0, y: 0.0, orientation: 0.0};
	let mut answer = Velocity{normal: 0.0, tangent: 0.0, angular: 0.0};
        loop {
            let packet: SSL_WrapperPacket = rx.recv().unwrap();
            {
	        let frame = packet.get_detection();
		let robots =  frame.get_robots_blue();
		pos.x = robots[0].get_x();
		pos.y = robots[0].get_y();
                pos.orientation = robots[0].get_orientation();
            }
            packets.push(packet);
            {
		//let dif_pos = Position{x:hold_pos.x - pos.x, y: hold_pos.y - pos.y, orientation: hold_pos.orientation - pos.orientation};
		let dif_pos = hold_pos - pos;
		answer.tangent = 0.0004*( dif_pos.y*(pos.orientation.sin())); 
		answer.normal = 0.0004*( dif_pos.y*(pos.orientation.cos())); 
		answer.normal += 0.0004*( -dif_pos.x*(pos.orientation.sin())); 
		answer.tangent += 0.0004*( dif_pos.x*(pos.orientation.cos())); 
		answer.angular = 0.8*(dif_pos.orientation);
                println!("Pos x: {}, pos y: {}, orientation {}", dif_pos.x, dif_pos.y, dif_pos.orientation);
            }
            let mut packet = grSim_Packet::new();
            {
                let commands = packet.mut_commands();
                let timestamp = precise_time_s();
                commands.set_timestamp(timestamp);
                commands.set_isteamyellow(false);
                let robot_commands = commands.mut_robot_commands();

                let mut robot_command = grSim_Robot_Command::new();
                robot_command.set_id(0);
                robot_command.set_kickspeedx(0.0);
                robot_command.set_kickspeedz(0.0);
                robot_command.set_veltangent(answer.tangent);
                robot_command.set_velnormal(answer.normal);
                robot_command.set_velangular(answer.angular);
                robot_command.set_spinner(false);
                robot_command.set_wheelsspeed(false);

                robot_commands.push(robot_command);
            }

            match packet.write_to_bytes() {
                Ok(bytes) => {
                    match tx2.send(bytes) {
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

            let next_time = SteadyTime::now();
            let delta = next_time - last_time;
            if delta >= Duration::seconds(1) {
                println!("recv packets: {}, delta: {}", packets.len(), delta);
                packets.clear();
                last_time = next_time;
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

        let grsim_ip = Ipv4Addr::new(0, 0, 0, 0);
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
