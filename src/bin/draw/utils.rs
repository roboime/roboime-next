use roboime_next::prelude::*;
use roboime_next::game;
use rusttype::{Font, Scale, PositionedGlyph};

pub static IDENT: [[f32; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

pub static INVXY: [[f32; 4]; 4] = [
    [-1.0,  0.0, 0.0, 0.0],
    [ 0.0, -1.0, 0.0, 0.0],
    [ 0.0,  0.0, 1.0, 0.0],
    [ 0.0,  0.0, 0.0, 1.0],
];

pub fn xyz_matrix(x: f32, y: f32, z: f32) -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [ x ,  y ,  z , 1.0],
    ]
}

pub fn xyzw_matrix(x: f32, y: f32, z: f32, w: f32) -> [[f32; 4]; 4] {
    let (s, c) = w.sin_cos();
    [
        [ c ,  s , 0.0, 0.0],
        [-s ,  c , 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [ x ,  y ,  z , 1.0],
    ]
}

pub fn ball_model_matrix<B: game::Ball>(ball: &B) -> [[f32; 4]; 4] {
    use super::BALL_RADIUS;

    let pos = ball.pos();
    let x = pos.x();
    let y = pos.y();
    let z = ball.z().unwrap_or(BALL_RADIUS);
    xyz_matrix(x, y, z)
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
        [    x   ,     y   , 0.0, 1.0],
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

#[allow(dead_code)]
pub fn layout_paragraph<'a>(font: &'a Font, scale: Scale, width: u32, text: &str) -> Vec<PositionedGlyph<'a>> {
    use unicode_normalization::UnicodeNormalization;
    use rusttype::point;

    let mut result = Vec::new();
    let v_metrics = font.v_metrics(scale);
    let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
    let mut caret = point(0.0, v_metrics.ascent);
    let mut last_glyph_id = None;
    for c in text.nfc() {
        if c.is_control() {
            match c {
                '\r' => {
                    caret = point(0.0, caret.y + advance_height);
                }
                '\n' => {},
                _ => {}
            }
            continue;
        }
        let base_glyph = if let Some(glyph) = font.glyph(c) {
            glyph
        } else {
            continue;
        };
        if let Some(id) = last_glyph_id.take() {
            caret.x += font.pair_kerning(scale, id, base_glyph.id());
        }
        last_glyph_id = Some(base_glyph.id());
        let mut glyph = base_glyph.scaled(scale).positioned(caret);
        if let Some(bb) = glyph.pixel_bounding_box() {
            if bb.max.x > width as i32 {
                caret = point(0.0, caret.y + advance_height);
                glyph = glyph.into_unpositioned().positioned(caret);
                last_glyph_id = None;
            }
        }
        caret.x += glyph.unpositioned().h_metrics().advance_width;
        result.push(glyph);
    }
    result
}

pub fn layout_align<'a>(font: &'a Font, scale: Scale, text_left: &str, text_center: &str, text_right: &str) -> Vec<PositionedGlyph<'a>> {
    use unicode_normalization::UnicodeNormalization;
    use rusttype::point;

    let mut result = Vec::new();
    let mut caret = point(0.0, 0.0);
    let mut last_glyph_id = None;

    let offset = {
        let mut flush_text = |text: &str| {
            for c in text.nfc() {
                if c.is_control() { continue; }
                let base_glyph = if let Some(glyph) = font.glyph(c) { glyph } else { continue; };
                if let Some(id) = last_glyph_id.take() {
                    caret.x += font.pair_kerning(scale, id, base_glyph.id());
                }
                last_glyph_id = Some(base_glyph.id());
                let glyph = base_glyph.scaled(scale).positioned(caret);
                caret.x += glyph.unpositioned().h_metrics().advance_width;
                result.push(glyph);
            }
            caret
        };
        let caret = flush_text(text_left);
        let offset_left = -caret.x;
        let x0 = caret.x;
        let caret = flush_text(text_center);
        let x1 = caret.x;
        let offset_center = -(x1 - x0) * 0.5;
        flush_text(text_right);
        offset_left + offset_center
    };

    for glyph in result.iter_mut() {
        let g = glyph.clone();
        let mut position = g.position();
        position.x += offset;
        *glyph = g.into_unpositioned().positioned(position);
    }

    result
}
