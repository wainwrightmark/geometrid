use core::{
    fmt::{self, Write},
    iter,
    ops::{Index, IndexMut, Shl, Shr},
};

use crate::prelude::*;
use ethnum::U256;
#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};

//todo  all new methods from tile_set
/// A grid
/// A map from tiles to bools. Can contain
#[must_use]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(any(test, feature = "serde"), derive(Serialize, Deserialize))]
pub struct TileSet256<const WIDTH: u8, const HEIGHT: u8, const SIZE: usize>(U256);

impl<const WIDTH: u8, const HEIGHT: u8, const SIZE: usize> Default
    for TileSet256<WIDTH, HEIGHT, SIZE>
{
    fn default() -> Self {
        Self::assert_legal();
        Self::EMPTY
    }
}

impl<const WIDTH: u8, const HEIGHT: u8, const SIZE: usize> TileSet256<WIDTH, HEIGHT, SIZE> {
    pub const EMPTY: Self = {
        Self::assert_legal();
        Self(U256::new(0))
    };

    /// The set where all tiles are present
    pub fn all() -> Self {
        Self::EMPTY.negate()
    }

    #[inline]
    const fn assert_legal() {
        debug_assert!(SIZE == (WIDTH as usize * HEIGHT as usize));
        debug_assert!(SIZE <= <U256>::BITS as usize);
    }

    #[must_use]
    pub fn from_fn<F: FnMut(Tile<WIDTH, HEIGHT>) -> bool>(mut cb: F) -> Self {
        Self::assert_legal();

        let mut result = Self::default();
        for tile in Tile::<WIDTH, HEIGHT>::iter_by_row() {
            if cb(tile) {
                result.set_bit(&tile, true);
            }
        }

        result
    }

    #[must_use]
    #[inline]
    pub const fn row_mask(y: u8) -> Self {
        Self::assert_legal();
        let mut upper: u128 = 0;
        let mut lower: u128 = 0;
        let mut tile = Tile::<WIDTH, HEIGHT>::try_new(0, y);

        while let Some(t) = tile {
            let i = t.inner();
            match t.inner().checked_sub(128) {
                Some(i) => upper |= 1u128 << i,
                None => lower |= 1u128 << i,
            }
            tile = t.const_add(&Vector::EAST);
        }

        let a = U256::from_words(upper, lower);
        Self(a)
    }

    #[must_use]
    #[inline]
    pub const fn col_mask(x: u8) -> Self {
        Self::assert_legal();
        let mut upper: u128 = 0;
        let mut lower: u128 = 0;
        let mut tile = Tile::<WIDTH, HEIGHT>::try_new(x, 0);

        while let Some(t) = tile {
            let i = t.inner();
            match t.inner().checked_sub(128) {
                Some(i) => upper |= 1u128 << i,
                None => lower |= 1u128 << i,
            }
            tile = t.const_add(&Vector::SOUTH);
        }

        let a = U256::from_words(upper, lower);
        Self(a)
    }

    #[must_use]
    #[inline]
    pub const fn from_inner(inner: U256) -> Self {
        Self::assert_legal();
        Self(inner)
    }

    pub fn from_iter(iter: impl Iterator<Item = Tile<WIDTH, HEIGHT>>) -> Self {
        Self::assert_legal();
        let mut r = Self::default();
        for x in iter {
            r.set_bit(&x, true);
        }
        r
    }

    #[must_use]
    #[inline]
    pub const fn into_inner(self) -> U256 {
        self.0
    }

    #[must_use]
    #[inline]
    pub const fn is_empty(self) -> bool {
        let (h, l) = self.0.into_words();
        h == 0 && l == 0
    }

    #[inline]
    pub fn set_bit(&mut self, tile: &Tile<WIDTH, HEIGHT>, bit: bool) {
        if bit {
            self.0 |= U256::ONE.shl(tile.inner());
        } else {
            self.0 &= !(U256::ONE.shl(tile.inner()));
        }
    }

    #[must_use]
    #[inline]
    pub fn get_bit(&self, tile: &Tile<WIDTH, HEIGHT>) -> bool {
        (self.0.shr(tile.inner())) & U256::ONE == U256::ONE
    }

    /// Returns a copy of self with the bit at `tile` set to `bit`
    #[must_use]
    #[inline]
    pub fn with_bit_set(&self, tile: &Tile<WIDTH, HEIGHT>, bit: bool) -> Self {
        let inner = if bit {
            self.0 | U256::ONE.shl(tile.inner())
        } else {
            self.0 & !(U256::ONE.shl(tile.inner()))
        };

        Self(inner)
    }

    #[must_use]
    pub const fn iter(&self) -> impl DoubleEndedIterator<Item = bool> + ExactSizeIterator {
        TileSetIter256::<1> {
            bottom_index: 0,
            top_index: SIZE,
            inner: self.0,
        }
    }

    #[must_use]
    pub const fn row(&self, y: u8) -> impl DoubleEndedIterator<Item = bool> + ExactSizeIterator {
        TileSetIter256::<1> {
            bottom_index: (y * WIDTH) as usize,
            top_index: ((y + 1) * WIDTH) as usize,
            inner: self.0,
        }
    }

    #[must_use]
    pub const fn col(&self, x: u8) -> impl DoubleEndedIterator<Item = bool> + ExactSizeIterator {
        TileSetIter256::<HEIGHT> {
            bottom_index: x as usize,
            top_index: ((WIDTH * (HEIGHT - 1)) + x + 1) as usize,
            inner: self.0,
        }
    }

    #[must_use]
    pub fn enumerate(
        &self,
    ) -> impl DoubleEndedIterator<Item = (Tile<WIDTH, HEIGHT>, bool)> + ExactSizeIterator {
        self.iter()
            .enumerate()
            .map(|(i, x)| (Tile::try_from_usize(i).unwrap(), x))
    }

    #[must_use]
    pub fn iter_true_tiles(&self) -> impl Iterator<Item = Tile<WIDTH, HEIGHT>> {
        self.iter()
            .enumerate()
            .flat_map(|(i, b)| b.then(|| Tile::try_from_usize(i).unwrap()))
    }

    #[must_use]
    pub const fn count(&self) -> usize {
        self.0.count_ones() as usize
    }

    /// Get the scale to make the grid take up as much as possible of a given area
    #[must_use]
    pub fn get_scale(total_width: f32, total_height: f32) -> f32 {
        let x_multiplier = total_width / WIDTH as f32;
        let y_multiplier = total_height / HEIGHT as f32;

        x_multiplier.min(y_multiplier)
    }

    /// Return the set of tiles in both self and `rhs`.
    #[must_use]
    pub const fn intersect(&self, rhs: &Self) -> Self {
        let (left_high, left_low) = self.0.into_words();
        let (right_high, right_low) = rhs.0.into_words();

        let high = left_high & right_high;
        let low = left_low & right_low;
        Self(U256::from_words(high, low))
    }

    /// Return the set of tiles in either self or `rhs` or both.
    #[must_use]
    pub const fn union(&self, rhs: &Self) -> Self {
        let (left_high, left_low) = self.0.into_words();
        let (right_high, right_low) = rhs.0.into_words();

        let high = left_high | right_high;
        let low = left_low | right_low;
        Self(U256::from_words(high, low))
    }

    #[must_use]
    pub const fn is_subset(&self, rhs: &Self) -> bool {
        let (self_high, self_low) = self.0.into_words();
        let intersect = self.intersect(rhs);
        let (intersect_high, intersect_low) = intersect.0.into_words();

        self_high == intersect_high && self_low == intersect_low
    }

    #[must_use]
    pub const fn is_superset(&self, rhs: &Self) -> bool {
        let (rhs_high, rhs_low) = rhs.0.into_words();
        let intersect = self.intersect(rhs);
        let (intersect_high, intersect_low) = intersect.0.into_words();

        rhs_high == intersect_high && rhs_low == intersect_low
    }

    /// Returns a new set containing all elements which belong to one set but not both
    #[must_use]
    pub fn symmetric_difference(&self, rhs: &Self) -> Self {
        Self(self.0 ^ rhs.0)
    }

    #[must_use]
    pub fn negate(&self) -> Self {
        let mask: U256 = <U256>::MAX >> (<U256>::BITS - SIZE as u32);
        Self(!self.0 & mask)
    }

    #[must_use]
    pub fn shift_north(&self, rows: u8) -> Self {
        let a = self.0.shr(rows * WIDTH);
        let mask: U256 = <U256>::MAX >> (<U256>::BITS - SIZE as u32);
        Self(a & mask)
    }

    #[must_use]
    pub fn shift_south(&self, rows: u8) -> Self {
        let a = self.0.shl(rows * WIDTH);
        let mask: U256 = <U256>::MAX >> (<U256>::BITS - SIZE as u32);
        Self(a & mask)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TileSetIter256<const STEP: u8> {
    inner: U256,
    bottom_index: usize,
    top_index: usize,
}

impl<const STEP: u8> ExactSizeIterator for TileSetIter256<STEP> {
    fn len(&self) -> usize {
        self.count()
    }
}

impl<const STEP: u8> Iterator for TileSetIter256<STEP> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bottom_index >= self.top_index {
            None
        } else {
            let r = (self.inner >> self.bottom_index) & 1 == 1;
            self.bottom_index = self.bottom_index + (STEP as usize);
            Some(r)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count(), Some(self.count()))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        let distance = (self.top_index.saturating_sub(self.bottom_index)) as usize;
        let count = (distance / STEP as usize);
        count
    }
}

impl<const STEP: u8> DoubleEndedIterator for TileSetIter256<STEP> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.top_index == 0 {
            return None;
        }
        let next_index = self.top_index.saturating_sub(STEP as usize);
        if self.bottom_index > next_index {
            None
        } else {
            self.top_index = next_index;
            let r = (self.inner >> self.top_index) & 1 == 1;

            Some(r)
        }
    }
}

impl<const W: u8, const H: u8, const SIZE: usize> fmt::Display for TileSet256<W, H, SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.iter().enumerate();

        for (i, e) in iter {
            if i > 0 && i % (W as usize) == 0 {
                f.write_char('\n')?;
            }
            if e {
                f.write_char('*');
            } else {
                f.write_char('_');
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use itertools::Itertools;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_possible() {
        let mut grid: TileSet256<16, 16, 256> = TileSet256::default();
    }

    #[test]
    fn basic_tests() {
        let mut grid: TileSet256<3, 3, 9> = TileSet256::from_fn(|x| x.inner() % 2 == 0);

        assert_eq!(grid.to_string(), "*_*\n_*_\n*_*");

        assert_eq!(grid.count(), 5);

        for tile in Tile::<3, 3>::iter_by_row() {
            assert_eq!(grid.get_bit(&tile), tile.inner() % 2 == 0)
        }

        grid.set_bit(&Tile::CENTER, false);

        assert_eq!(grid.to_string(), "*_*\n___\n*_*");
        assert_eq!(grid.count(), 4);

        assert_eq!(
            grid.iter_true_tiles()
                .map(|x| x.inner())
                .collect_vec()
                .as_slice(),
            &[0, 2, 6, 8]
        );

        assert_eq!(grid.iter().count(), 9);
        assert_eq!(grid.into_inner(), 325);

        assert_eq!(grid.negate().to_string(), "_*_\n***\n_*_");
    }

    #[test]
    fn test_intersect() {
        let grid_left: TileSet256<3, 3, 9> = TileSet256::from_fn(|x| x.x() == 1);
        let grid_right: TileSet256<3, 3, 9> = TileSet256::from_fn(|x| x.y() == 1);

        assert_eq!(
            grid_left.intersect(&grid_right).to_string(),
            "___\n_*_\n___"
        )
    }

    #[test]
    fn test_union() {
        let grid_left: TileSet256<3, 3, 9> = TileSet256::from_fn(|x| x.x() == 0);
        let grid_top: TileSet256<3, 3, 9> = TileSet256::from_fn(|x| x.y() == 0);

        assert_eq!(grid_left.union(&grid_top).to_string(), "***\n*__\n*__")
    }

    #[test]
    fn test_symmetric_difference() {
        let grid_left: TileSet256<3, 3, 9> = TileSet256::from_fn(|x| x.x() == 0);
        let grid_top: TileSet256<3, 3, 9> = TileSet256::from_fn(|x| x.y() == 0);

        assert_eq!(
            grid_left.symmetric_difference(&grid_top).to_string(),
            "_**\n\
         *__\n\
         *__"
        )
    }

    #[test]
    fn test_subset() {
        let grid_top: TileSet256<3, 3, 9> = TileSet256::from_fn(|x| x.y() == 0);
        let all: TileSet256<3, 3, 9> = TileSet256::all();

        assert!(grid_top.is_subset(&all));
        assert!(grid_top.is_subset(&grid_top));
        assert!(!all.is_subset(&grid_top));
    }

    #[test]
    fn test_superset() {
        let grid_top: TileSet256<3, 3, 9> = TileSet256::from_fn(|x| x.y() == 0);
        let all: TileSet256<3, 3, 9> = TileSet256::all();

        assert!(!grid_top.is_superset(&all));
        assert!(grid_top.is_superset(&grid_top));
        assert!(all.is_superset(&grid_top));
    }

    #[test]
    fn test_from_inner() {
        assert_eq!(
            TileSet256::<3, 3, 9>::from_inner(U256::from(3u128)).to_string(),
            "**_\n___\n___"
        )
    }

    #[test]
    fn test_from_iter() {
        let grid = TileSet256::<4, 3, 12>::from_iter(
            [
                Tile::try_from_inner(0).unwrap(),
                Tile::try_from_inner(1).unwrap(),
            ]
            .into_iter(),
        );
        assert_eq!(grid.to_string(), "**__\n____\n____")
    }

    #[test]
    fn test_iter_reverse() {
        let grid = TileSet256::<4, 3, 12>::from_fn(|x| x.inner() >= 6);

        assert_eq!(
            grid.iter()
                .rev()
                .map(|x| x.then(|| "*").unwrap_or("_"))
                .join(""),
            "******______"
        );
    }

    #[test]
    fn test_row() {
        let grid = TileSet256::<4, 3, 12>::from_fn(|x| x.inner() % 3 == 1);

        assert_eq!(
            grid.row(0).map(|x| x.then(|| "*").unwrap_or("_")).join(""),
            "_*__"
        );
        assert_eq!(
            grid.row(1).map(|x| x.then(|| "*").unwrap_or("_")).join(""),
            "*__*"
        );
        assert_eq!(
            grid.row(2).map(|x| x.then(|| "*").unwrap_or("_")).join(""),
            "__*_"
        );
    }

    #[test]
    fn test_col() {
        let grid = TileSet256::<4, 3, 12>::from_fn(|x| x.inner() % 2 == 1);

        assert_eq!(
            grid.col(0).map(|x| x.then(|| "*").unwrap_or("_")).join(""),
            "_*_"
        );
        assert_eq!(
            grid.col(1).map(|x| x.then(|| "*").unwrap_or("_")).join(""),
            "*_*"
        );
        assert_eq!(
            grid.col(2).map(|x| x.then(|| "*").unwrap_or("_")).join(""),
            "_*_"
        );

        assert_eq!(
            grid.col(3).map(|x| x.then(|| "*").unwrap_or("_")).join(""),
            "*_*"
        );
    }

    #[test]
    fn test_enumerate() {
        let grid = TileSet256::<3, 3, 9>::from_fn(|x| x.inner() == 5);

        assert_eq!(
            grid.enumerate()
                .map(|(t, x)| t.inner().to_string() + x.then(|| "*").unwrap_or("_"))
                .join(""),
            "0_1_2_3_4_5*6_7_8_"
        );
    }

    #[test]
    fn test_shift() {
        let full_grid = TileSet256::<2, 3, 6>::default().negate();

        assert_eq!(full_grid.shift_north(0), full_grid);
        assert_eq!(full_grid.shift_south(0), full_grid);

        assert_eq!(full_grid.shift_north(1).to_string(), "**\n**\n__");
        assert_eq!(full_grid.shift_south(1).to_string(), "__\n**\n**");

        assert_eq!(full_grid.shift_north(2).to_string(), "**\n__\n__");
        assert_eq!(full_grid.shift_south(2).to_string(), "__\n__\n**");
    }

    #[test]
    fn test_row_mask() {
        type Grid = TileSet256<4, 3, 12>;
        assert_eq!(Grid::row_mask(0).to_string(), "****\n____\n____");
        assert_eq!(Grid::row_mask(1).to_string(), "____\n****\n____");
        assert_eq!(Grid::row_mask(2).to_string(), "____\n____\n****");
    }

    #[test]
    fn test_col_mask() {
        type Grid = TileSet256<4, 3, 12>;
        assert_eq!(Grid::col_mask(0).to_string(), "*___\n*___\n*___");
        assert_eq!(Grid::col_mask(1).to_string(), "_*__\n_*__\n_*__");
        assert_eq!(Grid::col_mask(2).to_string(), "__*_\n__*_\n__*_");
        assert_eq!(Grid::col_mask(3).to_string(), "___*\n___*\n___*");
    }

    #[test]
    fn test_get_scale() {
        type Grid = TileSet256<4, 3, 12>;

        let scale_square = Grid::get_scale(100.0, 100.0);
        let scale_rect = Grid::get_scale(100.0, 50.0);

        assert_eq!(scale_square, 25.0);
        assert_eq!(scale_rect, 16.666666)
    }

    #[test]
    fn test_all() {
        type Grid = TileSet256<4, 3, 12>;
        let all = Grid::all();

        assert_eq!("****\n****\n****", all.to_string().as_str())
    }

    #[test]
    fn test_is_empty() {
        type Grid = TileSet256<4, 3, 12>;
        assert!(Grid::EMPTY.is_empty());
        assert!(!Grid::EMPTY.with_bit_set(&Tile::NORTH_EAST, true).is_empty())
    }

    #[test]
    fn test_with_bit_set() {
        type Grid = TileSet256<4, 3, 12>;
        assert_eq!(
            "*___\n____\n____",
            Grid::EMPTY
                .with_bit_set(&Tile::NORTH_WEST, true)
                .to_string()
                .as_str()
        );
        assert_eq!(
            "_***\n****\n****",
            Grid::all()
                .with_bit_set(&Tile::NORTH_WEST, false)
                .to_string()
                .as_str()
        );
    }
}
