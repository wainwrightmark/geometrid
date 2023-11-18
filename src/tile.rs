use core::{fmt::Display, ops::Add};

use crate::prelude::*;

#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};

/// A tile in 2d space
#[must_use]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(any(test, feature = "serde"), derive(Serialize, Deserialize))]
pub struct Tile<const WIDTH: u8, const HEIGHT: u8>(u8);

impl<const WIDTH: u8, const HEIGHT: u8, V: AsRef<Vector>> Add<V> for Tile<WIDTH, HEIGHT> {
    type Output = Option<Self>;

    fn add(self, rhs: V) -> Self::Output {
        self.const_add(rhs.as_ref())
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> From<Tile<WIDTH, HEIGHT>> for u8 {
    fn from(value: Tile<WIDTH, HEIGHT>) -> Self {
        value.0
    }
}
impl<const WIDTH: u8, const HEIGHT: u8> From<&Tile<WIDTH, HEIGHT>> for u8 {
    fn from(value: &Tile<WIDTH, HEIGHT>) -> Self {
        value.0
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> From<Tile<WIDTH, HEIGHT>> for usize {
    fn from(value: Tile<WIDTH, HEIGHT>) -> Self {
        value.0.into()
    }
}
impl<const WIDTH: u8, const HEIGHT: u8> From<&Tile<WIDTH, HEIGHT>> for usize {
    fn from(value: &Tile<WIDTH, HEIGHT>) -> Self {
        value.0.into()
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> Display for Tile<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({},{})", self.x(), self.y())
    }
}
impl<const WIDTH: u8, const HEIGHT: u8> core::fmt::Debug for Tile<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({},{})", self.x(), self.y())
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> Tile<WIDTH, HEIGHT> {
    pub const NORTH_WEST: Self = Self(0);
    pub const NORTH_EAST: Self = Self::new_unchecked(Self::MAX_COL, 0);
    pub const SOUTH_WEST: Self = Self::new_unchecked(0, Self::MAX_ROW);
    pub const SOUTH_EAST: Self = Self::new_unchecked(Self::MAX_COL, Self::MAX_ROW);

    pub const MAX_COL: u8 = WIDTH - 1;
    pub const MAX_ROW: u8 = HEIGHT - 1;

    pub const COUNT: usize = WIDTH as usize * HEIGHT as usize;

    pub const CENTER: Self = Self::new_unchecked(WIDTH / 2, HEIGHT / 2);

    #[must_use]
    pub const fn new_const<const X: u8, const Y: u8>() -> Self {
        Self::new_unchecked(X, Y)
    }

    #[must_use]
    pub(crate) const fn new_unchecked(x: u8, y: u8) -> Self {
        debug_assert!(x < WIDTH);
        debug_assert!(y < HEIGHT);
        debug_assert!(Self::COUNT <= u8::MAX as usize);
        Self(x + (WIDTH * y))
    }

    #[must_use]
    pub const fn try_new(x: u8, y: u8) -> Option<Self> {
        if x >= WIDTH {
            return None;
        }
        if y >= HEIGHT {
            return None;
        }
        let Some(i1) = y.checked_mul(WIDTH) else {
            return None;
        };
        let Some(i2) = i1.checked_add(x) else {
            return None;
        };
        Self::try_from_inner(i2)
    }

    #[must_use]
    pub const fn try_from_dynamic(dynamic_tile: DynamicTile) -> Option<Self> {
        if dynamic_tile.0.x.is_negative() || dynamic_tile.0.y.is_negative() {
            return None;
        }

        Self::try_new(
            dynamic_tile.0.x.unsigned_abs(),
            dynamic_tile.0.y.unsigned_abs(),
        )
    }

    #[must_use]
    pub const fn x(&self) -> u8 {
        self.0 % WIDTH
    }

    #[must_use]
    pub const fn y(&self) -> u8 {
        self.0 / WIDTH
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
    pub const fn try_from_usize(value: usize) -> Option<Self> {
        if value >= Self::COUNT {
            return None;
        }
        let inner = value as u8;
        Some(Self(inner))
    }

    #[must_use]
    pub const fn flip(&self, axes: FlipAxes) -> Self {
        use FlipAxes::*;
        match axes {
            None => *self,
            Horizontal => Self::new_unchecked(Self::MAX_COL - self.x(), self.y()),
            Vertical => Self::new_unchecked(self.x(), Self::MAX_ROW - self.y()),
            Both => Self::new_unchecked(Self::MAX_COL - self.x(), Self::MAX_ROW - self.y()),
        }
    }

    #[must_use]
    pub const fn try_next(&self) -> Option<Self> {
        let Some(next) = self.inner().checked_add(1) else {
            return None;
        };
        Self::try_from_inner(next)
    }

    /// Iterate through all tiles by row
    /// This method has better performance than `iter_by_col`
    #[must_use]
    pub fn iter_by_row() -> TileByRowIter<WIDTH, HEIGHT> {
        TileByRowIter::default()
    }

    /// Iterate through all tiles by column
    /// This method has worse performance than `iter_by_row`
    #[must_use]
    pub const fn iter_by_col() -> TileByColumnIter<WIDTH, HEIGHT> {
        TileByColumnIter(Some(Self(0)))
    }

    /// Iterate through adjacent elements (includes diagonals)
    #[must_use]
    pub fn iter_adjacent<'a>(&'a self) -> impl Iterator<Item = Self> + 'a {
        Vector::UNITS.into_iter().flat_map(|v| *self + v)
    }

    /// Iterate through contiguous elements (does not include diagonals)
    #[must_use]
    pub fn iter_contiguous<'a>(&'a self) -> impl Iterator<Item = Self> + 'a {
        Vector::CARDINALS.into_iter().flat_map(|v| *self + v)
    }

    /// Whether two tiles are adjacent (includes diagonals)
    pub const fn is_adjacent_to(&self, rhs: &Self) -> bool {
        self.0 != rhs.0 && self.x().abs_diff(rhs.x()) <= 1 && self.y().abs_diff(rhs.y()) <= 1
    }

    /// Whether two tiles are contiguous (does not include diagonals)
    pub const fn is_contiguous_with(&self, rhs: &Self) -> bool {
        if self.0 == rhs.0 {
            return false;
        }
        let c = self.x().abs_diff(rhs.x());
        let r = self.y().abs_diff(rhs.y());

        if c <= 1 && r <= 1 {
            if (c == 1) ^ (r == 1) {
                return true;
            }
        }
        return false;
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
    pub const fn get_vertex(&self, corner: &Corner) -> Option<Vertex<WIDTH, HEIGHT>> {
        use Corner::*;

        match corner {
            NorthWest => Vertex::try_new(self.x(), self.y()),
            NorthEast => Vertex::try_new(self.x() + 1, self.y()),
            SouthWest => Vertex::try_new(self.x(), self.y() + 1),
            SouthEast => Vertex::try_new(self.x() + 1, self.y() + 1),
        }
    }

    #[must_use]
    pub const fn get_north_west_vertex(&self) -> Vertex<WIDTH, HEIGHT> {
        Vertex::new_unchecked(self.x(), self.y())
    }

    /// Returns the Manhattan distance between two tiles.
    /// Also known as the taxicab distance, the Manhattan distance is the sum of the distances in the two axes.
    #[must_use]
    pub const fn manhattan_distance(&self, other: &Self) -> u8 {
        self.x().abs_diff(other.x()) + self.y().abs_diff(other.y())
    }

    /// Returns true if this is an edge tile (or corner tile)
    #[must_use]
    pub const fn is_edge(&self) -> bool {
        (self.x() == 0 || self.x() == Self::MAX_COL) || (self.y() == 0 || self.y() == Self::MAX_ROW)
    }

    /// Returns true if this is a corner tile
    #[must_use]
    pub const fn is_corner(&self) -> bool {
        Self::NORTH_EAST.0 == self.0
            || Self::NORTH_WEST.0 == self.0
            || Self::SOUTH_EAST.0 == self.0
            || Self::SOUTH_WEST.0 == self.0
    }

    /// Returns the number of adjacent tiles
    /// 3 for a corner tile
    /// 5 for an edge tile
    /// 8 otherwise
    pub const fn adjacent_tile_count(&self) -> u8 {
        if self.is_corner() {
            return 3;
        } else if self.is_edge() {
            return 5;
        } else {
            return 8;
        }
    }
}

impl<const L: u8> Tile<L, L> {
    pub const fn rotate(&self, quarter_turns: QuarterTurns) -> Self {
        match quarter_turns {
            QuarterTurns::Zero => *self,
            QuarterTurns::One => Self::new_unchecked(L - 1 - self.y(), self.x()),
            QuarterTurns::Two => Self::new_unchecked(L - 1 - self.x(), L - 1 - self.y()),
            QuarterTurns::Three => Self::new_unchecked(self.y(), L - 1 - self.x()),
        }
    }
}

#[cfg(any(test, feature = "glam"))]
impl<const C: u8, const R: u8> HasCenter for Tile<C, R> {
    fn get_center(&self, scale: f32) -> glam::f32::Vec2 {
        let x = scale * ((self.x() as f32) + 0.5);
        let y = scale * ((self.y() as f32) + 0.5);

        glam::f32::Vec2 { x, y }
    }
}

#[must_use]
#[derive(Clone)]
pub struct TileByColumnIter<const WIDTH: u8, const HEIGHT: u8>(Option<Tile<WIDTH, HEIGHT>>);

impl<const C: u8, const R: u8> Iterator for TileByColumnIter<C, R> {
    type Item = Tile<C, R>;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.0?;

        self.0 = (r + Vector::SOUTH).or_else(|| Tile::try_new(r.x() + 1, 0));

        return Some(r);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::*;
    use itertools::Itertools;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_iter_by_row() {
        let str = Tile::<3, 4>::iter_by_row().join("|");

        assert_eq!(
            str,
            "(0,0)|(1,0)|(2,0)|(0,1)|(1,1)|(2,1)|(0,2)|(1,2)|(2,2)|(0,3)|(1,3)|(2,3)",
        )
    }

    #[test]
    fn test_iter_by_col() {
        let str = Tile::<3, 4>::iter_by_col().join("|");

        assert_eq!(
            str,
            "(0,0)|(0,1)|(0,2)|(0,3)|(1,0)|(1,1)|(1,2)|(1,3)|(2,0)|(2,1)|(2,2)|(2,3)",
        )
    }

    #[test]
    fn test_from() {
        for tile in Tile::<3, 4>::iter_by_row() {
            let n = Tile::try_new(tile.x(), tile.y()).unwrap();
            assert_eq!(tile, n)
        }
    }

    #[test]
    fn test_flip1() {
        let str = Tile::<3, 3>::iter_by_row()
            .map(|x| x.flip(FlipAxes::Vertical))
            .join("|");

        assert_eq!(str, "(0,2)|(1,2)|(2,2)|(0,1)|(1,1)|(2,1)|(0,0)|(1,0)|(2,0)")
    }

    #[test]
    fn test_rotate1() {
        let str = Tile::<3, 3>::iter_by_row()
            .map(|x| x.rotate(QuarterTurns::One))
            .join("|");

        assert_eq!(str, "(2,0)|(2,1)|(2,2)|(1,0)|(1,1)|(1,2)|(0,0)|(0,1)|(0,2)")
    }

    #[test]
    fn test_flip2() {
        let tile: Tile<4, 4> = Tile::new_const::<1, 2>();
        assert_eq!(tile.flip(FlipAxes::None), Tile::new_const::<1, 2>());
        assert_eq!(tile.flip(FlipAxes::Horizontal), Tile::new_const::<2, 2>());
        assert_eq!(tile.flip(FlipAxes::Vertical), Tile::new_const::<1, 1>());
        assert_eq!(tile.flip(FlipAxes::Both), Tile::new_const::<2, 1>());
    }

    #[test]
    fn test_rotate2() {
        let tile: Tile<4, 4> = Tile::new_const::<0, 0>();
        assert_eq!(tile.rotate(QuarterTurns::Zero), Tile::new_const::<0, 0>());
        assert_eq!(tile.rotate(QuarterTurns::One), Tile::new_const::<3, 0>());
        assert_eq!(tile.rotate(QuarterTurns::Two), Tile::new_const::<3, 3>());
        assert_eq!(tile.rotate(QuarterTurns::Three), Tile::new_const::<0, 3>());
    }

    #[test]
    fn test_serde() {
        let tile: Tile<3, 3> = Tile(2);

        assert_tokens(
            &tile,
            &[Token::NewtypeStruct { name: "Tile" }, Token::U8(2)],
        );
    }

    #[test]
    fn test_manhattan() {
        let a: Tile<3, 3> = Tile::new_const::<0, 0>();
        let b: Tile<3, 3> = Tile::new_const::<2, 1>();

        assert_eq!(a.manhattan_distance(&b), 3);
        assert_eq!(b.manhattan_distance(&a), 3);
    }

    #[test]
    fn test_add() {
        let tile: Tile<3, 3> = Tile::new_const::<1, 1>();
        assert_eq!(tile + Vector::NORTH, Tile::try_new(1, 0))
    }

    #[test]
    fn test_add_gives_none() {
        let tile: Tile<4, 4> = Tile::new_const::<3, 0>();
        let r = tile + Vector::new(1, 0);
        assert_eq!(r, None)
    }

    #[test]
    fn test_int_from() {
        let tile: Tile<3, 3> = Tile::new_const::<1, 1>();

        assert_eq!(<Tile<3, 3> as Into<u8>>::into(tile), 4u8);
        assert_eq!(<Tile<3, 3> as Into<usize>>::into(tile), 4usize);
        assert_eq!(<&Tile<3, 3> as Into<u8>>::into(&tile), 4u8);
        assert_eq!(<&Tile<3, 3> as Into<usize>>::into(&tile), 4usize);
    }

    #[test]
    fn test_get_center() {
        let tile: Tile<3, 3> = Tile::new_const::<1, 2>();

        assert_eq!(tile.get_center(2.0), glam::f32::Vec2::new(3.0, 5.0));
    }

    #[test]
    fn test_debug() {
        let tile: Tile<3, 3> = Tile::new_const::<1, 2>();

        assert_eq!(format!("{tile:?}"), "(1,2)")
    }

    #[test]
    fn test_try_from() {
        assert_eq!(
            Tile::<3, 3>::try_from_inner(8),
            Some(Tile::new_const::<2, 2>())
        );
        assert_eq!(
            Tile::<3, 3>::try_from_usize(8),
            Some(Tile::new_const::<2, 2>())
        );
        assert_eq!(Tile::<3, 3>::try_from_inner(9), None);
        assert_eq!(Tile::<3, 3>::try_from_usize(9), None);
    }

    #[test]
    fn test_try_next() {
        let mut tile = Tile::<3, 3>(0);
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
    fn test_get_vertex() {
        let tile = Tile::<2, 2>::new_const::<0, 0>();

        assert_eq!(
            tile.get_vertex(&Corner::NorthWest),
            Some(Vertex::new_const::<0, 0>())
        );
        assert_eq!(
            tile.get_vertex(&Corner::NorthEast),
            Some(Vertex::new_const::<1, 0>())
        );
        assert_eq!(
            tile.get_vertex(&Corner::SouthWest),
            Some(Vertex::new_const::<0, 1>())
        );
        assert_eq!(
            tile.get_vertex(&Corner::SouthEast),
            Some(Vertex::new_const::<1, 1>())
        );

        assert_eq!(tile.get_north_west_vertex(), Vertex::new_const::<0, 0>())
    }

    #[test]
    fn test_adjacent() {
        let tile = Tile::<3, 3>::new_const::<0, 0>();

        let expected_adjacent_tiles = [
            Tile::<3, 3>::new_const::<1, 0>(),
            Tile::<3, 3>::new_const::<1, 1>(),
            Tile::<3, 3>::new_const::<0, 1>(),
        ];

        assert_eq!(tile.iter_adjacent().collect_vec(), expected_adjacent_tiles);

        for rhs in Tile::<3, 3>::iter_by_row() {
            let expected = expected_adjacent_tiles.contains(&rhs);
            let actual = tile.is_adjacent_to(&rhs);
            assert_eq!(expected, actual)
        }
    }

    #[test]
    fn test_contiguous() {
        let tile = Tile::<3, 3>::new_const::<0, 0>();

        let expected_contiguous_tiles = [
            Tile::<3, 3>::new_const::<1, 0>(),
            Tile::<3, 3>::new_const::<0, 1>(),
        ];

        assert_eq!(
            tile.iter_contiguous().collect_vec(),
            expected_contiguous_tiles
        );

        for rhs in Tile::<3, 3>::iter_by_row() {
            let expected = expected_contiguous_tiles.contains(&rhs);
            let actual = tile.is_contiguous_with(&rhs);
            assert_eq!(expected, actual, "{}", rhs)
        }
    }

    #[test]
    fn test_from_dynamic() {
        let pairs = [
            ((0i8, 0i8), Some((0u8, 0u8))),
            ((1i8, 2i8), Some((1u8, 2u8))),
            ((3i8, 0i8), None),
            ((0i8, 3i8), None),
            ((-1i8, 0i8), None),
            ((0i8, -1i8), None),
        ];

        for ((dyn_x, dyn_y), tile_option) in pairs {
            let expected = tile_option.map(|(x, y)| Tile::<3, 3>::try_new(x, y).unwrap());

            let dynamic_tile = DynamicTile(Vector { x: dyn_x, y: dyn_y });

            let actual = Tile::<3, 3>::try_from_dynamic(dynamic_tile);

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_is_corner() {
        let corners: TileSet16<3, 4, 12> = TileSet16::from_fn(|tile| tile.is_corner());

        assert_eq!("*_*\n___\n___\n*_*", corners.to_string())
    }

    #[test]
    fn test_is_edge() {
        let edges: TileSet16<3, 4, 12> = TileSet16::from_fn(|tile| tile.is_edge());

        assert_eq!("***\n*_*\n*_*\n***", edges.to_string())
    }

    #[test]
    fn test_adjacent_tile_count() {
        let adjacencies: TileMap<u8, 3, 4, 12> =
            TileMap::from_fn(|tile| tile.adjacent_tile_count());

        assert_eq!("3|5|3\n5|8|5\n5|8|5\n3|5|3", adjacencies.to_string())
    }
}
