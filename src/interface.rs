use std::thread::{self, JoinHandle};
use std::sync::mpsc::Receiver;
use std::net::{IpAddr, SocketAddr, UdpSocket, Ipv4Addr, SocketAddrV4, ToSocketAddrs};
use net2::{UdpBuilder, UdpSocketExt};
use protocol::parse_from_bytes;
use protocol::messages_robocup_ssl_wrapper_legacy::SSL_WrapperPacket;
use protocol::messages_robocup_ssl_detection::SSL_DetectionFrame;
use protocol::messages_robocup_ssl_geometry_legacy::SSL_GeometryData;
use ::{Result, Error, ErrorKind, GameState, SharedGameState, RobotState, Position, Pose};

pub trait InterfaceHandle {
    fn join(self) -> Result<()>;
}

macro_rules! peel {
    ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
}

macro_rules! tuple {
    () => ();
    ( $($name:ident,)+ ) => (
        impl<$($name:InterfaceHandle),*> InterfaceHandle for ($($name,)*) {
            #[allow(non_snake_case, unused_assignments)]
            fn join(self) -> Result<()> {
                let ($($name,)*) = self;
                $(
                    try!($name.join());
                )*
                Ok(())
            }
        }
        peel! { $($name,)* }
    )
}

tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }

macro_rules! try_or {
    ($x:expr, |$e:ident| $b:block) => {
        match $x {
            Ok(x) => x,
            Err($e) => $b
        }
    }
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

trait ToSocketAddrsExt : ToSocketAddrs {
    fn to_single_socket_addr(&self) -> Result<SocketAddr> {
        let mut addrs_iter = try!(self.to_socket_addrs());
        match addrs_iter.next() {
            None => Err(Error::new(ErrorKind::Io, "no address given")),
            Some(addr) => match addrs_iter.next() {
                None => Ok(addr),
                Some(_) => Err(Error::new(ErrorKind::Io, "multiple addresses given")),
            }
        }
    }
}

impl<T: ToSocketAddrs> ToSocketAddrsExt for T {}

/// A builder for an interface to [grSim](https://github.com/roboime/grSim/)
///
/// __NOTE:__ currently it will any interface, a future improvement should allow setting which
/// interface to bind on.
///
#[derive(Debug, Clone)]
pub struct GrSimInterface {
    vision_addr: SocketAddr,
    grsim_addr: SocketAddr,
}

impl GrSimInterface {
    /// Instantiate with default values.
    pub fn new() -> GrSimInterface {
        GrSimInterface {
            vision_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(224, 5, 23, 2)), 10002),
            grsim_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 20011),
        }
    }

    /// Set the vision address and port used on the simulator, the address is used for
    /// joining the multicast and the port for binding that socket.
    pub fn vision_addr<A: ToSocketAddrs>(&mut self, addrs: A) -> Result<&GrSimInterface> {
        self.vision_addr = try!(addrs.to_single_socket_addr());
        Ok(self)
    }

    /// Set the grsim address and port used on the simulator, this is used when sending commands to
    /// grSim.
    pub fn grsim_addr<A: ToSocketAddrs>(&mut self, addrs: A) -> Result<&GrSimInterface> {
        self.grsim_addr = try!(addrs.to_single_socket_addr());
        Ok(self)
    }

    /// Spawn the necessary threads and start listening to changes and ppushing commands.
    pub fn spawn(&self, game_state: SharedGameState, rx: Receiver<Vec<u8>>) -> Result<GrSimHandle> {
        use time::{Duration, SteadyTime};

        let any_addr = Ipv4Addr::new(0, 0, 0, 0);
        let vision_addr = self.vision_addr.clone();
        let vision_bind = SocketAddrV4::new(any_addr, vision_addr.port());
        let vision_mcast = match vision_addr {
            SocketAddr::V4(socket) => *socket.ip(),
            SocketAddr::V6(_) => {
                return Err(Error::new(ErrorKind::Io, "vision multicast address in IPv6 is not supported"));
            }
        };
        let thread_builder = thread::Builder::new().name("grSim Updater".to_string());
        let updater_thread = try!(thread_builder.spawn(move || {

            let socket = try!(UdpBuilder::new_v4().unwrap().reuse_address(true).unwrap().bind(vision_bind));
            try!(socket.join_multicast_v4(&vision_mcast, &any_addr));

            println!("grSim interaface receiving from {}", vision_addr);

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
        }));

        let any_addr = Ipv4Addr::new(0, 0, 0, 0);
        let grsim_bind = SocketAddrV4::new(any_addr, 0);
        let grsim_addr = self.grsim_addr.clone();
        let thread_builder = thread::Builder::new().name("grSim Commander".to_string());
        let commander_thread = try!(thread_builder.spawn(move || {
            let socket = try!(UdpSocket::bind(grsim_bind));

            println!("grSim interface sending to {}", grsim_addr);

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
        }));

        Ok(GrSimHandle {
            updater_handle: updater_thread,
            commander_handle: commander_thread,
        })
    }
}

pub struct GrSimHandle {
    updater_handle: JoinHandle<Result<()>>,
    commander_handle: JoinHandle<Result<()>>,
}

impl InterfaceHandle for GrSimHandle {
    fn join(self) -> Result<()> {
        let GrSimHandle {
            updater_handle: updater_thread,
            commander_handle: commander_thread,
        } = self;
        let updater_result = try!(updater_thread.join());
        let commander_result = try!(commander_thread.join());
        try!(updater_result);
        try!(commander_result);
        Ok(())
    }
}
