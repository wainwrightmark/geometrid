#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};

use core::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(any(test, feature = "serde"), derive(Serialize, Deserialize))]
pub struct Center {
    pub x: f32,
    pub y: f32,
}

impl Add for Center {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Center {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<f32> for Center {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Center {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

pub trait HasCenter {
    fn get_center(&self, scale: f32) -> Center;
}

impl Center {
    /// Create a new center
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[cfg(any(test, feature = "std"))]
    #[inline]
    #[must_use]
    /// The absolute distance to the other point
    /// Requires std
    pub fn distance(&self, other: &Self) -> f32 {
        let dx: f32 = (self.x - other.x).abs();
        let dy: f32 = (self.y - other.y).abs();
        f32::sqrt((dx * dx) + (dy * dy))
    }

    #[cfg(any(test, feature = "std"))]
    #[inline]
    #[must_use]
    /// The angle to the other point, in radians
    /// Requires std
    pub fn angle_to(&self, other: &Self) -> f32 {
        let x_diff = other.x - self.x;
        let y_diff = other.y - self.y;

        (y_diff).atan2(x_diff)
    }
}

#[cfg(test)]
mod tests {
    use core::f32::consts;

    use super::*;
    use crate::prelude::*;
    use itertools::Itertools;
    use serde_test::{assert_tokens, Token};

    #[test]
    pub fn test_add() {
        let a = Center::new(2., 2.);
        let b = Center::new(1., 3.);
        assert_eq!(a + b, Center::new(3., 5.));
    }

    #[test]
    pub fn test_mul() {
        let a = Center::new(-1., 2.);
        assert_eq!(a * -2., Center::new(2., -4.))
    }

    #[test]
    pub fn test_distance() {
        let a = Center::new(2., 2.);
        let b = Center::new(1., 3.);
        assert_eq!(a.distance(&b), f32::sqrt(2.))
    }

    #[test]
    pub fn test_angle() {
        let a = Center::new(2., 2.);
        let b = Center::new(1., 3.);
        assert_eq!(a.angle_to(&b), consts::PI * 0.75)
    }
}
