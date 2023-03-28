use core::{
    fmt::{self, Write},
    iter,
    ops::{Index, IndexMut},
};

use crate::prelude::*;

#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};

macro_rules! tile_set {
    ($name:ident,$iter_name:ident, $inner: ty) => {
        #[must_use]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
        #[cfg_attr(any(test, feature = "serde"), derive(Serialize, Deserialize))]
        pub struct $name<const COLS: u8, const ROWS: u8, const SIZE: usize>($inner);

        impl<const COLS: u8, const ROWS: u8, const SIZE: usize> Default
            for $name<COLS, ROWS, SIZE>
        {
            fn default() -> Self {
                Self::assert_legal();
                Self(Default::default())
            }
        }

        impl<const COLS: u8, const ROWS: u8, const SIZE: usize> $name<COLS, ROWS, SIZE> {
            #[inline]
            const fn assert_legal() {
                debug_assert!(SIZE == (COLS as usize * ROWS as usize) );
                debug_assert!(SIZE <= <$inner>::BITS as usize);
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
            pub const fn from_inner(inner: $inner) -> Self {
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
            pub const fn into_inner(self) -> $inner {
                self.0
            }

            #[inline]
            pub fn set_bit(&mut self, tile: &Tile<COLS, ROWS>, bit: bool) {
                if bit {
                    self.0 |= (1 as $inner << tile.inner() as u32);
                } else {
                    self.0 &= !(1 as $inner << tile.inner() as u32);
                }
            }

            #[must_use]
            #[inline]
            pub const fn get_bit(&self, tile: &Tile<COLS, ROWS>) -> bool {
                (self.0 >> tile.inner() as u32) & 1 == 1
            }

            #[must_use]
            pub const fn iter(&self) -> impl DoubleEndedIterator<Item = bool> + ExactSizeIterator {
                $iter_name {
                    bottom_index: 0,
                    top_index: SIZE,
                    inner: self.0,
                }
            }

            pub const fn row(&self, row: u8) -> impl DoubleEndedIterator<Item = bool> + ExactSizeIterator {
                $iter_name {
                    bottom_index: ((row * COLS)) as usize,
                    top_index: ((row + 1) * COLS) as usize,
                    inner: self.0,
                }
            }

            pub fn enumerate(
                &self,
            ) -> impl DoubleEndedIterator<Item = (Tile<COLS, ROWS>, bool)> + ExactSizeIterator {
                self.iter()
                    .enumerate()
                    .map(|(i, x)| (Tile::try_from_usize(i).unwrap(), x))
            }

            #[must_use]
            pub fn iter_true_tiles(&self) -> impl Iterator<Item = Tile<COLS, ROWS>> {
                self.iter()
                    .enumerate()
                    .flat_map(|(i, b)| b.then(|| Tile::try_from_usize(i).unwrap()))
            }

            #[must_use]
            pub const fn count(&self) -> u32 {
                self.0.count_ones()
            }

            /// Get the scale to make the grid take up as much as possible of a given area
            #[must_use]
            pub fn get_scale(total_width: f32, total_height: f32) -> f32 {
                let x_multiplier = total_width / COLS as f32;
                let y_multiplier = total_height / ROWS as f32;

                x_multiplier.min(y_multiplier)
            }

            #[must_use]
            pub const fn intersect(&self, rhs: &Self) -> Self {
                Self(self.0 | rhs.0)
            }

            #[must_use]
            pub const fn union(&self, rhs: &Self) -> Self {
                Self(self.0 & rhs.0)
            }

            #[must_use]
            pub const fn negate(&self) -> Self {
                let mask: $inner = <$inner>::MAX >> (<$inner>::BITS - SIZE as u32);
                Self(!self.0 & mask)
            }
        }

            #[derive(Debug, Clone, Copy)]
    pub struct $iter_name {
        inner: $inner,
        bottom_index: usize,
        top_index: usize,
    }

    impl ExactSizeIterator for $iter_name {
        fn len(&self) -> usize {
            self.count()
        }
    }

    impl Iterator for $iter_name {
        type Item = bool;

        fn next(&mut self) -> Option<Self::Item> {
            if self.bottom_index >= self.top_index {
                None
            } else {
                let r = (self.inner >> self.bottom_index) & 1 == 1;
                self.bottom_index += 1;
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
            (self.top_index - self.bottom_index) as usize
        }
    }

    impl DoubleEndedIterator for $iter_name {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.bottom_index >= self.top_index {
                None
            } else {
                self.top_index -= 1;
                let r = (self.inner >> self.top_index) & 1 == 1;

                Some(r)
            }
        }
    }

        impl<const W: u8, const H: u8, const SIZE: usize> fmt::Display for $name<W, H, SIZE> {
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
    };
}

tile_set!(TileSet8, TileSetIter8, u8);
tile_set!(TileSet16, TileSetIter16, u16);
tile_set!(TileSet32, TileSetIter32, u32);
tile_set!(TileSet64, TileSetIter64, u64);
tile_set!(TileSet128, TileSetIter128, u128);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use itertools::Itertools;
    #[cfg(any(test, feature = "serde"))]
    use serde_test::{assert_tokens, Token};

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_bad_set16() {
        let mut grid: TileSet16<3, 3, 10> = TileSet16::default();
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_bad_set8() {
        let mut grid: TileSet8<3, 3, 9> = TileSet8::default();
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_bad_set32() {
        let mut grid: TileSet32<6, 6, 36> = TileSet32::default();
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_bad_set64() {
        let mut grid: TileSet64<8, 8, 65> = TileSet64::default();
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_bad_set128() {
        let mut grid: TileSet128<12, 11, 132> = TileSet128::default();
    }

    #[test]
    fn basic_tests() {
        let mut grid: TileSet16<3, 3, 9> = TileSet16::from_fn(|x| x.inner() % 2 == 0);

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
        let grid_left: TileSet16<3, 3, 9> = TileSet16::from_fn(|x| x.col() == 0);
        let grid_right: TileSet16<3, 3, 9> = TileSet16::from_fn(|x| x.col() == 2);

        assert_eq!(
            grid_left.intersect(&grid_right).to_string(),
            "*_*\n*_*\n*_*"
        )
    }

    #[test]
    fn test_union() {
        let grid_left: TileSet16<3, 3, 9> = TileSet16::from_fn(|x| x.col() == 0);
        let grid_top: TileSet16<3, 3, 9> = TileSet16::from_fn(|x| x.row() == 0);

        assert_eq!(grid_left.union(&grid_top).to_string(), "*__\n___\n___")
    }

    #[test]
    fn test_from_inner() {
        assert_eq!(
            TileSet16::<3, 3, 9>::from_inner(3).to_string(),
            "**_\n___\n___"
        )
    }

    #[test]
    fn test_from_iter() {
        let grid = TileSet16::<3, 3, 9>::from_iter(
            [
                Tile::try_from_inner(0).unwrap(),
                Tile::try_from_inner(1).unwrap(),
            ]
            .into_iter(),
        );
        assert_eq!(grid.to_string(), "**_\n___\n___")
    }

    #[test]
    fn test_iter_reverse() {
        let grid = TileSet16::<3, 3, 9>::from_fn(|x| x.inner() >= 6);

        assert_eq!(
            grid.iter()
                .rev()
                .map(|x| x.then(|| "*").unwrap_or("_"))
                .join(""),
            "***______"
        );
    }

    #[test]
    fn test_row() {
        let grid = TileSet16::<3, 3, 9>::from_fn(|x| x.inner() % 2 == 1);

        assert_eq!(
            grid.row(0).map(|x| x.then(|| "*").unwrap_or("_")).join(""),
            "_*_"
        );
        assert_eq!(
            grid.row(1).map(|x| x.then(|| "*").unwrap_or("_")).join(""),
            "*_*"
        );
        assert_eq!(
            grid.row(2).map(|x| x.then(|| "*").unwrap_or("_")).join(""),
            "_*_"
        );
    }

    #[test]
    fn test_enumerate(){
        let grid = TileSet16::<3, 3, 9>::from_fn(|x| x.inner() == 5 );

        assert_eq!(grid.enumerate().map(|(t,x)| t.inner().to_string() + x.then(|| "*").unwrap_or("_") ).join(""), "0_1_2_3_4_5*6_7_8_");
    }
}
