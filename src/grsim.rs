use std::net::{IpAddr, SocketAddr, UdpSocket, Ipv4Addr, SocketAddrV4, ToSocketAddrs};
use net2::{UdpBuilder, UdpSocketExt};
use protocol::{Message, parse_from_bytes};
use protocol::messages_robocup_ssl_wrapper_legacy::SSL_WrapperPacket;
use protocol::messages_robocup_ssl_detection::SSL_DetectionFrame;
use protocol::messages_robocup_ssl_geometry_legacy::SSL_GeometryData;
use protocol::grSim_Packet::grSim_Packet;
use protocol::grSim_Commands::grSim_Robot_Command;
use ::prelude::*;
use ::{game, Result, Error, ErrorKind};

impl game::State {
    fn update_from_ssl_geometry(&mut self, geometry: &SSL_GeometryData) {
        let geometry = geometry.get_field();
        let mut geom = self.get_field_geom_mut();

        fn m(x: i32) -> f32 { (x as f32) / 1000.0 }

        geom.field_length = m(geometry.get_field_length());
        geom.field_width = m(geometry.get_field_width());
        geom.goal_width = m(geometry.get_goal_width());
        geom.center_circle_radius = m(geometry.get_center_circle_radius());
        geom.defense_radius = m(geometry.get_defense_radius());
        geom.defense_stretch = m(geometry.get_defense_stretch());
        //geom.free_kick_from_defense_dist = m(geometry.get_free_kick_from_defense_dist());
        geom.penalty_spot_from_field_line_dist = m(geometry.get_penalty_spot_from_field_line_dist());
        geom.penalty_line_from_spot_dist = m(geometry.get_penalty_line_from_spot_dist());
    }

    fn update_from_ssl_detection(&mut self, detection: &SSL_DetectionFrame) {
        let timestamp = detection.get_t_capture();
        let dt = timestamp - self.get_timestamp();
        self.set_timestamp(timestamp);

        fn m(x: f32) -> f32 { x / 1000.0 }

        // TODO: select ball
        for ball in detection.get_balls() {
            let ball_state = self.get_ball_mut();
            ball_state.update_position(m(ball.get_x()), m(ball.get_y()), dt);
        }

        // TODO: remove missing robots
        {
            let blue_robots = self.get_robots_blue_mut();
            for robot in detection.get_robots_blue() {
                let id = robot.get_robot_id() as u8;
                let robot_state = blue_robots.entry(id).or_insert_with(|| game::Robot::new(id));
                robot_state.update_pose(m(robot.get_x()), m(robot.get_y()), robot.get_orientation(), dt);
            }
        }
        {
            let yellow_robots = self.get_robots_yellow_mut();
            for robot in detection.get_robots_yellow() {
                let id = robot.get_robot_id() as u8;
                let robot_state = yellow_robots.entry(id).or_insert_with(|| game::Robot::new(id));
                robot_state.update_pose(m(robot.get_x()), m(robot.get_y()), robot.get_orientation(), dt);
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
pub struct Interface {
    vision_addr: SocketAddr,
    grsim_addr: SocketAddr,
}

impl Interface {
    /// Instantiate with default values.
    pub fn new() -> Interface {
        Interface {
            vision_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(224, 5, 23, 2)), 10002),
            grsim_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 20011),
        }
    }

    /// Set the vision address and port used on the simulator, the address is used for
    /// joining the multicast and the port for binding that socket.
    pub fn vision_addr<A: ToSocketAddrs>(&mut self, addrs: A) -> Result<&mut Interface> {
        self.vision_addr = try!(addrs.to_single_socket_addr());
        Ok(self)
    }

    /// Set the vision port used on the simulator used for binding the socket.
    pub fn vision_port(&mut self, port: u16) -> &mut Interface {
        // TODO: use self.vision_addr.set_port when sockaddr_setters is stable
        self.vision_addr = SocketAddr::new(self.vision_addr.ip(), port);
        self
    }

    /// Set the grsim address and port used on the simulator, this is used when sending commands to
    /// grSim.
    pub fn grsim_addr<A: ToSocketAddrs>(&mut self, addrs: A) -> Result<&mut Interface> {
        self.grsim_addr = try!(addrs.to_single_socket_addr());
        Ok(self)
    }

    /// Set the grsim address to send commands to.  The port is preserved (default if not set).
    pub fn grsim_ip(&mut self, ip: IpAddr) -> &mut Interface {
        // TODO: use self.grsim_addr.set_ip when sockaddr_setters is stable
        self.grsim_addr = SocketAddr::new(ip, self.grsim_addr.port());
        self
    }

    /// Spawn the necessary threads and start listening to changes and ppushing commands.
    pub fn spawn(&self) -> Result<GrSimHandle> {
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

        Ok(GrSimHandle {
            recv_socket: recv_socket,
            send_socket: send_socket,
            grsim_addr: grsim_addr,
            buf: [0; 1024],
        })
    }
}

pub struct GrSimHandle {
    recv_socket: UdpSocket,
    send_socket: UdpSocket,
    grsim_addr: SocketAddr,
    // 1KB buffer, packets are usually not greater than ~200 bytes
    buf: [u8; 1024],
}

impl GrSimHandle {
    /// Returns Ok(true) if there was a geometry update.
    pub fn recv_to_state(&mut self, state: &mut game::State) -> Result<()> {
        let &mut GrSimHandle { ref mut buf, ref recv_socket, ..  } = self;

        let packet = try!(recv_socket.recv_ssl_packet(buf));

        if packet.has_geometry() {
            state.update_from_ssl_geometry(packet.get_geometry());
        }
        if packet.has_detection() {
            state.update_from_ssl_detection(packet.get_detection());
            state.inc_counter();
        }

        Ok(())
    }

    pub fn wait_for_geom(&mut self, state: &mut game::State) -> Result<()> {
        let &mut GrSimHandle { ref mut buf, ref recv_socket, ..  } = self;

        loop {
            let packet = try!(recv_socket.recv_ssl_packet(buf));

            if packet.has_geometry() {
                state.update_from_ssl_geometry(packet.get_geometry());
                break;
            }
        }

        Ok(())
    }

    pub fn send_command(&mut self, command: game::Command) -> Result<()> {
        let &mut GrSimHandle { ref mut send_socket, ref grsim_addr, ..  } = self;

        let mut packet = grSim_Packet::new();
        {
            let commands = packet.mut_commands();
            commands.set_timestamp(0.0); // TODO/XXX/FIXME
            commands.set_isteamyellow(command.is_yellow);
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

impl InterfaceHandle for GrSimHandle {
    fn join(self) -> Result<()> {
        Ok(())
    }
}
