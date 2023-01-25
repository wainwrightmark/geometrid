use core::{
    fmt::{self, Write},
    iter,
    ops::{Index, IndexMut},
};

use crate::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

macro_rules! tile_grid {
    ($name:ident, $tile:ident, $inner:ty) => {
        #[must_use]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct $name<T, const WIDTH: $inner, const HEIGHT: $inner, const SIZE: usize>(
            #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
            #[cfg_attr(feature = "serde", serde(bound(serialize = "T: Serialize")))]
            #[cfg_attr(feature = "serde", serde(bound(deserialize = "T: Deserialize<'de>")))]
            [T; SIZE],
        );

        impl<T: Default + Copy, const W: $inner, const H: $inner, const SIZE: usize> Default
            for $name<T, W, H, SIZE>
        {
            fn default() -> Self {
                debug_assert!(SIZE == (W * H) as usize);
                Self([T::default(); SIZE])
            }
        }

        impl<T, const W: $inner, const H: $inner, const SIZE: usize> $name<T, W, H, SIZE> {
            #[must_use]
            pub fn from_fn<F: FnMut($tile<W, H>) -> T>(mut cb: F) -> Self {
                debug_assert!(SIZE == (W * H) as usize);
                let arr = core::array::from_fn(|i| cb($tile::try_from_usize(i).unwrap()));
                Self(arr)
            }

            #[must_use]
            #[inline]
            pub fn into_inner(self) -> [T; SIZE] {
                self.0
            }

            #[must_use]
            #[inline]
            pub fn from_inner(inner: [T; SIZE]) -> Self {
                debug_assert!(SIZE == (W * H) as usize);
                Self(inner)
            }

            #[must_use]
            #[inline]
            pub fn enumerate(&self) -> impl iter::Iterator<Item = ($tile<W, H>, &'_ T)> {
                self.0
                    .iter()
                    .enumerate()
                    .map(|(inner, x)| ($tile::try_from_usize(inner).unwrap(), x))
            }

            #[inline]
            pub fn swap(&mut self, p1: $tile<W, H>, p2: $tile<W, H>) {
                self.0.swap(p1.into(), p2.into());
            }

            #[must_use]
            #[inline]
            pub fn iter(&self) -> core::slice::Iter<'_, T> {
                self.0.iter()
            }

            #[must_use]
            #[inline]
            pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
                self.0.iter_mut()
            }

            #[must_use]
            #[inline]
            pub fn row(&self, row: $inner) -> &[T] {
                let start = row * W;
                let end = start + W;
                &self.0[(start as usize)..(end as usize)]
            }

            #[must_use]
            #[inline]
            pub fn row_mut(&mut self, row: $inner) -> &mut [T] {
                let start = row * W;
                let end = start + W;
                &mut self.0[(start as usize)..(end as usize)]
            }

            #[must_use]
            pub fn column_iter(&self, column: $inner) -> impl DoubleEndedIterator<Item = &T> + '_ {
                (0..H)
                    .map(move |row| column + (row * W))
                    .map(|x| &self.0[x as usize])
            }

            /// Get the scale to make the grid take up as much as possible of a given area
            #[must_use]
            fn get_scale(total_width: f32, total_height: f32) -> f32 {
                let x_multiplier = total_width / W as f32;
                let y_multiplier = total_height / H as f32;

                x_multiplier.min(y_multiplier)
            }
        }

        impl<T, const COLS: $inner, const ROWS: $inner, const SIZE: usize> Flippable
            for $name<T, COLS, ROWS, SIZE>
        {
            fn flip_horizontal(&mut self) {
                for y in 0..ROWS {
                    for x in 0..COLS / 2 {
                        let p1 = $tile::<COLS, ROWS>::new_unchecked(x, y);
                        let mut p2 = p1;
                        p2.flip_horizontal();
                        self.swap(p1, p2);
                    }
                }
            }

            fn flip_vertical(&mut self) {
                for y in 0..ROWS / 2 {
                    for x in 0..COLS {
                        let p1 = $tile::<COLS, ROWS>::try_new(x, y).unwrap();
                        let mut p2 = p1;
                        p2.flip_vertical();
                        self.swap(p1, p2);
                    }
                }
            }
        }

        impl<T, const L: $inner, const SIZE: usize> Rotatable for $name<T, L, L, SIZE> {
            fn rotate(&mut self, quarter_turns: QuarterTurns) {
                match quarter_turns {
                    QuarterTurns::Zero => {
                        return;
                    }
                    QuarterTurns::One => {
                        for row in 0..=(L / 2) {
                            for column in row..=(L / 2) {
                                let o_row = L - (1 + row);
                                let o_column = L - (1 + column);
                                if row != o_row || column != o_column {
                                    let p0 = $tile::new_unchecked(column, row);
                                    let p1 = $tile::new_unchecked(row, o_column);
                                    let p2 = $tile::new_unchecked(o_column, o_row);
                                    let p3 = $tile::new_unchecked(o_row, column);

                                    //0123
                                    self.swap(p0, p3); //3120
                                    self.swap(p0, p1); //1320
                                    self.swap(p1, p2); //1230
                                }
                            }
                        }
                    }
                    QuarterTurns::Two => {
                        for row in 0..=(L / 2) {
                            for column in row..=(L / 2) {
                                let o_row = L - (1 + row);
                                let o_column = L - (1 + column);
                                if row != o_row || column != o_column {
                                    let p0 = $tile::new_unchecked(column, row);
                                    let p1 = $tile::new_unchecked(row, o_column);
                                    let p2 = $tile::new_unchecked(o_column, o_row);
                                    let p3 = $tile::new_unchecked(o_row, column);

                                    //0123
                                    self.swap(p0, p3); //3120
                                    self.swap(p1, p2); //3210
                                }
                            }
                        }
                    }
                    QuarterTurns::Three => {
                        for row in 0..=(L / 2) {
                            for column in row..=(L / 2) {
                                let o_row = L - (1 + row);
                                let o_column = L - (1 + column);
                                if row != o_row || column != o_column {
                                    let p0 = $tile::new_unchecked(column, row);
                                    let p1 = $tile::new_unchecked(row, o_column);
                                    let p2 = $tile::new_unchecked(o_column, o_row);
                                    let p3 = $tile::new_unchecked(o_row, column);

                                    //0123
                                    self.swap(p1, p2); //0213
                                    self.swap(p0, p1); //2013
                                    self.swap(p0, p3); //3012
                                }
                            }
                        }
                    }
                }
            }
        }

        impl<T, const W: $inner, const H: $inner, const SIZE: usize> Index<$tile<W, H>>
            for $name<T, W, H, SIZE>
        {
            type Output = T;

            fn index(&self, index: $tile<W, H>) -> &Self::Output {
                let u: usize = index.into();
                &self.0[u]
            }
        }

        impl<T, const W: $inner, const H: $inner, const SIZE: usize> IndexMut<$tile<W, H>>
            for $name<T, W, H, SIZE>
        {
            fn index_mut(&mut self, index: $tile<W, H>) -> &mut Self::Output {
                let u: usize = index.into();
                &mut self.0[u]
            }
        }

        impl<T, const W: $inner, const H: $inner, const SIZE: usize> AsRef<[T; SIZE]>
            for $name<T, W, H, SIZE>
        {
            #[inline]
            fn as_ref(&self) -> &[T; SIZE] {
                &self.0
            }
        }

        impl<T, const W: $inner, const H: $inner, const SIZE: usize> AsMut<[T; SIZE]>
            for $name<T, W, H, SIZE>
        {
            #[inline]
            fn as_mut(&mut self) -> &mut [T; SIZE] {
                &mut self.0
            }
        }

        impl<'a, T, const W: $inner, const H: $inner, const SIZE: usize> IntoIterator
            for &'a $name<T, W, H, SIZE>
        {
            type Item = &'a T;
            type IntoIter = core::slice::Iter<'a, T>;
            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.0.iter()
            }
        }

        impl<'a, T, const W: $inner, const H: $inner, const SIZE: usize> IntoIterator
            for &'a mut $name<T, W, H, SIZE>
        {
            type Item = &'a mut T;
            type IntoIter = core::slice::IterMut<'a, T>;
            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.0.iter_mut()
            }
        }

        impl<T, const W: $inner, const H: $inner, const SIZE: usize> IntoIterator
            for $name<T, W, H, SIZE>
        {
            type Item = T;
            type IntoIter = core::array::IntoIter<T, SIZE>;
            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                IntoIterator::into_iter(self.0)
            }
        }

        impl<T: fmt::Display, const W: $inner, const H: $inner, const SIZE: usize> fmt::Display
            for $name<T, W, H, SIZE>
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let iter = self.0.iter().enumerate();

                for (i, e) in iter {
                    if i == 0 {
                    } else if i % (W as usize) == 0 {
                        f.write_char('\n')?;
                    } else {
                        f.write_char('|')?;
                    }

                    e.fmt(f)?;
                }

                Ok(())
            }
        }
    };
}

tile_grid!(TileGrid16, Tile16, u16);
tile_grid!(TileGrid8, Tile8, u8);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use itertools::Itertools;
    #[cfg(feature = "serde")]
    use serde_test::{assert_tokens, Token};

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_bad_grid() {
        let mut grid: TileGrid8<usize, 3, 3, 10> = TileGrid8::default();
    }

    #[test]
    fn test_flip_vertical() {
        let mut grid: TileGrid8<usize, 3, 3, 9> = TileGrid8::from_fn(|x| x.into());

        assert_eq!(grid.to_string(), "0|1|2\n3|4|5\n6|7|8");
        grid.flip_vertical();
        assert_eq!(grid.to_string(), "6|7|8\n3|4|5\n0|1|2");
    }

    #[test]
    fn test_flip_horizontal() {
        let mut grid: TileGrid8<usize, 3, 3, 9> = TileGrid8::from_fn(|x| x.into());

        assert_eq!(grid.to_string(), "0|1|2\n3|4|5\n6|7|8");
        grid.flip_horizontal();
        assert_eq!(grid.to_string(), "2|1|0\n5|4|3\n8|7|6");
    }

    #[test]
    fn rotate_clockwise_3() {
        let mut grid: TileGrid8<usize, 3, 3, 9> = TileGrid8::from_fn(|x| x.into());

        assert_eq!(grid.to_string(), "0|1|2\n3|4|5\n6|7|8");
        grid.rotate(QuarterTurns::One);
        assert_eq!(grid.to_string(), "6|3|0\n7|4|1\n8|5|2");
        grid.rotate(QuarterTurns::One);
        assert_eq!(grid.to_string(), "8|7|6\n5|4|3\n2|1|0");
        grid.rotate(QuarterTurns::One);
        assert_eq!(grid.to_string(), "2|5|8\n1|4|7\n0|3|6");
        grid.rotate(QuarterTurns::One);
        assert_eq!(grid.to_string(), "0|1|2\n3|4|5\n6|7|8");
    }

    #[test]
    fn rotate_clockwise_4() {
        let mut grid: TileGrid8<usize, 4, 4, 16> = TileGrid8::from_fn(|x| x.into());

        assert_eq!(grid.to_string(), "0|1|2|3\n4|5|6|7\n8|9|10|11\n12|13|14|15");
        grid.rotate(QuarterTurns::One);
        assert_eq!(grid.to_string(), "12|8|4|0\n13|6|10|1\n14|5|9|2\n15|11|7|3");
        grid.rotate(QuarterTurns::One);
        assert_eq!(grid.to_string(), "15|14|13|12\n11|10|9|8\n7|6|5|4\n3|2|1|0");
        grid.rotate(QuarterTurns::One);
        assert_eq!(grid.to_string(), "3|7|11|15\n2|9|5|14\n1|10|6|13\n0|4|8|12");
        grid.rotate(QuarterTurns::One);
        assert_eq!(grid.to_string(), "0|1|2|3\n4|5|6|7\n8|9|10|11\n12|13|14|15");
    }

    #[test]
    fn rotate_anticlockwise_3() {
        let mut grid: TileGrid8<usize, 3, 3, 9> = TileGrid8::from_fn(|x| x.into());

        assert_eq!(grid.to_string(), "0|1|2\n3|4|5\n6|7|8");
        grid.rotate(QuarterTurns::Three);
        assert_eq!(grid.to_string(), "2|5|8\n1|4|7\n0|3|6");
        grid.rotate(QuarterTurns::Three);
        assert_eq!(grid.to_string(), "8|7|6\n5|4|3\n2|1|0");
        grid.rotate(QuarterTurns::Three);
        assert_eq!(grid.to_string(), "6|3|0\n7|4|1\n8|5|2");
        grid.rotate(QuarterTurns::Three);
        assert_eq!(grid.to_string(), "0|1|2\n3|4|5\n6|7|8");
    }

    #[test]
    fn basic_tests() {
        let mut grid: TileGrid8<usize, 3, 3, 9> = TileGrid8::from_fn(|x| x.into());

        for i in 0..9 {
            assert_eq!(grid[Tile8::<3, 3>::try_from_usize(i).unwrap()], i)
        }

        let str = grid.to_string();
        assert_eq!(str, "0|1|2\n3|4|5\n6|7|8");

        let s_flat = grid.iter().join("|");
        assert_eq!(s_flat, "0|1|2|3|4|5|6|7|8");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        let grid: TileGrid8<usize, 2, 2, 4> = TileGrid8::from_fn(|x| x.into());

        assert_tokens(
            &grid,
            &[
                Token::NewtypeStruct { name: "TileGrid8" },
                Token::Tuple { len: 4 },
                Token::U64(0),
                Token::U64(1),
                Token::U64(2),
                Token::U64(3),
                Token::TupleEnd,
            ],
        );
    }
}
