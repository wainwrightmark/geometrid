use core::{
    fmt::Debug,
    ops::{Add, Mul, Neg},
};

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

use crate::point_absolute::*;

macro_rules! vector {
    ($name:ident, $inner:ty) => {
        #[must_use]
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct $name {
            x: $inner,
            y: $inner,
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if *self == $name::ZERO {
                    write!(f, "Zero")
                } else if let Some(index) = $name::UNITS.iter().position(|x| x == self) {
                    let name = $name::UNIT_NAMES[index];
                    write!(f, "{name}")
                } else {
                    f.debug_struct("$name")
                        .field("x", &self.x)
                        .field("y", &self.y)
                        .finish()
                }
            }
        }

        impl $name {
            pub const ZERO: Self = Self { x: 0, y: 0 };
            pub const UP: Self = Self { x: 0, y: -1 };
            pub const UP_RIGHT: Self = Self { x: 1, y: -1 };
            pub const RIGHT: Self = Self { x: 1, y: 0 };
            pub const DOWN_RIGHT: Self = Self { x: 1, y: 1 };
            pub const DOWN: Self = Self { x: 0, y: 1 };
            pub const DOWN_LEFT: Self = Self { x: -1, y: 1 };
            pub const LEFT: Self = Self { x: -1, y: 0 };
            pub const UP_LEFT: Self = Self { x: -1, y: -1 };

            pub const CARDINALS: [Self; 4] = [Self::UP, Self::RIGHT, Self::DOWN, Self::LEFT];
            pub const UNITS: [Self; 8] = [
                Self::UP,
                Self::UP_RIGHT,
                Self::RIGHT,
                Self::DOWN_RIGHT,
                Self::DOWN,
                Self::DOWN_LEFT,
                Self::LEFT,
                Self::UP_LEFT,
            ];

            pub const UNIT_NAMES: [&'static str; 8] = [
                "Up",
                "Up Right",
                "Right",
                "Down Right",
                "Down",
                "Down Left",
                "Left",
                "Up Left",
            ];

            #[must_use]
            #[inline]
            pub const fn new(x: $inner, y: $inner) -> Self {
                Self { x, y }
            }

            #[must_use]
            #[inline]
            pub const fn x(&self) -> $inner {
                self.x
            }

            #[must_use]
            #[inline]
            pub const fn y(&self) -> $inner {
                self.y
            }

            #[must_use]
            #[inline]
            pub const fn is_zero(&self) -> bool {
                self.x == 0 && self.y == 0
            }

            #[must_use]
            #[inline]
            pub const fn is_unit(&self) -> bool {
                self.x.abs() <= 1 && self.y.abs() <= 1 && !self.is_zero()
            }

            #[must_use]
            #[inline]
            pub const fn is_diagonal(&self) -> bool {
                self.x != 0 && self.y != 0
            }

            #[must_use]
            #[inline]
            pub const fn flip_horizontal(&self) -> Self {
                Self {
                    x: -self.x,
                    y: self.y,
                }
            }

            #[must_use]
            #[inline]
            pub const fn flip_vertical(&self) -> Self {
                Self {
                    x: self.x,
                    y: -self.y,
                }
            }

            #[must_use]
            #[inline]
            pub const fn rotate(&self, quarter_turns: u8) -> Self {
                match quarter_turns % 4 {
                    1 => Self::new(self.y(), -self.x()),
                    2 => Self::new(-self.x(), -self.y()),
                    3 => Self::new(-self.y(), self.x()),
                    _ => *self,
                }
            }

            #[must_use]
            #[inline]
            pub const fn const_mul(self, rhs: isize) -> Self {
                Self {
                    x: self.x * (rhs as $inner),
                    y: self.y * (rhs as $inner),
                }
            }
        }

        impl Neg for $name {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self {
                    x: -self.x,
                    y: -self.y,
                }
            }
        }

        impl Neg for &$name {
            type Output = $name;
            fn neg(self) -> Self::Output {
                Self::Output {
                    x: -self.x,
                    y: -self.y,
                }
            }
        }

        impl Add for $name {
            type Output = $name;
            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl Add for &$name {
            type Output = $name;

            fn add(self, rhs: Self) -> Self::Output {
                $name {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl Mul<$inner> for $name {
            type Output = $name;

            fn mul(self, rhs: $inner) -> Self::Output {
                Self {
                    x: self.x * rhs,
                    y: self.y * rhs,
                }
            }
        }

        impl Mul<isize> for $name {
            type Output = $name;

            fn mul(self, rhs: isize) -> Self::Output {
                self.const_mul(rhs)
            }
        }

        impl Mul<usize> for $name {
            type Output = $name;

            fn mul(self, rhs: usize) -> Self::Output {
                Self {
                    x: self.x * (rhs as $inner),
                    y: self.y * (rhs as $inner),
                }
            }
        }
    };
}

macro_rules! point_add {
    ($absolute_name:ident, $relative_name:ident, $absolute_inner:ty, $relative_inner:ty) => {
        impl<const W: $absolute_inner, const H: $absolute_inner> Add<$relative_name>
            for $absolute_name<W, H>
        {
            type Output = Option<Self>;

            fn add(self, rhs: $relative_name) -> Self::Output {
                let x = (self.x() as $relative_inner) + rhs.x();
                let y = (self.y() as $relative_inner) + rhs.y();
                if x >= 0 && y >= 0 {
                    Self::try_new(x as $absolute_inner, y as $absolute_inner)
                } else {
                    None
                }
            }
        }
    };
}

macro_rules! adjacent_positions {
    ($absolute_name:ident, $relative_name:ident, $absolute_inner:ty) => {

        impl<const W: $absolute_inner, const H: $absolute_inner> $absolute_name<W, H> {
            #[must_use]
            pub fn get_adjacent_positions<'a>(&'a self) -> impl Iterator<Item = Self> + 'a {
                $relative_name:: UNITS.iter().filter_map(|x| self.add(*x))
            }
        }
    };
}

vector!(Vector64, i64);
vector!(Vector32, i32);
vector!(Vector16, i16);
vector!(Vector8, i8);

point_add!(PointAbsolute64, Vector64, u64, i64);
point_add!(PointAbsolute64, Vector32, u64, i32);
point_add!(PointAbsolute64, Vector16, u64, i16);
point_add!(PointAbsolute64, Vector8, u64, i8);


point_add!(PointAbsolute32, Vector32, u32, i32);
point_add!(PointAbsolute32, Vector16, u32, i16);
point_add!(PointAbsolute32, Vector8, u32, i8);

point_add!(PointAbsolute16, Vector16, u16, i16);
point_add!(PointAbsolute16, Vector8, u16, i8);

point_add!(PointAbsolute8, Vector8, u8, i8);


adjacent_positions!(PointAbsolute64, Vector64, u64);
adjacent_positions!(PointAbsolute32, Vector32, u32);
adjacent_positions!(PointAbsolute16, Vector16, u16);
adjacent_positions!(PointAbsolute8, Vector8, u8);