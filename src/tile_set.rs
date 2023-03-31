use core::{
    fmt::{self, Write},
    iter,
    ops::*,
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
                Self::EMPTY
            }
        }

        impl<const COLS: u8, const ROWS: u8, const SIZE: usize> $name<COLS, ROWS, SIZE> {

            pub const EMPTY: Self = {
                Self::assert_legal();
                Self(0)
            };

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
                $iter_name::<1> {
                    bottom_index: 0,
                    top_index: SIZE,
                    inner: self.0,
                }
            }

        #[must_use]
        pub const fn row(&self, row: u8) -> impl DoubleEndedIterator<Item = bool> + ExactSizeIterator {
            $iter_name::<1> {
                bottom_index: (row * COLS) as usize,
                top_index: ((row + 1) * COLS) as usize,
                inner: self.0,
            }
        }

        #[must_use]
        pub const fn col(&self, col: u8) -> impl DoubleEndedIterator<Item = bool> + ExactSizeIterator {

            $iter_name::<ROWS> {
                bottom_index: col as usize,
                top_index: ((COLS * (ROWS - 1)) + col + 1) as usize,
                inner: self.0,
            }
        }

        #[must_use]
        pub fn shift_north(&self, rows: u8)-> Self{
            let a =self.0.shr(rows * COLS);
            let mask: $inner = <$inner>::MAX >> (<$inner>::BITS - SIZE as u32);
            Self(a & mask)
        }

        #[must_use]
        pub fn shift_south(&self, rows: u8)-> Self{
            let a =self.0.shl(rows * COLS);
            let mask: $inner = <$inner>::MAX >> (<$inner>::BITS - SIZE as u32);
            Self(a & mask)
        }

        #[must_use]
    #[inline]
    pub const fn row_mask(row: u8) -> Self {
        Self::assert_legal();
        let mut inner : $inner = 0;
        let mut tile = Tile::<COLS, ROWS>::try_new(0, row);

        while let Some(t) = tile {
            let i = t.inner();
            inner |= 1 << i;
            tile = t.const_add(&Vector::EAST);
        }

        Self(inner)
    }

    #[must_use]
    #[inline]
    pub const fn col_mask(col: u8) -> Self {
        Self::assert_legal();
        let mut inner : $inner = 0;
        let mut tile = Tile::<COLS, ROWS>::try_new(col,  0);

        while let Some(t) = tile {
            let i = t.inner();
            inner |= 1 << i;
            tile = t.const_add(&Vector::SOUTH);
        }

        Self(inner)
    }

    #[must_use]
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
                Self(self.0 & rhs.0)
            }

            #[must_use]
            pub const fn union(&self, rhs: &Self) -> Self {
                Self(self.0 | rhs.0)
            }

            #[must_use]
            pub const fn negate(&self) -> Self {
                let mask: $inner = <$inner>::MAX >> (<$inner>::BITS - SIZE as u32);
                Self(!self.0 & mask)
            }
        }


#[derive(Debug, Clone, Copy)]
pub struct $iter_name<const STEP : u8> {
    inner: $inner,
    bottom_index: usize,
    top_index: usize,
}

impl<const STEP: u8> ExactSizeIterator for $iter_name<STEP> {
    fn len(&self) -> usize {
        self.count()
    }
}

impl<const STEP: u8> Iterator for $iter_name<STEP> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {

        if self.bottom_index  >= self.top_index {
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

impl<const STEP: u8> DoubleEndedIterator for $iter_name<STEP> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.top_index == 0{
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
        let grid_left: TileSet16<3, 3, 9> = TileSet16::from_fn(|x| x.col() == 1);
        let grid_right: TileSet16<3, 3, 9> = TileSet16::from_fn(|x| x.row() == 1);

        assert_eq!(
            grid_left.intersect(&grid_right).to_string(),
            "___\n_*_\n___"
        )
    }

    #[test]
    fn test_union() {
        let grid_left: TileSet16<3, 3, 9> = TileSet16::from_fn(|x| x.col() == 0);
        let grid_top: TileSet16<3, 3, 9> = TileSet16::from_fn(|x| x.row() == 0);

        assert_eq!(grid_left.union(&grid_top).to_string(), "***\n*__\n*__")
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
        let grid = TileSet16::<4, 3, 12>::from_fn(|x| x.inner() >= 6);

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
        let grid = TileSet16::<4, 3, 12>::from_fn(|x| x.inner() % 3 == 1);

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
        let grid = TileSet16::<4, 3, 12>::from_fn(|x| x.inner() % 2 == 1);

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
        let grid = TileSet16::<3, 3, 9>::from_fn(|x| x.inner() == 5);

        assert_eq!(
            grid.enumerate()
                .map(|(t, x)| t.inner().to_string() + x.then(|| "*").unwrap_or("_"))
                .join(""),
            "0_1_2_3_4_5*6_7_8_"
        );
    }

    #[test]
    fn test_shift(){
        let full_grid  = TileSet16::<2,3,6>::default().negate();

        assert_eq!(full_grid.shift_north(0), full_grid);
        assert_eq!(full_grid.shift_south(0), full_grid);

        assert_eq!(full_grid.shift_north(1).to_string(), "**\n**\n__");
        assert_eq!(full_grid.shift_south(1).to_string(), "__\n**\n**");

        assert_eq!(full_grid.shift_north(2).to_string(), "**\n__\n__");
        assert_eq!(full_grid.shift_south(2).to_string(), "__\n__\n**");
    }


    #[test]
    fn test_row_mask(){
        type Grid = TileSet16::<4, 3, 12>;
        assert_eq!(Grid::row_mask(0).to_string(), "****\n____\n____");
        assert_eq!(Grid::row_mask(1).to_string(), "____\n****\n____");
        assert_eq!(Grid::row_mask(2).to_string(), "____\n____\n****");
    }


    #[test]
    fn test_col_mask(){
        type Grid = TileSet16::<4, 3, 12>;
        assert_eq!(Grid::col_mask(0).to_string(), "*___\n*___\n*___");
        assert_eq!(Grid::col_mask(1).to_string(), "_*__\n_*__\n_*__");
        assert_eq!(Grid::col_mask(2).to_string(), "__*_\n__*_\n__*_");
        assert_eq!(Grid::col_mask(3).to_string(), "___*\n___*\n___*");
    }
}
