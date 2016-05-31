pub use self::Side::{Left, Right};
pub use self::Color::{Yellow, Blue};

use std::ops::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2d(pub f32, pub f32);

impl Vec2d {
    #[inline] pub fn set(&mut self, x: f32, y: f32) { self.0 = x; self.1 = y; }
    #[inline] pub fn angle(self) -> f32 { self.1.atan2(self.0) }
    #[inline] pub fn norm(self) -> f32 { self.norm2().sqrt() }
    #[inline] pub fn norm2(self) -> f32 { self.0 * self.0 + self.1 * self.1 }
    #[inline] pub fn rotate(self, w: f32) -> Vec2d {
        let (sin, cos) = w.sin_cos();
        Vec2d(self.0 * cos - self.1 * sin, self.0 * sin + self.1 * cos)
    }
}

impl Add for Vec2d {
    type Output = Vec2d;
    #[inline]
    fn add(self, rhs: Vec2d) -> Vec2d {
        Vec2d(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vec2d {
    #[inline]
    fn add_assign(&mut self, rhs: Vec2d) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for Vec2d {
    type Output = Vec2d;
    #[inline]
    fn sub(self, rhs: Vec2d) -> Vec2d {
        Vec2d(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for Vec2d {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec2d) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Mul for Vec2d {
    type Output = f32;
    #[inline]
    fn mul(self, rhs: Vec2d) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1
    }
}

impl Mul<f32> for Vec2d {
    type Output = Vec2d;
    #[inline]
    fn mul(self, rhs: f32) -> Vec2d {
        Vec2d(self.0 * rhs, self.1 * rhs)
    }
}

impl MulAssign<f32> for Vec2d {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

impl Div<f32> for Vec2d {
    type Output = Vec2d;
    #[inline]
    fn div(self, rhs: f32) -> Vec2d {
        Vec2d(self.0 / rhs, self.1 / rhs)
    }
}

impl DivAssign<f32> for Vec2d {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
        self.1 /= rhs;
    }
}

impl Neg for Vec2d {
    type Output = Vec2d;
    #[inline]
    fn neg(self) -> Vec2d {
        Vec2d(-self.0, -self.1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Yellow,
    Blue,
}

impl Color {
    #[inline] pub fn is_yellow(&self) -> bool { *self == Yellow }
    #[inline] pub fn is_blue(&self)   -> bool { *self == Blue }
    #[inline] pub fn yellow(is_yellow: bool) -> Color { if is_yellow { Yellow } else { Blue   } }
    #[inline] pub fn blue(is_blue: bool)     -> Color { if is_blue   { Blue   } else { Yellow } }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Side {
    Left,
    Right
}

impl Side {
    #[inline] pub fn is_right(&self) -> bool { *self == Right }
    #[inline] pub fn is_left(&self)  -> bool { *self == Left }
    #[inline] pub fn right(is_right: bool) -> Side { if is_right { Right } else { Left  } }
    #[inline] pub fn left(is_left: bool)   -> Side { if is_left  { Left  } else { Right } }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TeamSide(pub Color, pub Side);

impl TeamSide {
    #[inline]
    pub fn yellow_is_left(&self) -> bool {
        match *self {
            TeamSide(Yellow, Left)  => true,
            TeamSide(Yellow, Right) => false,
            TeamSide(Blue,   Right) => true,
            TeamSide(Blue,   Left)  => false,
        }
    }
}
