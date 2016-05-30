pub use self::Side::{Left, Right};
pub use self::Color::{Yellow, Blue};

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
