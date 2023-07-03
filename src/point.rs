#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};

use core::ops::{Add, Mul, Sub};

/// A point in 2d space.
/// As this is a floating point value, it can represent points like the center of a tile.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(any(test, feature = "serde"), derive(Serialize, Deserialize))]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Point {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

pub trait HasCenter {
    /// Get the `Point` at the centre of this
    fn get_center(&self, scale: f32) -> Point;
}

impl Point {
    /// Create a new `Point`
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// The square of length of the line from the origin to this `Point`.
    /// Cheaper to calculate than `length` and does not require `std`
    #[inline]
    pub fn square_length(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// The length of the line from the origin to this `Point`.
    /// Requires `std`
    #[cfg(any(test, feature = "std"))]
    #[inline]
    pub fn length(&self)-> f32{
        self.square_length().sqrt()
    }

    /// Linearly interpolate between this point and another
    #[inline]
    pub fn lerp(self, other: Self, t: f32) -> Self
    {
        let one_minus_t = 1.0 - t;
        Self::new(one_minus_t * self.x + t * other.x, one_minus_t * self.y + t * other.y)
    }

    /// Computes the `z` component of the cross product with another `Point`
    #[inline]
    pub fn cross_product(self, other: Self) -> f32
    {
        self.x * other.y - self.y * other.x
    }

    #[cfg(any(test, feature = "std"))]
    #[inline]
    #[must_use]
    /// The absolute distance to the other `Point`
    /// Requires `std`
    pub fn distance_to(&self, other: &Self) -> f32 {
        let dx: f32 = (self.x - other.x).abs();
        let dy: f32 = (self.y - other.y).abs();
        f32::sqrt((dx * dx) + (dy * dy))
    }

    #[cfg(any(test, feature = "std"))]
    #[inline]
    #[must_use]
    /// The angle to the other `Point`, in radians
    /// Requires `std`
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
        let a = Point::new(1., 2.);
        let b = Point::new(3., 4.);
        assert_eq!(a + b, Point::new(4., 6.));
    }
    
    #[test]
    pub fn test_sub() {
        let a = Point::new(3., 5.);
        let b = Point::new(2., 3.);
        assert_eq!(a - b, Point::new(1., 2.));
    }
    
    #[test]
    pub fn test_cross() {
        let a = Point::new(3., 5.);
        let b = Point::new(2., 3.);
        assert_eq!(a.cross_product(b), -1.);
    }
    
    #[test]
    pub fn test_lerp() {
        let a = Point::new(3., 5.);
        let b = Point::new(2., 3.);
        assert_eq!(a.lerp(b, 0.5), Point::new(2.5, 4.0));
    }
    
    #[test]
    pub fn test_length() {
        let a = Point::new(3., 4.);
        assert_eq!(a.length(), 5.);
    }
    
    #[test]
    pub fn test_square_length() {
        let a = Point::new(3., 4.);
        assert_eq!(a.square_length(), 25.);
    }
    


    #[test]
    pub fn test_mul() {
        let a = Point::new(-1., 2.);
        assert_eq!(a * -2., Point::new(2., -4.))
    }

    #[test]
    pub fn test_distance_to() {
        let a = Point::new(2., 2.);
        let b = Point::new(1., 3.);
        assert_eq!(a.distance_to(&b), f32::sqrt(2.))
    }

    #[test]
    pub fn test_angle() {
        let a = Point::new(2., 2.);
        let b = Point::new(1., 3.);
        assert_eq!(a.angle_to(&b), consts::PI * 0.75)
    }

    #[test]
    pub fn test_angle_down(){
        let a = Point::new(0.,0.);
        let b = Point::new(0., 1.);

        assert_eq!(a.angle_to(&b), consts::PI* 0.5)
    }
}
