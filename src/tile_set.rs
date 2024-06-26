use core::{
    fmt::{self, Write},
    ops::{FnMut, Shl, Shr},
};

use crate::prelude::*;

#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};

macro_rules! tile_set {
    ($name:ident, $iter_name:ident, $true_iter_name:ident, $inner: ty) => {

        /// A grid
        /// A map from tiles to bools. Can store up to 256 tiles.
        #[must_use]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
        #[cfg_attr(any(test, feature = "serde"), derive(Serialize, Deserialize))]
        pub struct $name<const WIDTH: u8, const HEIGHT: u8, const SIZE: usize>($inner);

        impl<const WIDTH: u8, const HEIGHT: u8, const SIZE: usize> Default
            for $name<WIDTH, HEIGHT, SIZE>
        {
            fn default() -> Self {
                Self::EMPTY
            }
        }

        impl<const WIDTH: u8, const HEIGHT: u8, const SIZE: usize> $name<WIDTH, HEIGHT, SIZE> {

        /// The set where all tiles are missing
        pub const EMPTY: Self = {
            Self::assert_legal();
            Self(0)
        };

        /// The set where all tiles are present
        pub const ALL: Self = {
            Self::EMPTY.negate()
        };



        #[inline]
        const fn assert_legal() {
            debug_assert!(SIZE == (WIDTH as usize * HEIGHT as usize) );
            debug_assert!(SIZE <= <$inner>::BITS as usize);
        }


        #[inline]
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

        #[inline]
        pub const fn from_inner(inner: $inner) -> Self {
            Self::assert_legal();
            Self(inner)
        }

        #[must_use]
        #[inline]
        pub const fn into_inner(self) -> $inner {
            self.0
        }

        #[must_use]
        #[inline]
        pub const fn is_empty(self)-> bool{
            self.0 == Self::EMPTY.0
        }

        #[inline]
        pub fn set_bit(&mut self, tile: &Tile<WIDTH, HEIGHT>, bit: bool) {
            if bit {
                self.0 |= (1 as $inner << u32::from(tile.inner()));
            } else {
                self.0 &= !(1 as $inner << u32::from(tile.inner()));
            }
        }

        /// Returns a copy of self with the bit at `tile` set to `bit`
        #[inline]
        pub const fn with_bit_set(&self, tile: &Tile<WIDTH, HEIGHT>, bit: bool)-> Self{
            let inner =
            if bit {
                self.0 | (1 as $inner << tile.inner() as u32)
            } else {
                self.0 & !(1 as $inner << tile.inner() as u32)
            };

            Self(inner)
        }

        #[must_use]
        #[inline]
        pub const fn get_bit(&self, tile: &Tile<WIDTH, HEIGHT>) -> bool {
            (self.0 >> tile.inner() as u32) & 1 == 1
        }

        #[must_use]
        #[inline]
        pub const fn iter(&self) -> impl DoubleEndedIterator<Item = bool> + ExactSizeIterator {
            $iter_name::<1> {
                bottom_index: 0,
                top_index: SIZE,
                inner: self.0,
            }
        }

        #[inline]
        #[must_use]
        pub const fn row(&self, y: u8) -> impl DoubleEndedIterator<Item = bool> + ExactSizeIterator {
            $iter_name::<1> {
                bottom_index: (y * WIDTH) as usize,
                top_index: ((y + 1) * WIDTH) as usize,
                inner: self.0,
            }
        }

        #[inline]
        #[must_use]
        pub const fn col(&self, x: u8) -> impl DoubleEndedIterator<Item = bool> + ExactSizeIterator {

            $iter_name::<HEIGHT> {
                bottom_index: x as usize,
                top_index: ((WIDTH * (HEIGHT - 1)) + x + 1) as usize,
                inner: self.0,
            }
        }


        #[inline]
        #[allow(clippy::cast_possible_truncation)]
        pub fn shift_north(&self, rows: u8)-> Self{
            let a =self.0.shr(rows * WIDTH);
            let mask: $inner = <$inner>::MAX >> (<$inner>::BITS - SIZE as u32);
            Self(a & mask)
        }

        #[inline]
        #[allow(clippy::cast_possible_truncation)]
        pub fn shift_south(&self, rows: u8)-> Self{
            let a =self.0.shl(rows * WIDTH);
            let mask: $inner = <$inner>::MAX >> (<$inner>::BITS - SIZE as u32);
            Self(a & mask)
        }


    const ROW_ZERO_MASK: $inner = {
        let mut inner : $inner = 0;
        let mut tile = Some(Tile::<WIDTH, HEIGHT>::NORTH_WEST);

        while let Some(t) = tile {
            let i = t.inner();
            inner |= 1 << i;
            tile = t.const_add(&Vector::EAST);
        }
        inner
    };

    #[inline]
    pub const fn row_mask(y: u8) -> Self {
        Self::assert_legal();
        let inner = Self::ROW_ZERO_MASK << (y * WIDTH);

        Self(inner)
    }

    const COL_ZERO_MASK: $inner = {
        let mut inner : $inner = 0;
        let mut tile = Some(Tile::<WIDTH, HEIGHT>::NORTH_WEST);

        while let Some(t) = tile {
            let i = t.inner();
            inner |= 1 << i;
            tile = t.const_add(&Vector::SOUTH);
        }
        inner
    };

    #[inline]
    pub const fn col_mask(x: u8) -> Self {
        Self::assert_legal();
        let inner = Self::COL_ZERO_MASK << (x);

        Self(inner)
    }

    #[must_use]
    #[inline]
    pub fn enumerate(
        &self,
    ) -> impl DoubleEndedIterator<Item = (Tile<WIDTH, HEIGHT>, bool)> + ExactSizeIterator {
        self.iter()
            .enumerate()
            .map(|(i, x)| (Tile::try_from_usize(i).unwrap(), x))
    }

    #[must_use]
    #[inline]
    pub fn iter_true_tiles(&self) -> impl ExactSizeIterator<Item = Tile<WIDTH, HEIGHT>> + Clone + core::fmt::Debug  + core::iter::FusedIterator + DoubleEndedIterator {
        $true_iter_name::new(self)
    }

    #[must_use]
    #[inline]
    pub const fn count(&self) -> u32 {
        self.0.count_ones()
    }

    /// Get the scale to make the grid take up as much as possible of a given area
    #[must_use]
    #[inline]
    pub fn get_scale(total_width: f32, total_height: f32) -> f32 {
        let x_multiplier = total_width / f32::from(WIDTH);
        let y_multiplier = total_height / f32::from(HEIGHT);

        x_multiplier.min(y_multiplier)
    }

    #[inline]
    pub const fn intersect(&self, rhs: &Self) -> Self {
        Self(self.0 & rhs.0)
    }

    #[inline]
    pub const fn union(&self, rhs: &Self) -> Self {
        Self(self.0 | rhs.0)
    }

    #[must_use]
    #[inline]
    pub const fn is_subset(&self, rhs: &Self)-> bool{
        self.intersect(rhs).0 == self.0
    }

    #[must_use]
    #[inline]
    pub const fn is_superset(&self, rhs: &Self)-> bool{
        self.intersect(rhs).0 == rhs.0
    }

    /// Returns a new set containing all elements which belong to one set but not both
    #[inline]
    pub const fn symmetric_difference(&self, rhs: &Self)-> Self{
        Self(self.0 ^ rhs.0)
    }

    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn negate(&self) -> Self {
        let mask: $inner = <$inner>::MAX >> (<$inner>::BITS - SIZE as u32);
        Self(!self.0 & mask)
    }

    /// The first tile in this set
    #[must_use]
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn first(&self) -> Option<Tile<WIDTH, HEIGHT>>
    {
        Tile::<WIDTH, HEIGHT>::try_from_inner( self.0.trailing_zeros() as u8)
    }

    /// Removes the first tile in this set and returns it
    /// Returns `None` if the set is empty
    #[must_use]
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_lossless)]
    pub fn pop(&mut self) -> Option<Tile<WIDTH, HEIGHT>>
    {
        if self.0 == 0{
            return None;
        }
        let index = self.0.trailing_zeros() as $inner;
        self.0 &= !(1 as $inner << index);
        Some(Tile::<WIDTH, HEIGHT>::from_inner_unchecked(index as u8))
    }

    /// Removes the first tile in this set and returns it
    /// Returns `None` if the set is empty
    #[must_use]
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    pub fn pop_last(&mut self) -> Option<Tile<WIDTH, HEIGHT>>
    {
        if self.0 == 0{
            return None;
        }
        let index = <$inner>::BITS - 1 - self.0.leading_zeros();
        self.0 &= !(1 as $inner << index);
        Some(Tile::<WIDTH, HEIGHT>::from_inner_unchecked(index as u8))
    }

    /// The last tile in this set
    #[must_use]
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn last(&self)-> Option<Tile<WIDTH, HEIGHT>>
    {
        let Some(index) = (<$inner>::BITS - 1).checked_sub( self.0.leading_zeros()) else{return None;};

        Some(Tile::<WIDTH, HEIGHT>::from_inner_unchecked(index as u8))
    }
}

impl<const WIDTH: u8, const HEIGHT: u8, const SIZE: usize> FromIterator<Tile<WIDTH, HEIGHT>> for $name<WIDTH, HEIGHT, SIZE>{

    #[inline]
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item = Tile<WIDTH, HEIGHT>>
     {
        Self::assert_legal();
        let mut r = Self::default();
        for x in iter {
            r.set_bit(&x, true);
        }
        r
    }

}

#[derive(Clone, Debug)]
struct $true_iter_name<const WIDTH: u8, const HEIGHT: u8, const SIZE: usize> {
    inner: $name::<WIDTH, HEIGHT, SIZE>,
}

impl<const WIDTH: u8, const HEIGHT: u8, const SIZE: usize> core::iter::FusedIterator
    for $true_iter_name<WIDTH, HEIGHT, SIZE>{}

impl<const WIDTH: u8, const HEIGHT: u8, const SIZE: usize> ExactSizeIterator
    for $true_iter_name<WIDTH, HEIGHT, SIZE>
{
    #[inline]
    fn len(&self) -> usize {
        self.inner.count() as usize
    }
}

impl<const WIDTH: u8, const HEIGHT: u8, const SIZE: usize> Iterator
    for $true_iter_name<WIDTH, HEIGHT, SIZE>
{
    type Item = Tile<WIDTH, HEIGHT>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.pop()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.len();
        (size, Some(size))
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.inner.count() as usize
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.inner.last()
    }

    #[inline]
    fn max(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.last()
    }

    #[inline]
    fn min(mut self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.next()
    }

    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let mut n = n as u32;
        if self.inner.count() <= n{
            self.inner.0 = Default::default(); // Empty the set
            return None;
        }

        let  mut shift: u32 = 0;
        loop {
            let tz = self.inner.0.trailing_zeros();
            shift += tz;
            self.inner.0 >>= tz; //can't be all zeros as we checked the count

            let to = self.inner.0.trailing_ones();
            if let Some(new_n) = n.checked_sub(to) {
                n = new_n;
                self.inner.0 >>= to;
                shift += to;
            } else {
                self.inner.0 >>= n + 1;
                let r = Self::Item::try_from_inner((shift + n ) as u8);
                self.inner.0 <<= shift + n + 1;

                return r;
            }
        }
    }
}

impl<const WIDTH: u8, const HEIGHT: u8, const SIZE: usize> core::iter::DoubleEndedIterator
    for $true_iter_name<WIDTH, HEIGHT, SIZE>{
        #[inline]
        fn next_back(&mut self) -> Option<Self::Item> {
            self.inner.pop_last()
        }

        #[inline]
        #[allow(clippy::cast_possible_truncation)]
        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            let mut n = n as u32;
            if self.inner.count() <= n{
                self.inner.0 = Default::default(); // Empty the set
                return None;
            }

            let  mut shift: u32 = 0;
            loop {
                let lz = self.inner.0.leading_zeros();
                shift += lz;
                self.inner.0 <<= lz; //can't be all zeros as we checked the count

                let lo = self.inner.0.leading_ones();
                if let Some(new_n) = n.checked_sub(lo) {
                    n = new_n;
                    self.inner.0 <<= lo;
                    shift += lo;
                } else {
                    self.inner.0 <<= n + 1;
                    let r = Self::Item::try_from_inner((<$inner>::BITS - (shift + n + 1)) as u8);
                    self.inner.0 >>= shift + n + 1;

                    return r;
                }
            }
        }
    }

impl<const WIDTH: u8, const HEIGHT: u8, const SIZE: usize> $true_iter_name<WIDTH, HEIGHT, SIZE> {
    #[inline]
    pub fn new(set: & $name<WIDTH, HEIGHT, SIZE>) -> Self {
        Self {
            inner: set.clone()
        }
    }
}




#[derive(Clone, Debug)]
pub struct $iter_name<const STEP : u8> {
    inner: $inner,
    bottom_index: usize,
    top_index: usize,
}

impl<const STEP: u8> ExactSizeIterator for $iter_name<STEP> {
    #[inline]
    fn len(&self) -> usize {
        self.clone().count()
    }
}

impl<const STEP: u8> Iterator for $iter_name<STEP> {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {

        if self.bottom_index  >= self.top_index {
            None
        } else {
            let r = (self.inner >> self.bottom_index) & 1 == 1;
            self.bottom_index += (STEP as usize);
            Some(r)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.clone().count();
        (count, Some(count))
    }

    #[inline]
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
    #[inline]
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
                        if !f.alternate(){
                            f.write_char('\n')?;
                        }
                    }
                    if e {
                        f.write_char('*')?;
                    } else {
                        f.write_char('_')?;
                    }
                }

                Ok(())
            }
        }
    };
}

tile_set!(TileSet8, TileSetIter8, TrueTilesIter8, u8);
tile_set!(TileSet16, TileSetIter16, TrueTilesIter16, u16);
tile_set!(TileSet32, TileSetIter32, TrueTilesIter32, u32);
tile_set!(TileSet64, TileSetIter64, TrueTilesIter64, u64);
tile_set!(TileSet128, TileSetIter128, TrueTilesIter128, u128);

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn basic_tests() {
        let mut grid: TileSet16<3, 3, 9> = TileSet16::from_fn(|x| x.inner() % 2 == 0);

        assert_eq!(grid.to_string(), "*_*\n_*_\n*_*");
        assert_eq!(format!("{grid:#}"), "*_*_*_*_*");

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
        let grid_left: TileSet16<3, 3, 9> = TileSet16::from_fn(|x| x.x() == 1);
        let grid_right: TileSet16<3, 3, 9> = TileSet16::from_fn(|x| x.y() == 1);

        assert_eq!(
            grid_left.intersect(&grid_right).to_string(),
            "___\n\
             _*_\n\
             ___"
        )
    }

    #[test]
    fn test_union() {
        let grid_left: TileSet16<3, 3, 9> = TileSet16::from_fn(|x| x.x() == 0);
        let grid_top: TileSet16<3, 3, 9> = TileSet16::from_fn(|x| x.y() == 0);

        assert_eq!(
            grid_left.union(&grid_top).to_string(),
            "***\n\
         *__\n\
         *__"
        )
    }

    #[test]
    fn test_symmetric_difference() {
        let grid_left: TileSet16<3, 3, 9> = TileSet16::from_fn(|x| x.x() == 0);
        let grid_top: TileSet16<3, 3, 9> = TileSet16::from_fn(|x| x.y() == 0);

        assert_eq!(
            grid_left.symmetric_difference(&grid_top).to_string(),
            "_**\n\
         *__\n\
         *__"
        )
    }

    #[test]
    fn test_subset() {
        let grid_top: TileSet16<3, 3, 9> = TileSet16::from_fn(|x| x.y() == 0);
        let all: TileSet16<3, 3, 9> = TileSet16::ALL;

        assert!(grid_top.is_subset(&all));
        assert!(grid_top.is_subset(&grid_top));
        assert!(!all.is_subset(&grid_top));
    }

    #[test]
    fn test_superset() {
        let grid_top: TileSet16<3, 3, 9> = TileSet16::from_fn(|x| x.y() == 0);
        let all: TileSet16<3, 3, 9> = TileSet16::ALL;

        assert!(!grid_top.is_superset(&all));
        assert!(grid_top.is_superset(&grid_top));
        assert!(all.is_superset(&grid_top));
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
    fn test_shift() {
        let full_grid = TileSet16::<2, 3, 6>::default().negate();

        assert_eq!(full_grid.shift_north(0), full_grid);
        assert_eq!(full_grid.shift_south(0), full_grid);

        assert_eq!(full_grid.shift_north(1).to_string(), "**\n**\n__");
        assert_eq!(full_grid.shift_south(1).to_string(), "__\n**\n**");

        assert_eq!(full_grid.shift_north(2).to_string(), "**\n__\n__");
        assert_eq!(full_grid.shift_south(2).to_string(), "__\n__\n**");
    }

    #[test]
    fn test_row_mask() {
        type Grid = TileSet16<4, 3, 12>;
        assert_eq!(Grid::row_mask(0).to_string(), "****\n____\n____");
        assert_eq!(Grid::row_mask(1).to_string(), "____\n****\n____");
        assert_eq!(Grid::row_mask(2).to_string(), "____\n____\n****");
    }

    #[test]
    fn test_col_mask() {
        type Grid = TileSet16<4, 3, 12>;
        assert_eq!(Grid::col_mask(0).to_string(), "*___\n*___\n*___");
        assert_eq!(Grid::col_mask(1).to_string(), "_*__\n_*__\n_*__");
        assert_eq!(Grid::col_mask(2).to_string(), "__*_\n__*_\n__*_");
        assert_eq!(Grid::col_mask(3).to_string(), "___*\n___*\n___*");
    }

    #[test]
    fn test_get_scale() {
        type Grid = TileSet16<4, 3, 12>;

        let scale_square = Grid::get_scale(100.0, 100.0);
        let scale_rect = Grid::get_scale(100.0, 50.0);

        assert_eq!(scale_square, 25.0);
        assert_eq!(scale_rect, 16.666666)
    }

    #[test]
    fn test_all() {
        type Grid = TileSet16<4, 3, 12>;
        let all = Grid::ALL;

        assert_eq!("****\n****\n****", all.to_string().as_str())
    }

    #[test]
    fn test_is_empty() {
        type Grid = TileSet16<4, 3, 12>;
        assert!(Grid::EMPTY.is_empty());
        assert!(!Grid::EMPTY.with_bit_set(&Tile::NORTH_EAST, true).is_empty())
    }

    #[test]
    fn test_with_bit_set() {
        type Grid = TileSet16<4, 3, 12>;
        assert_eq!(
            "*___\n____\n____",
            Grid::EMPTY
                .with_bit_set(&Tile::NORTH_WEST, true)
                .to_string()
                .as_str()
        );
        assert_eq!(
            "_***\n****\n****",
            Grid::ALL
                .with_bit_set(&Tile::NORTH_WEST, false)
                .to_string()
                .as_str()
        );
    }

    #[test]
    fn test_iter_length_and_count() {
        type Iter = TileSetIter16<2>;

        let iter = Iter {
            inner: 0,
            bottom_index: 0,
            top_index: 12,
        };

        let len = iter.len();

        assert_eq!(len, 6);

        let count = iter.count();

        assert_eq!(count, 6);
    }

    #[test]
    fn test_true_iter_length_and_count() {
        type Grid = TileSet16<4, 3, 12>;

        let mut iter = Grid::ALL.iter_true_tiles();

        assert_eq!(12, iter.len());
        assert_eq!(12, iter.clone().count());

        let _ = iter.next();
        assert_eq!(11, iter.len());
        assert_eq!(11, iter.count());
    }

    #[test]
    fn test_true_iter_min_max_last() {
        type Grid = TileSet16<4, 3, 12>;

        let iter = Grid::from_fn(|x| x.inner() % 5 == 0).iter_true_tiles();

        assert_eq!(0, iter.clone().min().unwrap().inner());
        assert_eq!(10, iter.clone().max().unwrap().inner());
        assert_eq!(10, iter.last().unwrap().inner());
    }

    #[test]
    fn test_first() {
        let mut set = TileSet16::<4, 3, 12>::from_fn(|tile| tile.x() > tile.y());

        let expected: Vec<_> = Tile::<4, 3>::iter_by_row()
            .filter(|tile| tile.x() > tile.y())
            .collect();
        let mut actual: Vec<Tile<4, 3>> = vec![];

        while let Some(first) = set.first() {
            set.set_bit(&first, false);
            actual.push(first);
        }

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_last() {
        let mut set = TileSet16::<4, 3, 12>::from_fn(|tile| tile.x() > tile.y());

        let mut expected: Vec<_> = Tile::<4, 3>::iter_by_row()
            .filter(|tile| tile.x() > tile.y())
            .collect();
        expected.reverse();
        let mut actual: Vec<Tile<4, 3>> = vec![];

        while let Some(last) = set.last() {
            set.set_bit(&last, false);
            actual.push(last);
        }

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_iter_true_rev() {
        let set = TileSet16::<4, 3, 12>::from_fn(|tile| tile.x() > tile.y());

        let mut expected: Vec<_> = Tile::<4, 3>::iter_by_row()
            .filter(|tile| tile.x() > tile.y())
            .collect();
        expected.reverse();
        let actual: Vec<Tile<4, 3>> = set.iter_true_tiles().rev().collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_true_iter_nth() {
        let set: TileSet64<9, 7, 63> = TileSet64::<9, 7, 63>::from_fn(|tile| tile.x() > tile.y());
        let vec: Vec<_> = Tile::<9, 7>::iter_by_row()
            .filter(|tile| tile.x() > tile.y())
            .collect();

        let mut set_iter = set.iter_true_tiles();
        let mut vec_iter = vec.into_iter();

        for n in [0usize, 1, 0, 5, 0, 1, 0, 40] {
            assert_eq!(set_iter.len(), vec_iter.len());
            let actual = set_iter.nth(n);
            let expected = vec_iter.nth(n);
            //println!("n: {n} actual: {actual:?} expected {expected:?}");
            assert_eq!(actual, expected);
        }
        assert_eq!(set_iter.len(), vec_iter.len());
    }

    #[test]
    fn test_true_iter_nth_back() {
        let set: TileSet64<9, 7, 63> = TileSet64::<9, 7, 63>::from_fn(|tile| tile.x() > tile.y());
        let vec: Vec<_> = Tile::<9, 7>::iter_by_row()
            .filter(|tile| tile.x() > tile.y())
            .collect();

        let mut set_iter = set.iter_true_tiles();
        let mut vec_iter = vec.into_iter();

        for n in [0usize, 1, 0, 5, 0, 1, 0, 40] {
            assert_eq!(set_iter.len(), vec_iter.len());
            let actual = set_iter.nth_back(n);
            let expected = vec_iter.nth_back(n);
            //println!("n: {n} actual: {actual:?} expected {expected:?}");
            assert_eq!(actual, expected);
        }
        assert_eq!(set_iter.len(), vec_iter.len());
    }
}
