use core::{
    fmt::{self},
    ops::{Add, Deref, DerefMut},
};

#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// A vertex in a dynamically sized 2d space
#[must_use]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(any(test, feature = "serde"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "serde"), serde(transparent))]
pub struct DynamicVertex(pub Vector);

impl Deref for DynamicVertex {
    type Target = Vector;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> AsRef<T> for DynamicVertex
where
    T: ?Sized,
    <DynamicVertex as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl DerefMut for DynamicVertex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vector> for DynamicVertex {
    fn from(val: Vector) -> Self {
        DynamicVertex(val)
    }
}

impl From<DynamicVertex> for Vector {
    fn from(val: DynamicVertex) -> Self {
        val.0
    }
}

impl fmt::Display for DynamicVertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.0.x, self.0.y)
    }
}

impl DynamicVertex {
    pub const fn flip(&self, axes: FlipAxes) -> Self {
        Self(self.0.flip(axes))
    }

    pub const fn const_add(&self, vector: &Vector) -> Self {
        Self(self.0.const_add(vector))
    }

    pub const fn get_tile(&self, corner: &Corner) -> DynamicTile {
        let (x, y) = match corner {
            Corner::NorthWest => (self.0.x - 1, self.0.y - 1),
            Corner::NorthEast => (self.0.x - 1, self.0.y),
            Corner::SouthWest => (self.0.x, self.0.y - 1),
            Corner::SouthEast => (self.0.x, self.0.y),
        };

        DynamicTile(Vector { x, y })
    }

    /// Get the nearest vertex to this center.
    /// Will round away from 0.0
    #[must_use]
    #[cfg(any(test, all(feature = "std", feature = "glam")))]
    pub fn from_center(center: &glam::f32::Vec2, scale: f32) -> Self {
        let x = center.x / scale;
        let y = center.y / scale;

        let x = x.round() as i8;
        let y = y.round() as i8;

        let vector = Vector { x, y };
        Self(vector)
    }
}

#[cfg(any(test, feature = "glam"))]
impl HasCenter for DynamicVertex {
    #[must_use]
    fn get_center(&self, scale: f32) -> glam::f32::Vec2 {
        let x = scale * (self.0.x as f32);
        let y = scale * (self.0.y as f32);

        glam::f32::Vec2 { x, y }
    }
}

impl<V: AsRef<Vector>> Add<V> for DynamicVertex {
    type Output = Self;

    fn add(self, rhs: V) -> Self::Output {
        self.const_add(rhs.as_ref())
    }
}

impl DynamicVertex {
    pub const fn rotate(&self, quarter_turns: QuarterTurns) -> Self {
        Self(self.0.rotate(quarter_turns))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_rotate() {
        let vertex: DynamicVertex = Vector::new(-2, 3).into();
        assert_eq!(vertex.rotate(QuarterTurns::One), Vector::new(-3, -2).into());
    }

    #[test]
    pub fn test_flip() {
        let vertex: DynamicVertex = Vector::new(-2, 3).into();
        assert_eq!(vertex.flip(FlipAxes::Both), Vector::new(2, -3).into());
    }

    #[test]
    pub fn test_center() {
        let vertex: DynamicVertex = Vector::new(-2, 3).into();

        assert_eq!(vertex.get_center(3.0), glam::f32::Vec2::new(-6.0, 9.0))
    }
    #[test]
    pub fn test_add() {
        let vertex: DynamicVertex = Vector::new(-2, 3).into();

        assert_eq!(vertex + Vector::new(1, 1), Vector::new(-1, 4).into());
    }

    #[test]
    pub fn test_into_tile() {
        let vertex: DynamicVertex = Vector::new(-2, 3).into();

        assert_eq!(
            vertex.get_tile(&Corner::NorthWest),
            Vector::new(-3, 2).into()
        );
        assert_eq!(
            vertex.get_tile(&Corner::NorthEast),
            Vector::new(-3, 3).into()
        );
        assert_eq!(
            vertex.get_tile(&Corner::SouthWest),
            Vector::new(-2, 2).into()
        );
        assert_eq!(
            vertex.get_tile(&Corner::SouthEast),
            Vector::new(-2, 3).into()
        );
    }

    #[test]
    pub fn test_from_center() {
        fn t(x: f32, y: f32, scale: f32, expected_x: i8, expected_y: i8) {
            let actual = DynamicVertex::from_center(&glam::f32::Vec2 { x, y }, scale);
            assert_eq!(
                DynamicVertex(Vector {
                    x: expected_x,
                    y: expected_y
                }),
                actual
            )
        }

        t(0., 0., 1.0, 0, 0);
        t(0.9, 0.9, 1.0, 1, 1);
        t(0.9, 0.9, 0.5, 2, 2);

        t(5., -4., 1., 5, -4);
        t(5., -4., 2., 3, -2);
    }
}
