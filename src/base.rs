pub use self::Side::{Left, Right};
pub use self::Color::{Yellow, Blue};
pub use self::TeamSide::{YellowIsLeft, BlueIsLeft};

use std::ops::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2d(pub f32, pub f32);

impl Vec2d {
    #[inline] pub fn x(&self) -> f32 { self.0 }
    #[inline] pub fn y(&self) -> f32 { self.1 }
    #[inline] pub fn set(&mut self, x: f32, y: f32) { self.0 = x; self.1 = y; }
    #[inline] pub fn angle(self) -> f32 { self.1.atan2(self.0) }
    #[inline] pub fn norm(self) -> f32 { self.norm2().sqrt() }
    #[inline] pub fn norm2(self) -> f32 { self.0 * self.0 + self.1 * self.1 }
    #[inline] pub fn rotate(self, angle: f32) -> Vec2d {
        let (sin, cos) = angle.sin_cos();
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

impl Default for Color {
    fn default() -> Self { Yellow }
}

impl Not for Color {
    type Output = Color;
    #[inline]
    fn not(self) -> Color {
        match self {
            Yellow => Blue,
            Blue => Yellow,
        }
    }
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
pub enum TeamSide {
    YellowIsLeft,
    BlueIsLeft,
}

impl Default for TeamSide {
    fn default() -> Self { YellowIsLeft }
}

impl TeamSide {
    #[inline] pub fn new(color: Color, side: Side) -> TeamSide {
        match (color, side) {
            (Blue,   Left)  => BlueIsLeft,
            (Blue,   Right) => YellowIsLeft,
            (Yellow, Left)  => YellowIsLeft,
            (Yellow, Right) => BlueIsLeft,
        }
    }
    #[inline] pub fn blue_is_left(self)    -> bool { self == BlueIsLeft }
    #[inline] pub fn blue_is_right(self)   -> bool { self == YellowIsLeft }
    #[inline] pub fn yellow_is_left(self)  -> bool { self == YellowIsLeft }
    #[inline] pub fn yellow_is_right(self) -> bool { self == BlueIsLeft }
    #[inline] pub fn side(self, color: Color) -> Side {
        match (self, color) {
            (BlueIsLeft,   Blue)   => Left,
            (BlueIsLeft,   Yellow) => Right,
            (YellowIsLeft, Blue)   => Right,
            (YellowIsLeft, Yellow) => Left,
        }
    }
    #[inline] pub fn color(self, side: Side) -> Color {
        match (self, side) {
            (BlueIsLeft,   Left)  => Blue,
            (BlueIsLeft,   Right) => Yellow,
            (YellowIsLeft, Right) => Blue,
            (YellowIsLeft, Left)  => Yellow,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id(pub Color, pub u8);

impl Id {
    pub fn color(self) -> Color { self.0 }
    pub fn id(self) -> u8 { self.1 }
}
