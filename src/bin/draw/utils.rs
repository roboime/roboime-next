use roboime_next::prelude::*;
use roboime_next::game;

pub fn ball_model_matrix<B: game::Ball>(ball: &B) -> [[f32; 4]; 4] {
    use super::BALL_RADIUS;

    let pos = ball.pos();
    let x = pos.x();
    let y = pos.y();
    let z = ball.z().unwrap_or(BALL_RADIUS);
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [ x ,  y ,  z , 1.0]
    ]
}

pub fn robot_model_matrix<R: game::Robot>(robot: &R) -> [[f32; 4]; 4] {
    let pos = robot.pos();
    let x = pos.x();
    let y = pos.y();
    let w = robot.w();
    [
        [ w.cos(),  w.sin(), 0.0, 0.0],
        [-w.sin(),  w.cos(), 0.0, 0.0],
        [   0.0  ,    0.0  , 1.0, 0.0],
        [    x   ,     y   , 0.0, 1.0]
    ]
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
