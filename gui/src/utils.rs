use std::collections::BTreeMap;
use roboime_next::prelude::*;
use roboime_next::game::{Ball, Robot};

pub trait ModelMatrix {
    fn model_matrix(&self) -> [[f32; 4]; 4];
}

impl ModelMatrix for Ball {
    fn model_matrix(&self) -> [[f32; 4]; 4] {
        use ::models::BALL_RADIUS;

        let x = self.get_x();
        let y = self.get_y();
        let z = BALL_RADIUS;
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [ x ,  y ,  z , 1.0]
        ]
    }
}

impl ModelMatrix for Robot {
    fn model_matrix(&self) -> [[f32; 4]; 4] {
        let x = self.get_x();
        let y = self.get_y();
        let w = self.get_w();
        [
            [ w.cos(),  w.sin(), 0.0, 0.0],
            [-w.sin(),  w.cos(), 0.0, 0.0],
            [   0.0  ,    0.0  , 1.0, 0.0],
            [    x   ,     y   , 0.0, 1.0]
        ]
    }
}

pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

pub fn perspective_matrix(width: f32, height: f32) -> [[f32; 4]; 4] {
    const SCALE: f32 = 2000.0;
    let x_scale = -SCALE / width;
    let y_scale = SCALE / height;

    let zfar = 1024.0;
    let znear = 0.1;

    [
        [x_scale,   0.0  ,              0.0              ,   0.0],
        [  0.0  , y_scale,              0.0              ,   0.0],
        [  0.0  ,   0.0  ,  (zfar+znear)/(zfar-znear)    ,   1.0],
        [  0.0  ,   0.0  , -(2.0*zfar*znear)/(zfar-znear),   0.0],
    ]
}

pub fn add_initial_robots(robots: &mut BTreeMap<u8, Robot>, robot_count: u8, right_side: bool) {
    use std::f32::consts::PI;
    use ::models::*;

    let w_0 = if right_side { 0.0 } else { PI };
    let w_delta = 2.0 * PI / (robot_count as f32) * if right_side { 1.0 } else { -1.0 };
    let x_offset = (CENTER_DIAMETER / 4.0 + FIELD_LENGTH / 4.0 - DEFENSE_RADIUS / 2.0) * if right_side { 1.0 } else { -1.0 };
    //let radius = FIELD_LENGTH / 8.0;
    let radius = (FIELD_LENGTH / 2.0 - CENTER_DIAMETER / 2.0 - DEFENSE_RADIUS) / 3.0 - ROBOT_RADIUS;

    for i in 0..robot_count {
        let robot_id = i as u8;
        let w = w_0 + (i as f32) * w_delta;
        let mut robot = Robot::new(robot_id);
        robot.set_x(radius * w.cos() + x_offset);
        robot.set_y(radius * w.sin());
        robot.set_w(w + PI);
        robots.insert(robot_id, robot);
    }
}
