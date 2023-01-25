use core::{
    fmt::Debug,
    ops::{Add, Mul, Neg},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::prelude::*;

use num_traits::{PrimInt, Zero, Signed, One};

pub trait VectorInner: PrimInt + Neg + Zero + Signed + One + Into<f32> {
}

impl VectorInner for i8 {}
impl VectorInner for i16 {}

pub trait Vector:Copy +  Clone + Sized + Add<Output = Self> {
    type Inner: VectorInner;
    #[must_use]
    fn x(&self) -> Self::Inner;
    #[must_use]
    fn y(&self) -> Self::Inner;

    fn new(x: Self::Inner, y: Self::Inner) -> Self;

    const ZERO: Self;
    const UP: Self;
    const UP_RIGHT: Self;
    const RIGHT: Self;
    const DOWN_RIGHT: Self;
    const DOWN: Self;
    const DOWN_LEFT: Self;
    const LEFT: Self;
    const UP_LEFT: Self;

    const CARDINALS: [Self; 4] = [Self::UP, Self::RIGHT, Self::DOWN, Self::LEFT];

    const UNITS: [Self; 8] = [
        Self::UP,
        Self::UP_RIGHT,
        Self::RIGHT,
        Self::DOWN_RIGHT,
        Self::DOWN,
        Self::DOWN_LEFT,
        Self::LEFT,
        Self::UP_LEFT,
    ];

    const UNIT_NAMES: [&'static str; 8] = [
        "Up",
        "Up Right",
        "Right",
        "Down Right",
        "Down",
        "Down Left",
        "Left",
        "Up Left",
    ];

    /// Returns true if this is the zero vector
    #[must_use]
    #[inline]
    fn is_zero(&self) -> bool {
        self.x().is_zero() && self.y().is_zero()
    }

    /// Returns true if this is a unit vector
    #[must_use]
    #[inline]
    fn is_unit(&self) -> bool {
        self.x().abs() <= Self::Inner::one() && self.y().abs() <= Self::Inner::one() && !self.is_zero()
    }

    /// Returns true if this is a diagonal vector, having both an x and a y component
    fn is_diagonal(&self) -> bool {
        !self.x().is_zero() && !self.y().is_zero()
    }
}

impl Vector8{
    pub const fn const_mul(self, rhs: i8)-> Self{
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}



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

        impl Vector for $name {
            type Inner = $inner;
            const ZERO: Self = Self { x: 0, y: 0 };
            const UP: Self = Self { x: 0, y: -1 };
            const UP_RIGHT: Self = Self { x: 1, y: -1 };
            const RIGHT: Self = Self { x: 1, y: 0 };
            const DOWN_RIGHT: Self = Self { x: 1, y: 1 };
            const DOWN: Self = Self { x: 0, y: 1 };
            const DOWN_LEFT: Self = Self { x: -1, y: 1 };
            const LEFT: Self = Self { x: -1, y: 0 };
            const UP_LEFT: Self = Self { x: -1, y: -1 };
         #[inline]
            fn new(x: $inner, y: $inner) -> Self {
                Self { x, y }
            }

            #[inline]
            fn x(&self) -> $inner {
                self.x
            }

            #[inline]
            fn y(&self) -> $inner {
                self.y
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
                self.mul(rhs)
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

        impl Flippable for $name {
            fn flip_horizontal(&mut self) {
                self.x = self.x.neg()
            }
        
            fn flip_vertical(&mut self) {
                self.y = self.y.neg()
            }
        }

        impl Rotatable for $name{
            fn rotate(&mut self, quarter_turns: QuarterTurns) {
                *self = match quarter_turns{
                    QuarterTurns::Zero => {return;},
                    QuarterTurns::One => Self::new(self.y(), -self.x()),
                    QuarterTurns::Two => Self::new(-self.x(), -self.y()),
                    QuarterTurns::Three =>Self::new(-self.y(), self.x()),
                }
            }
        }

        impl HasLocation for $name{
            fn location(&self, scale: f32) -> Location {
                let x = Into::<f32>::into(self.x()) * scale; 
                let y = Into::<f32>::into(self.y()) * scale; 
        
                Location { x, y }
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
                let x = (self.col() as $relative_inner) + rhs.x();
                let y = (self.row() as $relative_inner) + rhs.y();
                if x >= 0 && y >= 0 {
                    Self::try_new(x as $absolute_inner, y as $absolute_inner)
                } else {
                    None
                }
            }
        }
    };
}

// macro_rules! adjacent_positions {
//     ($absolute_name:ident, $relative_name:ident, $absolute_inner:ty) => {

//         impl<const W: $absolute_inner, const H: $absolute_inner> $absolute_name<W, H> {
//             #[must_use]
//             pub fn get_adjacent_positions<'a>(&'a self) -> impl Iterator<Item = Self> + 'a {
//                 $relative_name:: UNITS.iter().filter_map(|x| self.add(*x))
//             }
//         }
//     };
// }


vector!(Vector16, i16);
vector!(Vector8, i8);

point_add!(Tile16, Vector16, u16, i16);
point_add!(Tile16, Vector8, u16, i8);

point_add!(Tile8, Vector8, u8, i8);

// // adjacent_positions!(Tile64, Vector64, u64);
// // adjacent_positions!(Tile32, Vector32, u32);
// // adjacent_positions!(Tile16, Vector16, u16);
// // adjacent_positions!(Tile8, Vector8, u8);
