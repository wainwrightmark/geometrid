use core::{
    fmt::{self, Write},
    iter,
    ops::{Index, IndexMut, Shl, Shr},
};

use crate::prelude::*;

use ethnum::U256;
#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};

#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(any(test, feature = "serde"), derive(Serialize, Deserialize))]
pub struct TileSet256<const COLS: u8, const ROWS: u8, const SIZE: usize>(U256);

impl<const COLS: u8, const ROWS: u8, const SIZE: usize> Default for TileSet256<COLS, ROWS, SIZE> {
    fn default() -> Self {
        Self::assert_legal();
        Self(Default::default())
    }
}

impl<const COLS: u8, const ROWS: u8, const SIZE: usize> TileSet256<COLS, ROWS, SIZE> {
    #[inline]
    const fn assert_legal() {
        debug_assert!(SIZE == (COLS as usize * ROWS as usize));
        debug_assert!(SIZE <= <U256>::BITS as usize);
    }

    #[must_use]
    pub fn from_fn<F: FnMut(Tile<COLS, ROWS>) -> bool>(mut cb: F) -> Self {
        Self::assert_legal();

        let mut result = Self::default();
        for tile in Tile::<COLS, ROWS>::iter_by_row() {
            if cb(tile) {
                result.set_bit(&tile, true);
            }
        }

        result
    }

    #[must_use]
    #[inline]
    pub const fn from_inner(inner: U256) -> Self {
        Self::assert_legal();
        Self(inner)
    }

    pub fn from_iter(iter: impl Iterator<Item = Tile<COLS, ROWS>>) -> Self {
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

    #[inline]
    pub fn set_bit(&mut self, tile: &Tile<COLS, ROWS>, bit: bool) {
        if bit {
            self.0 |= U256::ONE.shl(tile.inner());
        } else {
            self.0 &= !(U256::ONE.shl(tile.inner()));
        }
    }

    #[must_use]
    #[inline]
    pub fn get_bit(&self, tile: &Tile<COLS, ROWS>) -> bool {
        (self.0.shr(tile.inner())) & U256::ONE == U256::ONE
    }

    #[must_use]
    pub const fn iter(&self) -> impl Iterator<Item = bool> {
        TileSetIter256::<SIZE> {
            index: 0,
            inner: self.0,
        }
    }

    #[must_use]
    pub fn iter_true_tiles(&self) -> impl Iterator<Item = Tile<COLS, ROWS>> {
        self.iter()
            .enumerate()
            .flat_map(|(i, b)| b.then(|| Tile::try_from_usize(i).unwrap()))
    }

    #[must_use]
    pub const fn count(&self) -> usize {
        self.0.count_ones() as usize
    }

    #[must_use]
    pub fn intersect(&self, rhs: &Self) -> Self {
        Self(self.0 | rhs.0)
    }

    #[must_use]
    pub fn union(&self, rhs: &Self) -> Self {
        Self(self.0 & rhs.0)
    }

    #[must_use]
    pub fn negate(&self) -> Self {
        let mask: U256 = <U256>::MAX >> (<U256>::BITS - SIZE as u32);
        Self(!self.0 & mask)
    }
}

#[derive(Debug, Clone)]
pub struct TileSetIter256<const SIZE: usize> {
    inner: U256,
    index: usize,
}

impl<const SIZE: usize> Iterator for TileSetIter256<SIZE> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= SIZE {
            None
        } else {
            let r = (self.inner >> self.index) & 1 == 1;
            self.index += 1;
            Some(r)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            (SIZE - self.index) as usize,
            Some((SIZE - self.index) as usize),
        )
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        (SIZE - self.index) as usize
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
    #[should_panic(expected = "assertion failed")]
    fn test_wrong_size() {
        let mut grid: TileSet256<16, 16, 255> = TileSet256::default();
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_too_big() {
        let mut grid: TileSet256<17, 16, 264> = TileSet256::default();
    }

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
        let grid_left: TileSet256<3, 3, 9> = TileSet256::from_fn(|x| x.col() == 0);
        let grid_right: TileSet256<3, 3, 9> = TileSet256::from_fn(|x| x.col() == 2);

        assert_eq!(
            grid_left.intersect(&grid_right).to_string(),
            "*_*\n*_*\n*_*"
        )
    }

    #[test]
    fn test_union() {
        let grid_left: TileSet256<3, 3, 9> = TileSet256::from_fn(|x| x.col() == 0);
        let grid_top: TileSet256<3, 3, 9> = TileSet256::from_fn(|x| x.row() == 0);

        assert_eq!(grid_left.union(&grid_top).to_string(), "*__\n___\n___")
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
        let grid = TileSet256::<3, 3, 9>::from_iter(
            [
                Tile::try_from_inner(0).unwrap(),
                Tile::try_from_inner(1).unwrap(),
            ]
            .into_iter(),
        );
        assert_eq!(grid.to_string(), "**_\n___\n___")
    }
}