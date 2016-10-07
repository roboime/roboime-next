use std::slice;
use std::net::{IpAddr, SocketAddr, UdpSocket, Ipv4Addr, SocketAddrV4, ToSocketAddrs};
use net2::UdpBuilder;
use protocol::parse_from_bytes;
use protocol::messages_robocup_ssl_wrapper_legacy::SSL_WrapperPacket;
use protocol::messages_robocup_ssl_detection::{SSL_DetectionFrame, SSL_DetectionRobot, SSL_DetectionBall};
use protocol::messages_robocup_ssl_geometry_legacy::SSL_GeometryFieldSize;
use ::prelude::*;
use ::{game, Result, Error, ErrorKind};

#[cfg(feature="usb-transceiver")]
mod transceiver {
    use std::time::Duration;
    use std::sync::Mutex;
    use libusb::{Context, DeviceHandle};
    use ::{game, Result, Error, ErrorKind};

    lazy_static! {
        static ref CTX: Context = Context::new().unwrap();
        pub static ref HANDLE: Mutex<Transceiver> = Mutex::new(Transceiver::new().unwrap());
    }

    pub struct Transceiver {
        handle: DeviceHandle<'static>,
    }

    impl Transceiver {
        pub fn new() -> Result<Transceiver> {
            #[inline(always)]
            fn not_found() -> Error {
                Error::new(ErrorKind::Io, "RoboIME transceiver not found")
            }
            let mut handle = try!(CTX.open_device_with_vid_pid(0x0483, 0x5740).ok_or_else(not_found));
            if try!(handle.kernel_driver_active(0)) {
                try!(handle.detach_kernel_driver(0));
            }
            try!(handle.claim_interface(0));
            info!("usb transceiver claimed");

            Ok(Transceiver {
                handle: handle,
            })
        }

        pub fn send_command(&mut self, command: game::Command, _buf: &mut [u8]) -> Result<()> {
            let &mut Transceiver { ref mut handle } = self;

            for (robot_id, robot_command) in command.robots {
                use std::f32::consts::FRAC_1_SQRT_2;
                use game::RobotAction::*;

                fn write_speed(d: &mut [u8], s: f32) {
                    assert!(d.len() >= 2);
                    let limf = 1000.0;
                    let limi = limf as i16;
                    let v = if s > limf { limi } else if s < -limf { -limi } else { s as i16 };
                    d[0] = v as u8;
                    d[1] = (v >> 8) as u8;
                }

                // use robot_id
                debug!("robot_id: {}, v_norm: {}, v_tang: {}, v_ang: {}", robot_id,
                       robot_command.v_normal, robot_command.v_tangent, robot_command.v_angular);

                // Protocol (29 bytes):
                //
                //  0                   1                   2
                //  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8
                // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
                // |M|I|~~~~~~~~~~~~~~~~~|K|D|~| Vn| Vt| Va|~~~~~~~~~~~~~~~~~|
                // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
                //
                // ~: reserved
                // M: magic number: 'a' in ascii = 0x61 = 97
                // I: robot id: 0-11
                // K: kick boolean: none=0b00000000, normal=0b00000001, chip=0b00000010
                // D: dribble velocity: u8
                // Vn: normal velocity: little-endian i16 (1000 = 4m/s [max abs])
                // Vt: normal velocity: little-endian i16 (1000 = 4m/s [max abs])
                // Va: normal velocity: little-endian i16 (unit not yet specified)
                //
                let mut data = [0u8; 29];

                data[0] = 0x61; // magic number?
                data[1] = robot_id;

                write_speed(&mut data[14..], -250.0 * robot_command.v_normal);
                write_speed(&mut data[16..],  250.0 * robot_command.v_tangent);
                write_speed(&mut data[18..],  100.0 * robot_command.v_angular);

                match robot_command.action {
                    Normal => {}
                    Dribble => {
                        let dribble_speed = 0xfe; // could be dynamic...
                        data[12] = dribble_speed;
                    }
                    Kick(force) => {
                        // should be dynamic...
                        if force > 0.0 {
                            data[11] = 0b_0000_0001;
                        }
                    }
                    ChipKick(force) => {
                        // XXX: hardcoded 45 degrees angled chip kick
                        let d_force = force * FRAC_1_SQRT_2;
                        // should be dynamic...
                        if d_force > 0.0 {
                            data[11] = 0b_0000_0010;
                        }
                    }
                }

                trace!("{:?}", &data);
                let write_res = handle.write_bulk(0x01, &data, Duration::from_millis(100));
                if let Err(err) = write_res {
                    warn!("Error on bulk write: {:?}", err);
                }
                //let read_res = handle.read_bulk(0x81, buf, Duration::from_millis(100));
                //if let Err(err) = read_res {
                //    warn!("Error on bulk read: {:?}", err);
                //}
            }

            Ok(())
        }
    }
}
#[cfg(not(feature="usb-transceiver"))]
mod transceiver {
    use std::marker::PhantomData;
    use ::{game, Result};

    pub struct Transceiver;

    impl Transceiver {
        pub fn new() -> Result<Transceiver> {
            info!("using dummy transceiver");
            Ok(Transceiver)
        }

        pub fn send_command(&mut self, _command: game::Command, _buf: &mut [u8]) -> Result<()> {
            debug!("dummy send_command");
            Ok(())
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
pub struct Builder {
    vision_addr: SocketAddr,
    team_side: TeamSide,
}

impl Builder {
    /// Instantiate with default values.
    pub fn new() -> Builder {
        Builder {
            vision_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(224, 5, 23, 2)), 10005),
            team_side: Default::default(),
        }
    }

    pub fn team_side(&mut self, team_side: TeamSide) -> &mut Builder {
        self.team_side = team_side;
        self
    }

    /// Set the vision address and port used on the simulator, the address is used for
    /// joining the multicast and the port for binding that socket.
    pub fn vision_addr<A: ToSocketAddrs>(&mut self, addrs: A) -> Result<&mut Builder> {
        self.vision_addr = try!(addrs.to_single_socket_addr());
        Ok(self)
    }

    /// Set the vision port used on the simulator used for binding the socket.
    pub fn vision_port(&mut self, port: u16) -> &mut Builder {
        // TODO: use self.vision_addr.set_port when sockaddr_setters is stable
        self.vision_addr = SocketAddr::new(self.vision_addr.ip(), port);
        self
    }

    /// Spawn the necessary threads and start listening to changes and ppushing commands.
    pub fn build(&mut self) -> Result<State> {
        let any_addr = Ipv4Addr::new(0, 0, 0, 0);
        let vision_addr = self.vision_addr.clone();
        let vision_bind = SocketAddrV4::new(any_addr, vision_addr.port());
        let vision_mcast = match vision_addr {
            SocketAddr::V4(socket) => *socket.ip(),
            SocketAddr::V6(_) => {
                return Err(Error::new(ErrorKind::Io, "vision multicast address in IPv6 is not supported"));
            }
        };

        let recv_socket = try!(UdpBuilder::new_v4().unwrap().reuse_address(true).unwrap().bind(vision_bind));
        try!(recv_socket.join_multicast_v4(&vision_mcast, &any_addr));

        info!("receiving from {}", vision_addr);

        //let mut context = try!(Context::new());
        //let transceiver = try!(Transceiver::new());

        Ok(State {
            recv_socket: recv_socket,
            buf: [0; 1024],
            geometry: SSL_GeometryFieldSize::new(),
            detection: SSL_DetectionFrame::new(),
            counter: 0,
            //transceiver: transceiver,
            team_side: self.team_side,
            // XXX: hardcoded team info!!!!
            blue_team_info: TeamInfo { score: 0, goalie: 5 },
            yellow_team_info: TeamInfo { score: 0, goalie: 2 },
        })
    }
}

pub struct State {
    recv_socket: UdpSocket,
    // 1KB buffer, packets are usually not greater than ~200 bytes
    buf: [u8; 1024],
    geometry: SSL_GeometryFieldSize,
    detection: SSL_DetectionFrame,
    counter: u64,
    //transceiver: Transceiver,
    team_side: TeamSide,
    blue_team_info: TeamInfo,
    yellow_team_info: TeamInfo,
}

impl State {
    /// Returns Ok(true) if there was a geometry update.
    pub fn recv_state(&mut self) -> Result<()> {
        let &mut State { ref mut buf, ref recv_socket, ..  } = self;

        let packet = try!(recv_socket.recv_ssl_packet(buf));

        if packet.has_geometry() {
            self.geometry = packet.get_geometry().get_field().clone();
        }
        if packet.has_detection() {
            self.detection = packet.get_detection().clone();
            self.counter += 1;
        }

        Ok(())
    }

    pub fn wait_for_geom(&mut self) -> Result<()> {
        let &mut State { ref mut buf, ref recv_socket, ..  } = self;

        loop {
            let packet = try!(recv_socket.recv_ssl_packet(buf));
            if packet.has_geometry() {
                self.geometry = packet.get_geometry().get_field().clone();
                break;
            }
        }

        Ok(())
    }

    pub fn send_command(&mut self, command: game::Command) -> Result<()> {
        //self.transceiver.send_command(command, &mut self.buf)
        let mut transceiver = try!(transceiver::HANDLE.lock());
        transceiver.send_command(command, &mut self.buf)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TeamInfo {
    goalie: u8,
    score: u8,
}

impl<'a> game::TeamInfo for TeamInfo {
    fn score(&self) -> u8 { self.score }
    fn goalie(&self) -> u8 { self.goalie }
}

impl<'a> game::State<'a> for State {
    type Ball = &'a SSL_DetectionBall;
    type Robot = (Color, &'a SSL_DetectionRobot);
    type Robots = Iter<'a>;
    type Geometry = &'a SSL_GeometryFieldSize;
    type TeamInfo = TeamInfo;

    fn counter(&self) -> u64 {
        self.counter
    }

    fn timestamp(&self) -> f64 {
        self.detection.get_t_capture()
    }

    fn ball(&'a self) -> Self::Ball {
        self.detection.get_balls().get(0).unwrap_or(SSL_DetectionBall::default_instance())
    }

    fn robot(&'a self, id: Id) -> Option<Self::Robot> {
        match id {
            Id(Blue, i) => self.detection.get_robots_blue().get(i as usize).map(|r| (Blue, r)),
            Id(Yellow, i) => self.detection.get_robots_yellow().get(i as usize).map(|r| (Yellow, r)),
        }
    }

    fn robots(&'a self) -> Self::Robots {
        Iter {
            blue_robots: self.detection.get_robots_blue().iter(),
            yellow_robots: self.detection.get_robots_yellow().iter(),
            state: IterState::Both,
        }
    }

    fn robots_team(&'a self, color: Color) -> Self::Robots {
        match color {
            Blue => Iter {
                blue_robots: self.detection.get_robots_blue().iter(),
                yellow_robots: [].iter(),
                state: IterState::Blue,
            },
            Yellow => Iter {
                blue_robots: [].iter(),
                yellow_robots: self.detection.get_robots_yellow().iter(),
                state: IterState::Yellow,
            },
        }
    }

    fn geometry(&'a self) -> Self::Geometry {
        &self.geometry
    }

    fn team_info(&'a self, color: Color) -> Self::TeamInfo {
        match color {
            Blue => self.blue_team_info,
            Yellow => self.yellow_team_info,
        }
    }

    // in grSim, blue is always left
    fn team_side(&self) -> TeamSide { self.team_side }
}

enum IterState {
    Both,
    Blue,
    Yellow,
}

pub struct Iter<'a> {
    blue_robots: slice::Iter<'a, SSL_DetectionRobot>,
    yellow_robots: slice::Iter<'a, SSL_DetectionRobot>,
    state: IterState,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (Color, &'a SSL_DetectionRobot);

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            IterState::Both => match self.blue_robots.next() {
                e @ Some(..) => e.map(|r| (Blue, r)),
                None => {
                    self.state = IterState::Yellow;
                    self.yellow_robots.next().map(|r| (Yellow, r))
                }
            },
            IterState::Blue => self.blue_robots.next().map(|r| (Blue, r)),
            IterState::Yellow => self.yellow_robots.next().map(|r| (Yellow, r)),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = match self.state {
            IterState::Both => self.blue_robots.len() + self.yellow_robots.len(),
            IterState::Blue => self.blue_robots.len(),
            IterState::Yellow => self.yellow_robots.len(),
        };
        (len, Some(len))
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}
