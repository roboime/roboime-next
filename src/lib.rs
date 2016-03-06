//!
//! Currently this is a simlpe proof of concpet.
//!

extern crate roboime_next_protocol as protocol;
extern crate net2;
extern crate time;

use std::net::{UdpSocket, Ipv4Addr, SocketAddrV4};
use std::sync::mpsc::channel;
use std::thread::spawn;
use protocol::parse_from_bytes;
use protocol::messages_robocup_ssl_wrapper::SSL_WrapperPacket;
use net2::UdpSocketExt;
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
        let mut buf = &mut [0u8; 1024];
        loop {
            match socket.recv_from(buf) {
                Ok((size, _)) => {
                    match parse_from_bytes::<SSL_WrapperPacket>(&buf[0..size]) {
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

    let work_thread = spawn(move || {
        println!("work thread started");

        let mut packets: Vec<SSL_WrapperPacket> = Vec::new();
        let mut last_time = SteadyTime::now();
        loop {
            packets.push(rx.recv().unwrap());
            let next_time = SteadyTime::now();
            let delta = next_time - last_time;
            if delta >= Duration::seconds(1) {
                println!("packets: {}, delta: {}", packets.len(), delta);
                packets.clear();
                last_time = next_time;
            }
        }
    });

    recv_thread.join().unwrap();
    work_thread.join().unwrap();
}
