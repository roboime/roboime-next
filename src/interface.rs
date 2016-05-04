//!
//! Function that create threads to interact with external tools.
//!

use std::thread::{self, JoinHandle};
use std::sync::mpsc::Receiver;
use std::net::{UdpSocket, Ipv4Addr, SocketAddrV4};
use net2::{UdpBuilder, UdpSocketExt};
use protocol::parse_from_bytes;
use protocol::messages_robocup_ssl_wrapper_legacy::SSL_WrapperPacket;
use protocol::messages_robocup_ssl_detection::SSL_DetectionFrame;
use protocol::messages_robocup_ssl_geometry_legacy::SSL_GeometryData;
use ::{Result, GameState, SharedGameState, RobotState, Position, Pose};

macro_rules! try_or {
    ($x:expr, |$e:ident| $b:block) => {
        match $x {
            Ok(x) => x,
            Err($e) => $b
        }
    }
}

// TODO: remove thread:Builder from all builders and use borrowed builder pattern

//#[derive(Debug, Clone)]
pub struct GrSimUpdaterBuilder {
    thread_builder: thread::Builder,
    interface: Ipv4Addr,
    vision_addr: Ipv4Addr,
    port: u16,
}

impl GameState {
    fn update_from_ssl_geometry(&mut self, _geometry: &SSL_GeometryData) {
        unimplemented!();
    }

    fn update_from_ssl_detection(&mut self, detection: &SSL_DetectionFrame) {
        let timestamp = detection.get_t_capture();
        let dt = timestamp - self.get_timestamp();
        self.set_timestamp(timestamp);

        // TODO: select ball
        for ball in detection.get_balls() {
            let ball_state = self.get_ball_mut();
            ball_state.update_position(ball.get_x(), ball.get_y(), dt);
        }

        // TODO: remove missing robots
        {
            let blue_robots = self.get_robots_blue_mut();
            for robot in detection.get_robots_blue() {
                let id = robot.get_robot_id() as u8;
                let robot_state = blue_robots.entry(id).or_insert_with(|| RobotState::new(id));
                robot_state.update_pose(robot.get_x(), robot.get_y(), robot.get_orientation(), dt);
            }
        }
        {
            let yellow_robots = self.get_robots_yellow_mut();
            for robot in detection.get_robots_yellow() {
                let id = robot.get_robot_id() as u8;
                let robot_state = yellow_robots.entry(id).or_insert_with(|| RobotState::new(id));
                robot_state.update_pose(robot.get_x(), robot.get_y(), robot.get_orientation(), dt);
            }
        }
    }
}

trait UdpSocketExt2 {
    fn recv_ssl_packet(&self, buf: &mut [u8]) -> Result<SSL_WrapperPacket>;
}

impl UdpSocketExt2 for UdpSocket {
    fn recv_ssl_packet(&self, buf: &mut [u8]) -> Result<SSL_WrapperPacket> {
        let (size, _) = try!(self.recv_from(buf));
        Ok(try!(parse_from_bytes::<SSL_WrapperPacket>(&buf[0..size])))
    }
}

impl GrSimUpdaterBuilder {
    /// Instantiate with default values.
    pub fn new() -> GrSimUpdaterBuilder {
        GrSimUpdaterBuilder {
            thread_builder: thread::Builder::new().name("grSim Updater".to_string()),
            interface: Ipv4Addr::new(0, 0, 0, 0),
            vision_addr: Ipv4Addr::new(224, 5, 23, 2),
            port: 10002,
        }
    }

    /// Set the listen interface address (IPv4 only for now)
    pub fn interface(mut self, interface: Ipv4Addr) -> GrSimUpdaterBuilder {
        self.interface = interface;
        self
    }

    /// Set the listen multicast address (IPv4 only for now)
    pub fn vision_addr(mut self, vision_addr: Ipv4Addr) -> GrSimUpdaterBuilder {
        self.vision_addr = vision_addr;
        self
    }

    /// Set the listen port
    pub fn port(mut self, port: u16) -> GrSimUpdaterBuilder {
        self.port = port;
        self
    }

    /// Spawns a grSim updater thread.
    pub fn spawn(self, game_state: SharedGameState) -> Result<JoinHandle<Result<()>>> {
        let iface = self.interface;
        let addr = SocketAddrV4::new(iface, self.port);
        let mcast_addr = self.vision_addr;

        Ok(try!(self.thread_builder.spawn(move || {
            let socket = try!(UdpBuilder::new_v4().unwrap().reuse_address(true).unwrap().bind(addr));
            try!(socket.join_multicast_v4(&mcast_addr, &iface));
            println!("grSim updater started on {}", addr);

            // 1KB buffer, packets are usually not greater than ~200 bytes
            let buf = &mut [0u8; 1024];

            loop {
                let packet = try_or!(socket.recv_ssl_packet(buf), |err| {
                    println!("error: {}; skipping packet...", err);
                    continue;
                });
                let mut game_state = try!(game_state.write());
                if packet.has_geometry() {
                    game_state.update_from_ssl_geometry(packet.get_geometry());
                }
                if packet.has_detection() {
                    game_state.update_from_ssl_detection(packet.get_detection());
                    game_state.inc_counter();
                }
            }
        })))
    }
}

//#[derive(Debug, Clone)]
pub struct GrSimCommanderBuilder {
    thread_builder: thread::Builder,
    interface: Ipv4Addr,
    grsim_addr: Ipv4Addr,
    port: u16,
}

impl GrSimCommanderBuilder {
    pub fn new() -> GrSimCommanderBuilder {
        GrSimCommanderBuilder {
            thread_builder: thread::Builder::new().name("grSim Commander".to_string()),
            interface: Ipv4Addr::new(0, 0, 0, 0),
            grsim_addr: Ipv4Addr::new(127, 0, 0, 1),
            port: 20011,
        }
    }

    /// Set the listen interface address (IPv4 only for now)
    pub fn interface(mut self, interface: Ipv4Addr) -> GrSimCommanderBuilder {
        self.interface = interface;
        self
    }

    /// Set the listen multicast address (IPv4 only for now)
    pub fn grsim_addr(mut self, grsim_addr: Ipv4Addr) -> GrSimCommanderBuilder {
        self.grsim_addr = grsim_addr;
        self
    }

    /// Set the listen port
    pub fn port(mut self, port: u16) -> GrSimCommanderBuilder {
        self.port = port;
        self
    }

    /// Start the commander thread
    pub fn spawn(self, rx: Receiver<Vec<u8>>) -> Result<JoinHandle<Result<()>>> {
        use time::{Duration, SteadyTime};

        let iface = self.interface.clone();
        let addr = SocketAddrV4::new(iface, 0);
        let grsim_ip = self.grsim_addr.clone();
        let grsim_addr = SocketAddrV4::new(grsim_ip, 20011);

        Ok(try!(self.thread_builder.spawn(move || {
            let socket = try!(UdpSocket::bind(addr));
            println!("grSim commander started on {}", addr);

            let mut last_time = SteadyTime::now();
            let mut counter = 0;

            loop {
                match rx.recv() {
                    Ok(ref bytes) => {
                        match socket.send_to(bytes, grsim_addr) {
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

            Ok(())
        })))
    }
}
