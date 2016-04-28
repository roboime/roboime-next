//!	# Description
//!	- This module is responsible to encapsulating and do the communication with grSim and SSL-Vision in a simpler way
//!

use std::net::{UdpSocket, Ipv4Addr, SocketAddrV4};
use net2::{UdpBuilder, UdpSocketExt};
use time::{Duration, SteadyTime};
use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
#[allow(dead_code)]
enum SocketTask {
	Send,
	Receive,
}

#[derive(Debug)]
pub struct Interface {
	socket: UdpSocket,
	socket_task: SocketTask,			// Send or Receive
	socket_addr: SocketAddrV4,			// 127.0.0.1:20011
	socket_addr_face: SocketAddrV4,		// 0.0.0.0:00000
}

#[allow(dead_code)]
/// # grSim_sender
/// - Example of use
///
/// ```
/// let mut interface: Interface = Interface::new_send_grSim();
///	interface.connect_send();
///	interface.send_data(rx2);
/// ```
impl Interface {
	pub fn grSim_sender() -> Interface {
		Interface {
			socket: {
				match UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0)){
					Ok(s) => s,
            		Err(e) => panic!("couldn't instance interface: {}", e),
				}
			},
			socket_task: SocketTask::Send,
			socket_addr: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 20011),
			socket_addr_face: SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0),
		}
	}
        

	pub fn send_data(&mut self, rx2: Receiver<Vec<u8>>) -> () {
		self.socket = match UdpSocket::bind(self.socket_addr_face) {
            Ok(s) => s,
            Err(e) => panic!("couldn't bind socket: {}", e),
        };

        println!("send socket bound to {}", self.socket_addr_face);

		let mut last_time = SteadyTime::now();
        let mut counter = 0;

        loop {
            match rx2.recv() {
                Ok(ref bytes) => {
                    match self.socket.send_to(bytes, self.socket_addr) {
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

#[allow(dead_code)]
/// # grSim_receiver
/// - Example of use
///
/// ```
/// let mut interface: Interface = Interface::new_send_grSim();
///	interface.connect_send();
///	interface.send_data(rx2);
/// ```
impl Interface {
	pub fn grSim_receiver() -> Interface {
		Interface {
			socket: {
				match UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0)){
					Ok(s) => s,
            		Err(e) => panic!("couldn't instance interface: {}", e),
				}
			},
			socket_task: SocketTask::Receive,
			socket_addr: SocketAddrV4::new(Ipv4Addr::new(224, 5, 23, 2), 20011),
			socket_addr_face: SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 10002),
		}
	}

	pub fn receive_data(&mut self) -> () {
        println!("recv thread started");

        let iface = Ipv4Addr::new(0, 0, 0, 0);
        let addr = SocketAddrV4::new(iface, 10002);
        //let socket = match UdpSocket::bind("0.0.0.0:10005") {
        let socket = match UdpBuilder::new_v4().unwrap().reuse_address(true).unwrap().bind(socket_addr_face) {
            Ok(s) => s,
            Err(e) => panic!("couldn't bind socket: {}", e),
        };
        println!("bound to {}", socket_addr_face);

        let mcast_addr = Ipv4Addr::new(224, 5, 23, 2);
        match socket.join_multicast_v4(&socket_addr., &iface) {
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
	}
}