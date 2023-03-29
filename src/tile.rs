use core::{fmt::Display, ops::Add};

use crate::{
    center::{Center, HasCenter},
    corner::Corner,
    flip_axes::FlipAxes,
    quarter_turns::QuarterTurns,
    vector::Vector,
    vertex::Vertex,
};

#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};

#[must_use]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(any(test, feature = "serde"), derive(Serialize, Deserialize))]
pub struct Tile<const COLS: u8, const ROWS: u8>(u8);

impl<const COLS: u8, const ROWS: u8, V: AsRef<Vector>> Add<V> for Tile<COLS, ROWS> {
    type Output = Option<Self>;

    fn add(self, rhs: V) -> Self::Output {
        self.const_add(rhs.as_ref())
    }
}

impl<const COLS: u8, const ROWS: u8> From<Tile<COLS, ROWS>> for u8 {
    fn from(value: Tile<COLS, ROWS>) -> Self {
        value.0
    }
}
impl<const COLS: u8, const ROWS: u8> From<&Tile<COLS, ROWS>> for u8 {
    fn from(value: &Tile<COLS, ROWS>) -> Self {
        value.0
    }
}

impl<const COLS: u8, const ROWS: u8> From<Tile<COLS, ROWS>> for usize {
    fn from(value: Tile<COLS, ROWS>) -> Self {
        value.0.into()
    }
}
impl<const COLS: u8, const ROWS: u8> From<&Tile<COLS, ROWS>> for usize {
    fn from(value: &Tile<COLS, ROWS>) -> Self {
        value.0.into()
    }
}

impl<const COLS: u8, const ROWS: u8> Display for Tile<COLS, ROWS> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({},{})", self.col(), self.row())
    }
}
impl<const COLS: u8, const ROWS: u8> core::fmt::Debug for Tile<COLS, ROWS> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({},{})", self.col(), self.row())
    }
}

impl<const COLS: u8, const ROWS: u8> Tile<COLS, ROWS> {
    pub const NORTH_WEST: Self = Self(0);
    pub const NORTH_EAST: Self = Self::new_unchecked(Self::MAX_COL, 0);
    pub const SOUTH_WEST: Self = Self::new_unchecked(0, Self::MAX_ROW);
    pub const SOUTH_EAST: Self = Self::new_unchecked(Self::MAX_COL, Self::MAX_ROW);

    pub const MAX_COL: u8 = COLS - 1;
    pub const MAX_ROW: u8 = ROWS - 1;

    pub const COUNT: usize = COLS as usize * ROWS as usize;

    pub const CENTER: Self = Self::new_unchecked(COLS / 2, ROWS / 2);

    #[must_use]
    pub const fn new_const<const X: u8, const Y: u8>() -> Self {
        Self::new_unchecked(X, Y)
    }

    #[must_use]
    pub(crate) const fn new_unchecked(col: u8, row: u8) -> Self {
        debug_assert!(col < COLS);
        debug_assert!(row < ROWS);
        debug_assert!(Self::COUNT <= u8::MAX as usize);
        Self(col + (COLS * row))
    }

    #[must_use]
    pub const fn try_new(col: u8, row: u8) -> Option<Self> {
        if col >= COLS {
            return None;
        }
        if row >= ROWS {
            return None;
        }
        let Some(i1) = row.checked_mul(COLS) else{return None};
        let Some(i2) = i1.checked_add(col) else {return  None};
        Self::try_from_inner(i2)
    }

    #[must_use]
    pub const fn col(&self) -> u8 {
        self.0 % COLS
    }

    #[must_use]
    pub const fn row(&self) -> u8 {
        self.0 / COLS
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
            Horizontal => Self::new_unchecked(Self::MAX_COL - self.col(), self.row()),
            Vertical => Self::new_unchecked(self.col(), Self::MAX_ROW - self.row()),
            Both => Self::new_unchecked(Self::MAX_COL - self.col(), Self::MAX_ROW - self.row()),
        }
    }

    #[must_use]
    pub const fn try_next(&self) -> Option<Self> {
        let Some(next) = self.inner().checked_add(1) else{ return  None;};
        Self::try_from_inner(next)
    }

    /// Iterate through all tiles by row
    #[must_use]
    pub fn iter_by_row() -> impl Iterator<Item = Self> {
        ((Self::NORTH_WEST.0)..=(Self::SOUTH_EAST.0)).map(|x| Self(x))
    }

    /// Iterate through all tiles by column
    #[must_use]
    pub const fn iter_by_col() -> impl Iterator<Item = Self> {
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
    pub fn is_adjacent_to(&self, rhs: &Self) -> bool {
        self != rhs && self.col().abs_diff(rhs.col()) <= 1 && self.row().abs_diff(rhs.row()) <= 1
    }

    /// Whether two tiles are contiguous (does not include diagonals)
    pub fn is_contiguous_with(&self, rhs: &Self) -> bool {
        if self == rhs {
            return false;
        }
        let c = self.col().abs_diff(rhs.col());
        let r = self.row().abs_diff(rhs.row());

        if c <= 1 && r <= 1 {
            if (c == 1) ^ (r == 1) {
                return true;
            }
        }
        return false;
    }

    #[must_use]
    pub const fn const_add(&self, vector: &Vector) -> Option<Self> {
        let Some(c) = self.col().checked_add_signed(vector.x) else {return None;};
        let Some(r) = self.row().checked_add_signed(vector.y) else {return None;};

        Self::try_new(c, r)
    }

    #[must_use]
    pub const fn get_vertex(&self, corner: &Corner) -> Option<Vertex<COLS, ROWS>> {
        use Corner::*;

        match corner {
            NorthWest => Vertex::try_new(self.col(), self.row()),
            NorthEast => Vertex::try_new(self.col() + 1, self.row()),
            SouthWest => Vertex::try_new(self.col(), self.row() + 1),
            SouthEast => Vertex::try_new(self.col() + 1, self.row() + 1),
        }
    }

    #[must_use]
    pub const fn get_north_west_vertex(&self) -> Vertex<COLS, ROWS> {
        Vertex::new_unchecked(self.col(), self.row())
    }

    #[must_use]
    pub const fn manhattan_distance(&self, other: &Self) -> u8 {
        self.col().abs_diff(other.col()) + self.row().abs_diff(other.row())
    }
}

impl<const L: u8> Tile<L, L> {
    pub const fn rotate(&self, quarter_turns: QuarterTurns) -> Self {
        match quarter_turns {
            QuarterTurns::Zero => *self,
            QuarterTurns::One => Self::new_unchecked(L - 1 - self.row(), self.col()),
            QuarterTurns::Two => Self::new_unchecked(L - 1 - self.col(), L - 1 - self.row()),
            QuarterTurns::Three => Self::new_unchecked(self.row(), L - 1 - self.col()),
        }
    }
}

impl<const C: u8, const R: u8> HasCenter for Tile<C, R> {
    fn get_center(&self, scale: f32) -> crate::center::Center {
        let x = scale * ((self.col() as f32) + 0.5);
        let y = scale * ((self.row() as f32) + 0.5);

        Center { x, y }
    }
}

#[must_use]
#[derive(Clone)]
pub struct TileByColumnIter<const COLS: u8, const ROWS: u8>(Option<Tile<COLS, ROWS>>);

impl<const C: u8, const R: u8> Iterator for TileByColumnIter<C, R> {
    type Item = Tile<C, R>;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.0?;

        self.0 = (r + Vector::SOUTH).or_else(|| Tile::try_new(r.col() + 1, 0));

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
    fn test_iter_by_col(){
        let str = Tile::<3, 4>::iter_by_col().join("|");

        assert_eq!(
            str,
            "(0,0)|(0,1)|(0,2)|(0,3)|(1,0)|(1,1)|(1,2)|(1,3)|(2,0)|(2,1)|(2,2)|(2,3)",
        )
    }

    #[test]
    fn test_from() {
        for tile in Tile::<3, 4>::iter_by_row() {
            let n = Tile::try_new(tile.col(), tile.row()).unwrap();
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

        assert_eq!(tile.get_center(2.0), Center::new(3.0, 5.0));
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
}
