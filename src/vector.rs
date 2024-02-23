use core::{
    fmt::Debug,
    ops::{Add, Mul, Neg, Sub},
};

#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// A 2d vector.
/// The main use is that they can be added to any other primitive type.
#[must_use]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(any(test, feature = "serde"), derive(Serialize, Deserialize))]
pub struct Vector {
    pub x: i8,
    pub y: i8,
}

impl AsRef<Vector> for Vector {
    fn as_ref(&self) -> &Vector {
        self
    }
}

impl core::fmt::Display for Vector {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if *self == Vector::ZERO {
            write!(f, "Zero")
        } else if let Some(index) = Vector::UNITS.iter().position(|x| x == self) {
            let name = Vector::UNIT_NAMES[index];
            write!(f, "{name}")
        } else {
            f.debug_struct("Vector")
                .field("x", &self.x)
                .field("y", &self.y)
                .finish()
        }
    }
}

impl Vector {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const NORTH: Self = Self { x: 0, y: -1 };
    pub const NORTH_EAST: Self = Self { x: 1, y: -1 };
    pub const EAST: Self = Self { x: 1, y: 0 };
    pub const SOUTH_EAST: Self = Self { x: 1, y: 1 };
    pub const SOUTH: Self = Self { x: 0, y: 1 };
    pub const SOUTH_WEST: Self = Self { x: -1, y: 1 };
    pub const WEST: Self = Self { x: -1, y: 0 };
    pub const NORTH_WEST: Self = Self { x: -1, y: -1 };

    pub const CARDINALS: [Self; 4] = [Self::NORTH, Self::EAST, Self::SOUTH, Self::WEST];

    pub const UNITS: [Self; 8] = [
        Self::NORTH,
        Self::NORTH_EAST,
        Self::EAST,
        Self::SOUTH_EAST,
        Self::SOUTH,
        Self::SOUTH_WEST,
        Self::WEST,
        Self::NORTH_WEST,
    ];

    pub const UNIT_NAMES: [&'static str; 8] = [
        "North",
        "North East",
        "East",
        "South East",
        "South",
        "South West",
        "West",
        "North West",
    ];

    #[inline]
    pub const fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }

    pub const fn const_neg(&self) -> Self {
        self.const_mul(-1)
    }

    pub const fn const_mul(&self, rhs: i8) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }

    pub const fn const_add(&self, rhs: &Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }

    pub const fn const_sub(&self, rhs: &Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }

    pub const fn flip(&self, axes: FlipAxes) -> Self {
        match axes {
            FlipAxes::None => *self,
            FlipAxes::Horizontal => Self {
                x: -self.x,
                y: self.y,
            },
            FlipAxes::Vertical => Self {
                x: self.x,
                y: -self.y,
            },
            FlipAxes::Both => Self {
                x: -self.x,
                y: -self.y,
            },
        }
    }
    pub const fn rotate(&self, quarter_turns: QuarterTurns) -> Self {
        match quarter_turns {
            QuarterTurns::Zero => *self,
            QuarterTurns::One => Self::new(-self.y, self.x),
            QuarterTurns::Two => Self::new(-self.x, -self.y),
            QuarterTurns::Three => Self::new(self.y, -self.x),
        }
    }

    /// Returns true if this is the zero vector
    #[must_use]
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    /// Returns true if this is a unit vector
    #[must_use]
    #[inline]
    pub const fn is_unit(&self) -> bool {
        self.x.abs() <= 1 && self.y.abs() <= 1 && !self.is_zero()
    }

    /// Returns true if this is a diagonal vector, having both an x and a y component
    pub const fn is_diagonal(&self) -> bool {
        self.x != 0 && self.y != 0
    }

    pub const fn horizontal_component(&self) -> Self {
        Self { x: self.x, y: 0 }
    }

    pub const fn vertical_component(&self) -> Self {
        Self { x: 0, y: self.y }
    }
    /// Greater than operation that can be computed at compile time
    pub const fn const_gt(&self, other: &Self) -> bool {
        if self.x > other.x {
            return true;
        } else if self.x < other.x {
            return false;
        } else {
            self.y > other.y
        }
    }
}

impl Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.const_neg()
    }
}

impl Neg for &Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        self.const_neg()
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        self.const_add(&rhs)
    }
}

impl Add for &Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        self.const_add(rhs)
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        self.const_sub(&rhs)
    }
}

impl Sub for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        self.const_sub(rhs)
    }
}

impl Mul<i8> for Vector {
    type Output = Vector;

    fn mul(self, rhs: i8) -> Self::Output {
        self.const_mul(rhs)
    }
}

impl Mul<isize> for Vector {
    type Output = Vector;

    fn mul(self, rhs: isize) -> Self::Output {
        self.const_mul(rhs as i8)
    }
}

impl Mul<usize> for Vector {
    type Output = Vector;

    fn mul(self, rhs: usize) -> Self::Output {
        self.const_mul(rhs as i8)
    }
}

#[cfg(any(test, feature = "glam"))]
impl HasCenter for Vector {
    #[must_use]
    fn get_center(&self, scale: f32) -> glam::f32::Vec2 {
        let x = scale * (self.x as f32);
        let y = scale * (self.y as f32);

        glam::f32::Vec2 { x, y }
    }
}

#[cfg(test)]
mod tests {
    use core::f32::consts;

    use super::*;
    use crate::prelude::*;
    use itertools::Itertools;
    use serde_test::{assert_tokens, Token};
    type V = Vector;

    #[test]
    pub fn test_display() {
        assert_eq!("Zero", Vector::new(0, 0).to_string());
        assert_eq!("North", Vector::new(0, -1).to_string());
        assert_eq!("Vector { x: 0, y: -2 }", Vector::new(0, -2).to_string());
    }

    #[test]
    pub fn test_flip() {
        assert_eq!(V::NORTH_EAST.flip(FlipAxes::Vertical), V::SOUTH_EAST);
        assert_eq!(V::NORTH_EAST.flip(FlipAxes::Horizontal), V::NORTH_WEST);
        assert_eq!(V::NORTH_EAST.flip(FlipAxes::Both), V::SOUTH_WEST);
        assert_eq!(V::NORTH_EAST.flip(FlipAxes::None), V::NORTH_EAST);
    }

    #[test]
    pub fn test_rotate() {
        assert_eq!(V::NORTH.rotate(QuarterTurns::One), V::EAST);
        assert_eq!(V::NORTH.rotate(QuarterTurns::Two), V::SOUTH);
        assert_eq!(V::NORTH.rotate(QuarterTurns::Three), V::WEST);
        assert_eq!(V::NORTH.rotate(QuarterTurns::Zero), V::NORTH);

        assert_eq!(V::NORTH_WEST.rotate(QuarterTurns::One), V::NORTH_EAST);
        assert_eq!(V::NORTH_WEST.rotate(QuarterTurns::Two), V::SOUTH_EAST);
        assert_eq!(V::NORTH_WEST.rotate(QuarterTurns::Three), V::SOUTH_WEST);
        assert_eq!(V::NORTH_WEST.rotate(QuarterTurns::Zero), V::NORTH_WEST);
    }

    #[test]
    pub fn test_conditions() {
        assert_eq!(V::ZERO.is_zero(), true);
        assert_eq!(V::NORTH.is_zero(), false);

        assert_eq!(V::ZERO.is_unit(), false);
        assert_eq!(V::NORTH.is_unit(), true);
        assert_eq!((V::NORTH.const_mul(2)).is_unit(), false);

        assert_eq!(V::ZERO.is_diagonal(), false);
        assert_eq!(V::NORTH.is_diagonal(), false);
        assert_eq!(V::NORTH_EAST.is_diagonal(), true);
    }

    #[test]
    pub fn test_functions() {
        assert_eq!(V::NORTH.neg(), V::SOUTH);
        assert_eq!((&V::NORTH).neg(), V::SOUTH);

        assert_eq!(V::NORTH + V::EAST, V::NORTH_EAST);
        assert_eq!(&V::NORTH + &V::EAST, V::NORTH_EAST);

        assert_eq!(V::NORTH - V::EAST, V::NORTH_WEST);
        assert_eq!(&V::NORTH - &V::EAST, V::NORTH_WEST);

        assert_eq!(V::NORTH * -1i8, V::SOUTH);
        assert_eq!(V::NORTH * -1isize, V::SOUTH);
        assert_eq!(V::NORTH * 2usize, V::new(0, -2));
    }

    #[test]
    pub fn test_centre() {
        assert_eq!(
            V::SOUTH_WEST.get_center(2.0),
            glam::f32::Vec2::new(-2.0, 2.0)
        )
    }
}
