use core::{fmt, iter::FusedIterator, ops::Add};

#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// A vertex in 2d space
#[must_use]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(any(test, feature = "serde"), derive(Serialize, Deserialize))]
pub struct Vertex<const WIDTH: u8, const HEIGHT: u8>(u8);

impl<const WIDTH: u8, const HEIGHT: u8> From<Vertex<WIDTH, HEIGHT>> for DynamicVertex {
    #[allow(clippy::cast_possible_wrap)]
    fn from(val: Vertex<WIDTH, HEIGHT>) -> Self {
        DynamicVertex(Vector {
            x: val.x() as i8,
            y: val.y() as i8,
        })
    }
}

impl<const WIDTH: u8, const HEIGHT: u8, V: AsRef<Vector>> Add<V> for Vertex<WIDTH, HEIGHT> {
    type Output = Option<Self>;

    fn add(self, rhs: V) -> Self::Output {
        self.const_add(rhs.as_ref())
    }
}

impl<const W: u8, const H: u8> fmt::Display for Vertex<W, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x(), self.y())
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> Vertex<WIDTH, HEIGHT> {
    const COLUMNS: u8 = WIDTH;
    const HEIGHT: u8 = HEIGHT;
    pub const COUNT: usize = (WIDTH + 1) as usize * (HEIGHT + 1) as usize;

    pub const NORTH_WEST: Self = Self(0);
    pub const NORTH_EAST: Self = Self::new_unchecked(Self::MAX_COL, 0);
    pub const SOUTH_WEST: Self = Self::new_unchecked(0, Self::MAX_ROW);
    pub const SOUTH_EAST: Self = Self::new_unchecked(Self::MAX_COL, Self::MAX_ROW);

    pub const CENTER: Self = Self::new_unchecked(WIDTH / 2, HEIGHT / 2);

    const MAX_COL: u8 = WIDTH;
    const MAX_ROW: u8 = HEIGHT;

    pub const fn new_const<const X: u8, const Y: u8>() -> Self {
        Self::new_unchecked(X, Y)
    }

    #[inline]
    pub(crate) const fn new_unchecked(x: u8, y: u8) -> Self {
        debug_assert!(x <= Self::COLUMNS);
        debug_assert!(y <= Self::HEIGHT);
        debug_assert!(Self::COUNT <= u8::MAX as usize);
        Self(x + ((Self::COLUMNS + 1) * y))
    }

    #[must_use]
    pub const fn inner(&self) -> u8 {
        self.0
    }

    #[must_use]
    pub const fn try_from_inner(inner: u8) -> Option<Self> {
        if inner <= Self::SOUTH_EAST.inner() {
            Some(Self(inner))
        } else {
            None
        }
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn try_from_usize(value: usize) -> Option<Self> {
        if value >= Self::COUNT {
            return None;
        }
        let inner = value as u8;
        Some(Self(inner))
    }

    #[must_use]
    pub const fn try_new(x: u8, y: u8) -> Option<Self> {
        if x > WIDTH {
            return None;
        }
        if y > HEIGHT {
            return None;
        }

        let Some(i1) = y.checked_mul(WIDTH + 1) else {
            return None;
        };
        let Some(i2) = i1.checked_add(x) else {
            return None;
        };

        Self::try_from_inner(i2)
    }

    #[must_use]
    pub const fn try_from_dynamic(dynamic_vertex: DynamicVertex) -> Option<Self> {
        if dynamic_vertex.0.x.is_negative() || dynamic_vertex.0.y.is_negative() {
            return None;
        }

        Self::try_new(
            dynamic_vertex.0.x.unsigned_abs(),
            dynamic_vertex.0.y.unsigned_abs(),
        )
    }

    #[must_use]
    pub const fn x(&self) -> u8 {
        self.0 % (Self::COLUMNS + 1)
    }

    #[must_use]
    pub const fn y(&self) -> u8 {
        self.0 / (Self::COLUMNS + 1)
    }

    pub const fn flip(&self, axes: FlipAxes) -> Self {
        match axes {
            FlipAxes::None => *self,
            FlipAxes::Horizontal => Self::new_unchecked(Self::MAX_COL - self.x(), self.y()),
            FlipAxes::Vertical => Self::new_unchecked(self.x(), Self::MAX_ROW - self.y()),
            FlipAxes::Both => {
                Self::new_unchecked(Self::MAX_COL - self.x(), Self::MAX_ROW - self.y())
            }
        }
    }

    #[must_use]
    pub const fn try_next(&self) -> Option<Self> {
        let Some(next) = self.inner().checked_add(1) else {
            return None;
        };
        Self::try_from_inner(next)
    }

    #[must_use]
    pub fn iter_by_row() -> impl FusedIterator<Item = Self> + ExactSizeIterator + Clone {
        ((Self::NORTH_WEST.0)..=(Self::SOUTH_EAST.0)).map(Self)
    }

    #[must_use]
    pub const fn const_add(&self, vector: &Vector) -> Option<Self> {
        let Some(c) = self.x().checked_add_signed(vector.x) else {
            return None;
        };
        let Some(r) = self.y().checked_add_signed(vector.y) else {
            return None;
        };

        Self::try_new(c, r)
    }

    #[must_use]
    pub const fn get_tile(&self, corner: &Corner) -> Option<Tile<WIDTH, HEIGHT>> {
        match corner {
            Corner::NorthWest => {
                let Some(x) = self.x().checked_sub(1) else {
                    return None;
                };
                let Some(y) = self.y().checked_sub(1) else {
                    return None;
                };
                Tile::try_new(x, y)
            }
            Corner::NorthEast => {
                let Some(y) = self.y().checked_sub(1) else {
                    return None;
                };
                Tile::try_new(self.x(), y)
            }
            Corner::SouthWest => {
                let Some(x) = self.x().checked_sub(1) else {
                    return None;
                };
                Tile::try_new(x, self.y())
            }
            Corner::SouthEast => Tile::try_new(self.x(), self.y()),
        }
    }
}

#[cfg(any(test, feature = "glam"))]
impl<const WIDTH: u8, const HEIGHT: u8> HasCenter for Vertex<WIDTH, HEIGHT> {
    #[must_use]
    fn get_center(&self, scale: f32) -> glam::f32::Vec2 {
        let x = scale * f32::from(self.x());
        let y = scale * f32::from(self.y());

        glam::f32::Vec2 { x, y }
    }
}

impl<const L: u8> Vertex<L, L> {
    pub const fn rotate(&self, quarter_turns: QuarterTurns) -> Self {
        match quarter_turns {
            QuarterTurns::Zero => *self,
            QuarterTurns::One => Self::new_unchecked(L - self.y(), self.x()),
            QuarterTurns::Two => Self::new_unchecked(L - self.x(), L - self.y()),
            QuarterTurns::Three => Self::new_unchecked(self.y(), L - self.x()),
        }
    }
}

impl<const W: u8, const H: u8> From<Vertex<W, H>> for u8 {
    fn from(val: Vertex<W, H>) -> Self {
        val.0
    }
}

impl<const W: u8, const H: u8> From<&Vertex<W, H>> for u8 {
    fn from(val: &Vertex<W, H>) -> Self {
        val.0
    }
}

impl<const W: u8, const H: u8> From<Vertex<W, H>> for usize {
    fn from(val: Vertex<W, H>) -> Self {
        val.0.into()
    }
}

impl<const W: u8, const H: u8> From<&Vertex<W, H>> for usize {
    fn from(val: &Vertex<W, H>) -> Self {
        val.0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    #[cfg(any(test, feature = "serde"))]
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_iter_by_row() {
        let str = Vertex::<2, 3>::iter_by_row().join("|");

        assert_eq!(
            str,
            "(0,0)|(1,0)|(2,0)|(0,1)|(1,1)|(2,1)|(0,2)|(1,2)|(2,2)|(0,3)|(1,3)|(2,3)",
        )
    }

    #[test]
    fn test_from() {
        for tile in Vertex::<3, 4>::iter_by_row() {
            let n = Vertex::try_new(tile.x(), tile.y()).unwrap();
            assert_eq!(tile, n)
        }
    }

    #[test]
    fn test_flip_vertical() {
        let str = Vertex::<2, 2>::iter_by_row()
            .map(|x| x.flip(FlipAxes::Vertical))
            .join("|");

        assert_eq!(str, "(0,2)|(1,2)|(2,2)|(0,1)|(1,1)|(2,1)|(0,0)|(1,0)|(2,0)")
    }

    #[cfg(any(test, feature = "serde"))]
    #[test]
    fn test_serde() {
        let tile: Vertex<3, 3> = Vertex(2);

        assert_tokens(
            &tile,
            &[Token::NewtypeStruct { name: "Vertex" }, Token::U8(2)],
        );
    }

    #[test]
    fn test_add() {
        let vertex: Vertex<3, 3> = Vertex::new_const::<1, 1>();
        assert_eq!(vertex + Vector::NORTH, Vertex::try_new(1, 0))
    }

    #[test]
    fn test_add_gives_none() {
        let vertex: Vertex<4, 4> = Vertex::new_const::<4, 0>();
        let r = vertex + Vector::new(1, 0);
        assert_eq!(r, None)
    }

    #[test]
    fn test_try_from() {
        assert_eq!(
            Vertex::<2, 2>::try_from_inner(8),
            Some(Vertex::new_const::<2, 2>())
        );
        assert_eq!(
            Vertex::<2, 2>::try_from_usize(8),
            Some(Vertex::new_const::<2, 2>())
        );
        assert_eq!(Vertex::<2, 2>::try_from_inner(9), None);
        assert_eq!(Vertex::<2, 2>::try_from_usize(9), None);
    }

    #[test]
    fn test_get_center() {
        let tile: Vertex<1, 2> = Vertex::new_const::<1, 2>();

        assert_eq!(tile.get_center(2.0), glam::f32::Vec2::new(2.0, 4.0));
    }

    #[test]
    fn test_try_next() {
        let mut tile = Vertex::<2, 2>(0);
        let mut i = 0;
        loop {
            assert_eq!(tile.inner(), i);
            i += 1;
            if let Some(next) = tile.try_next() {
                tile = next;
            } else {
                assert_eq!(i, 9);
                break;
            }
        }
    }

    #[test]
    fn test_flip2() {
        let vertex: Vertex<3, 3> = Vertex::new_const::<1, 2>();
        assert_eq!(vertex.flip(FlipAxes::None), Vertex::new_const::<1, 2>());
        assert_eq!(
            vertex.flip(FlipAxes::Horizontal),
            Vertex::new_const::<2, 2>()
        );
        assert_eq!(vertex.flip(FlipAxes::Vertical), Vertex::new_const::<1, 1>());
        assert_eq!(vertex.flip(FlipAxes::Both), Vertex::new_const::<2, 1>());
    }

    #[test]
    fn test_rotate2() {
        let vertex: Vertex<3, 3> = Vertex::new_const::<0, 0>();
        assert_eq!(
            vertex.rotate(QuarterTurns::Zero),
            Vertex::new_const::<0, 0>()
        );
        assert_eq!(
            vertex.rotate(QuarterTurns::One),
            Vertex::new_const::<3, 0>()
        );
        assert_eq!(
            vertex.rotate(QuarterTurns::Two),
            Vertex::new_const::<3, 3>()
        );
        assert_eq!(
            vertex.rotate(QuarterTurns::Three),
            Vertex::new_const::<0, 3>()
        );
    }

    #[test]
    fn get_tile_none() {
        let vertex: Vertex<3, 3> = Vertex::new_const::<0, 0>();

        assert_eq!(vertex.get_tile(&Corner::NorthWest), None);
        assert_eq!(vertex.get_tile(&Corner::NorthEast), None);

        assert_eq!(vertex.get_tile(&Corner::SouthWest), None);
    }

    #[test]
    fn test_int_from() {
        let vertex: Vertex<2, 2> = Vertex::new_const::<1, 1>();

        assert_eq!(<Vertex<2, 2> as Into<u8>>::into(vertex), 4u8);
        assert_eq!(<Vertex<2, 2> as Into<usize>>::into(vertex), 4usize);
        assert_eq!(<&Vertex<2, 2> as Into<u8>>::into(&vertex), 4u8);
        assert_eq!(<&Vertex<2, 2> as Into<usize>>::into(&vertex), 4usize);
    }

    #[test]
    fn test_get_tile() {
        use Corner::*;
        let vertex: Vertex<3, 3> = Vertex::new_const::<1, 1>();

        assert_eq!(vertex.get_tile(&NorthWest), Some(Tile::new_const::<0, 0>()));
        assert_eq!(vertex.get_tile(&NorthEast), Some(Tile::new_const::<1, 0>()));
        assert_eq!(vertex.get_tile(&SouthWest), Some(Tile::new_const::<0, 1>()));
        assert_eq!(vertex.get_tile(&SouthEast), Some(Tile::new_const::<1, 1>()));
    }

    #[test]
    fn test_from_dynamic() {
        let pairs = [
            ((0i8, 0i8), Some((0u8, 0u8))),
            ((1i8, 2i8), Some((1u8, 2u8))),
            ((3i8, 0i8), Some((3u8, 0u8))),
            ((0i8, 3i8), Some((0u8, 3u8))),
            ((4i8, 0i8), None),
            ((0i8, 4i8), None),
            ((-1i8, 0i8), None),
            ((0i8, -1i8), None),
        ];

        for ((dyn_x, dyn_y), tile_option) in pairs {
            let expected = tile_option.map(|(x, y)| Vertex::<3, 3>::try_new(x, y).unwrap());

            let dynamic_tile = DynamicVertex(Vector { x: dyn_x, y: dyn_y });

            let actual = Vertex::<3, 3>::try_from_dynamic(dynamic_tile);

            assert_eq!(actual, expected);
        }
    }
}
