use std::slice;
use std::net::{IpAddr, SocketAddr, UdpSocket, Ipv4Addr, SocketAddrV4, ToSocketAddrs};
use net2::{UdpBuilder, UdpSocketExt};
use protocol::{Message, parse_from_bytes};
use protocol::messages_robocup_ssl_wrapper_legacy::SSL_WrapperPacket;
use protocol::messages_robocup_ssl_detection::{SSL_DetectionFrame, SSL_DetectionRobot, SSL_DetectionBall};
use protocol::messages_robocup_ssl_geometry_legacy::SSL_GeometryFieldSize;
use protocol::grSim_Packet::grSim_Packet;
use protocol::grSim_Commands::grSim_Robot_Command;
use ::prelude::*;
use ::{game, Result, Error, ErrorKind};

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
#[derive(Debug, Clone)]
pub struct Builder {
    vision_addr: SocketAddr,
    grsim_addr: SocketAddr,
}

impl Builder {
    /// Instantiate with default values.
    pub fn new() -> Builder {
        Builder {
            vision_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(224, 5, 23, 2)), 10002),
            grsim_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 20011),
        }
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

    /// Set the grsim address and port used on the simulator, this is used when sending commands to
    /// grSim.
    pub fn grsim_addr<A: ToSocketAddrs>(&mut self, addrs: A) -> Result<&mut Builder> {
        self.grsim_addr = try!(addrs.to_single_socket_addr());
        Ok(self)
    }

    /// Set the grsim address to send commands to.  The port is preserved (default if not set).
    pub fn grsim_ip(&mut self, ip: IpAddr) -> &mut Builder {
        // TODO: use self.grsim_addr.set_ip when sockaddr_setters is stable
        self.grsim_addr = SocketAddr::new(ip, self.grsim_addr.port());
        self
    }

    /// Spawn the necessary threads and start listening to changes and ppushing commands.
    pub fn build(&self) -> Result<State> {
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

        let any_addr = Ipv4Addr::new(0, 0, 0, 0);
        let grsim_bind = SocketAddrV4::new(any_addr, 0);
        let grsim_addr = self.grsim_addr;
        let send_socket = try!(UdpSocket::bind(grsim_bind));

        info!("receiving from {}, sending to {}", vision_addr, grsim_addr);

        Ok(State {
            recv_socket: recv_socket,
            send_socket: send_socket,
            grsim_addr: grsim_addr,
            buf: [0; 1024],
            geometry: SSL_GeometryFieldSize::new(),
            detection: SSL_DetectionFrame::new(),
            counter: 0,
            _data: PersistentData {},
        })
    }
}

struct PersistentData {
}

pub struct State {
    recv_socket: UdpSocket,
    send_socket: UdpSocket,
    grsim_addr: SocketAddr,
    // 1KB buffer, packets are usually not greater than ~200 bytes
    buf: [u8; 1024],
    geometry: SSL_GeometryFieldSize,
    detection: SSL_DetectionFrame,
    counter: u64,
    _data: PersistentData,
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
        let &mut State { ref mut send_socket, ref grsim_addr, ..  } = self;

        let mut packet = grSim_Packet::new();
        {
            let commands = packet.mut_commands();
            commands.set_timestamp(0.0); // TODO/XXX/FIXME
            commands.set_isteamyellow(command.color.is_yellow());
            let robot_commands = commands.mut_robot_commands();

            for (robot_id, robot_command) in command.robots {
                use std::f32::consts::FRAC_1_SQRT_2;
                use game::RobotAction::*;

                let mut c = grSim_Robot_Command::new();
                c.set_id(robot_id as u32);
                c.set_wheelsspeed(false);
                c.set_veltangent(robot_command.v_tangent);
                c.set_velnormal(robot_command.v_normal);
                c.set_velangular(robot_command.v_angular);
                let (spinner, kickx, kickz) = match robot_command.action {
                    Normal => (false, 0.0, 0.0),
                    Dribble => (true, 0.0, 0.0),
                    Kick(force) => (false, force, 0.0),
                    ChipKick(force) => {
                        // XXX: hardcoded 45 degrees angled chip kick
                        let d_force = force * FRAC_1_SQRT_2;
                        (false, d_force, d_force)
                    }
                };
                c.set_spinner(spinner);
                c.set_kickspeedx(kickx);
                c.set_kickspeedz(kickz);

                robot_commands.push(c);
            }
        }
        let ref bytes = try!(packet.write_to_bytes());
        try!(send_socket.send_to(bytes, grsim_addr));

        Ok(())
    }
}

impl<'a> game::State<'a> for State {
    type Ball = &'a SSL_DetectionBall;
    type Robot = (Color, &'a SSL_DetectionRobot);
    type Robots = Iter<'a>;
    type Geometry = &'a SSL_GeometryFieldSize;
    type TeamInfo = ();

    fn counter(&self) -> u64 {
        self.counter
    }

    fn timestamp(&self) -> f64 {
        self.detection.get_t_capture()
    }

    fn ball(&'a self) -> Self::Ball {
        self.detection.get_balls().get(0).unwrap()
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

    fn team_info(&'a self, _color: Color) -> Self::TeamInfo { () }
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

impl<'a> game::Ball for &'a SSL_DetectionBall {
    fn pos(&self) -> Vec2d { Vec2d(self.get_x() / 1000.0, self.get_y() / 1000.0) }
    fn vel(&self) -> Vec2d { Vec2d(0.0, 0.0) } // TODO
}

impl<'a> game::Robot for (Color, &'a SSL_DetectionRobot) {
    fn id(&self) -> Id { Id(self.0, self.1.get_robot_id() as u8) }
    fn pos(&self) -> Vec2d { Vec2d(self.1.get_x() / 1000.0, self.1.get_y() / 1000.0) }
    fn vel(&self) -> Vec2d { Vec2d(0.0, 0.0) } // TODO
    fn w(&self) -> f32 { self.1.get_orientation() }
    fn vw(&self) -> f32 { 0.0 } // TODO
}

impl<'a> game::Geometry for &'a SSL_GeometryFieldSize {
    fn field_length(&self) -> f32 { self.get_field_length() as f32 / 1000.0 }
    fn field_width(&self) -> f32 { self.get_field_width() as f32 / 1000.0 }
    fn goal_width(&self) -> f32 { self.get_goal_width() as f32 / 1000.0 }
    fn center_circle_radius(&self) -> f32 { self.get_center_circle_radius() as f32 / 1000.0 }
    fn defense_radius(&self) -> f32 { self.get_defense_radius() as f32 / 1000.0 }
    fn defense_stretch(&self) -> f32 { self.get_defense_stretch() as f32 / 1000.0 }
}
