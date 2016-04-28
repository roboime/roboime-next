//!	# Description
//!	- This module is responsible to encapsulating and do the communication with grSim and SSL-Vision in a simpler way
//!

extern crate roboime_next_protocol as protocol;

use std::net::{UdpSocket, Ipv4Addr, SocketAddrV4};
use net2::{UdpBuilder, UdpSocketExt};
use time::{Duration, SteadyTime};
use std::sync::mpsc::{Receiver, Sender};
use protocol::{parse_from_bytes};
use protocol::messages_robocup_ssl_wrapper_legacy::SSL_WrapperPacket;
//use protocol::grSim_Packet::grSim_Packet;
//use protocol::grSim_Commands::grSim_Robot_Command;

#[derive(Debug)]
#[allow(dead_code)]
enum SocketTask {
	Send,
	Receive,
}

#[derive(Debug)]
pub struct Interface {
	socket_task: SocketTask,	
	addr_face: Ipv4Addr,
	port_face: u16,
	addr_socket: Ipv4Addr,
	port_socket: u16,
}

impl Interface {
	pub fn gr_sim_sender() -> Interface {
		Interface {
			socket_task: SocketTask::Send,
			addr_face: Ipv4Addr::new(0, 0, 0, 0),
			port_face: 0,
			addr_socket: Ipv4Addr::new(127, 0, 0, 1),
			port_socket: 20011,
		}
	}

	pub fn send_to_gr_sim(&mut self, rx2: Receiver<Vec<u8>>) -> () {
        let addr_face_socket = SocketAddrV4::new(self.addr_face, self.port_face);
        let socket = match UdpSocket::bind(addr_face_socket) {
            Ok(s) => s,
            Err(e) => panic!("couldn't bind socket: {}", e),
        };
        println!("send socket bound to {}", addr_face_socket);

        let addr_grsim_socket = SocketAddrV4::new(self.addr_socket, self.port_socket);

        let mut last_time = SteadyTime::now();
        let mut counter = 0;

        loop {
            match rx2.recv() {
                Ok(ref bytes) => {
                    match socket.send_to(bytes, addr_grsim_socket) {
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
	}
}

impl Interface {
	pub fn gr_sim_receiver() -> Interface {
		Interface {
			socket_task: SocketTask::Receive,
			addr_face: Ipv4Addr::new(0, 0, 0, 0),
			port_face: 10002,
			addr_socket: Ipv4Addr::new(224, 5, 23, 2),
			port_socket: 10002,
		}
	}

	pub fn receive_from_gr_sim(&mut self, tx: Sender<SSL_WrapperPacket>) -> () {
		println!("recv thread started");

        let addr_face_socket = SocketAddrV4::new(self.addr_face, self.port_face);
        let socket = match UdpBuilder::new_v4().unwrap().reuse_address(true).unwrap().bind(addr_face_socket) {
            Ok(s) => s,
            Err(e) => panic!("couldn't bind socket: {}", e),
        };
        println!("bound to {}", addr_face_socket);

        match socket.join_multicast_v4(&self.addr_socket, &self.addr_face) {
            Ok(()) => (),
            Err(e) => panic!("couldn't join multicast: {}", e),
        }
        println!("joined multicast {}", self.addr_socket);

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
	}
}