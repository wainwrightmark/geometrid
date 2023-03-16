use core::{
    fmt,
    ops::{Add, Deref, DerefMut},
};

use itertools::Itertools;
#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};

use crate::{
    center::{Center, HasCenter},
    corner::Corner,
    flip_axes::FlipAxes,
    prelude::DynamicVertex,
    quarter_turns::QuarterTurns,
    tile::Tile,
    vector::Vector,
};

#[must_use]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

    #[must_use]
    pub const fn const_add(&self, vector: &Vector) -> Self {
        Self(self.0.const_add(vector))
    }

    #[must_use]
    pub const fn get_vertex(&self, corner: &Corner) -> DynamicVertex {
        use Corner::*;
        let (x, y) = match corner {
            NorthWest => (self.0.x, self.0.y),
            NorthEast => (self.0.x + 1, self.0.y),
            SouthWest => (self.0.x, self.0.y + 1),
            SouthEast => (self.0.x + 1, self.0.y + 1),
        };

        DynamicVertex(Vector { x, y })
    }
}

impl<V: AsRef<Vector>> Add<V> for DynamicTile {
    type Output = Self;

    fn add(self, rhs: V) -> Self::Output {
        self.const_add(rhs.as_ref())
    }
}

impl HasCenter for DynamicTile {
    #[must_use]
    fn get_center(&self, scale: f32) -> Center {
        let x = scale * ((self.0.x as f32) + 0.5);
        let y = scale * ((self.0.y as f32) + 0.5);

        Center { x, y }
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
    use crate::tile::*;
    use itertools::Itertools;
    use serde_test::{assert_tokens, Token};

    #[test]
    pub fn test_rotate() {
        let tile: DynamicTile = Vector::new(-2, 3).into();
        assert_eq!(tile.rotate(QuarterTurns::One), Vector::new(-3, -2).into());
    }

    #[test]
    pub fn test_flip() {
        let tile: DynamicTile = Vector::new(-2, 3).into();
        assert_eq!(tile.flip(FlipAxes::Both), Vector::new(2, -3).into());
    }

    #[test]
    pub fn test_center() {
        let tile: DynamicTile = Vector::new(-2, 3).into();

        assert_eq!(tile.get_center(3.0), Center::new(-4.5, 10.5))
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
}
