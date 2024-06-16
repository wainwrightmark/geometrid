use core::{
    fmt,
    ops::{Add, Deref, DerefMut},
};

#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// A tile in a dynamically sized 2d space
#[must_use]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(any(test, feature = "serde"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "serde"), serde(transparent))]
pub struct DynamicTile(pub Vector);

impl Deref for DynamicTile {
    type Target = Vector;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> AsRef<T> for DynamicTile
where
    T: ?Sized,
    <DynamicTile as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl DerefMut for DynamicTile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vector> for DynamicTile {
    fn from(val: Vector) -> Self {
        DynamicTile(val)
    }
}

impl From<DynamicTile> for Vector {
    fn from(val: DynamicTile) -> Self {
        val.0
    }
}

impl fmt::Display for DynamicTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.0.x, self.0.y)
    }
}

impl DynamicTile {
    pub const fn flip(&self, axes: FlipAxes) -> Self {
        Self(self.0.flip(axes))
    }

    pub const fn const_add(&self, vector: &Vector) -> Self {
        Self(self.0.const_add(vector))
    }

    pub const fn get_vertex(&self, corner: &Corner) -> DynamicVertex {

        let (x, y) = match corner {
            Corner::NorthWest => (self.0.x, self.0.y),
            Corner::NorthEast => (self.0.x + 1, self.0.y),
            Corner::SouthWest => (self.0.x, self.0.y + 1),
            Corner::SouthEast => (self.0.x + 1, self.0.y + 1),
        };

        DynamicVertex(Vector { x, y })
    }

    /// Gets the nearest tile to this center
    #[must_use]
    #[cfg(any(test, all(feature = "std", feature = "glam")))]
    pub fn from_center(center: &glam::f32::Vec2, scale: f32) -> Self {
        let x = center.x / scale;
        let y = center.y / scale;

        let x = x.floor() as i8;
        let y = y.floor() as i8;

        let vector = Vector { x, y };
        Self(vector)
    }
}

impl<V: AsRef<Vector>> Add<V> for DynamicTile {
    type Output = Self;

    fn add(self, rhs: V) -> Self::Output {
        self.const_add(rhs.as_ref())
    }
}

#[cfg(any(test, feature = "glam"))]
impl HasCenter for DynamicTile {
    #[must_use]
    fn get_center(&self, scale: f32) -> glam::f32::Vec2 {
        let x = scale * ((self.0.x as f32) + 0.5);
        let y = scale * ((self.0.y as f32) + 0.5);

        glam::f32::Vec2 { x, y }
    }
}

impl DynamicTile {
    pub const fn rotate(&self, quarter_turns: QuarterTurns) -> Self {
        Self(self.0.rotate(quarter_turns))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_rotate() {
        let tile: DynamicTile = Vector::new(-2, 3).into();
        let rotated = tile.rotate(QuarterTurns::One);
        assert_eq!(rotated.x, -3);
        assert_eq!(rotated.y, -2);
        assert_eq!(rotated, Vector::new(-3, -2).into());
    }

    #[test]
    pub fn test_flip() {
        let tile: DynamicTile = Vector::new(-2, 3).into();
        assert_eq!(tile.flip(FlipAxes::Both), Vector::new(2, -3).into());
    }

    #[test]
    pub fn test_center() {
        let tile: DynamicTile = Vector::new(-2, 3).into();

        assert_eq!(tile.get_center(3.0), glam::f32::Vec2::new(-4.5, 10.5))
    }

    #[test]
    pub fn test_into_vector() {
        let tile: DynamicTile = Vector::new(-2, 3).into();

        assert_eq!(Vector::from(tile), Vector::new(-2, 3));
    }

    #[test]
    pub fn test_get_vertex() {
        let tile: DynamicTile = Vector::new(-2, 3).into();

        assert_eq!(
            tile.get_vertex(&Corner::NorthWest),
            Vector::new(-2, 3).into()
        );
        assert_eq!(
            tile.get_vertex(&Corner::NorthEast),
            Vector::new(-1, 3).into()
        );
        assert_eq!(
            tile.get_vertex(&Corner::SouthWest),
            Vector::new(-2, 4).into()
        );
        assert_eq!(
            tile.get_vertex(&Corner::SouthEast),
            Vector::new(-1, 4).into()
        );
    }

    #[test]
    pub fn test_from_center() {
        fn t(x: f32, y: f32, scale: f32, expected_x: i8, expected_y: i8) {
            let actual = DynamicTile::from_center(&glam::f32::Vec2 { x, y }, scale);
            assert_eq!(
                DynamicTile(Vector {
                    x: expected_x,
                    y: expected_y
                }),
                actual
            )
        }

        t(0., 0., 1.0, 0, 0);
        t(0.9, 0.9, 1.0, 0, 0);
        t(0.9, 0.9, 0.5, 1, 1);

        t(5., -4., 1., 5, -4);
        t(5., -4., 2., 2, -2);
    }
}
