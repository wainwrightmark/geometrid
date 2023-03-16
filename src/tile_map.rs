use core::{
    fmt::{self, Write},
    iter,
    ops::{Index, IndexMut},
};

use crate::prelude::*;

#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};

#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(any(test, feature = "serde"), derive(Serialize, Deserialize))]
pub struct TileMap<T, const COLS: u8, const ROWS: u8, const SIZE: usize>(
    #[cfg_attr(any(test, feature = "serde"), serde(with = "serde_arrays"))]
    #[cfg_attr(any(test, feature = "serde"), serde(bound(serialize = "T: Serialize")))]
    #[cfg_attr(
        any(test, feature = "serde"),
        serde(bound(deserialize = "T: Deserialize<'de>"))
    )]
    [T; SIZE],
);

impl<T: Default + Copy, const COLS: u8, const ROWS: u8, const SIZE: usize> Default
    for TileMap<T, COLS, ROWS, SIZE>
{
    fn default() -> Self {
        debug_assert!(SIZE == (COLS * ROWS) as usize);
        Self([T::default(); SIZE])
    }
}

impl<T, const COLS: u8, const ROWS: u8, const SIZE: usize> TileMap<T, COLS, ROWS, SIZE> {
    #[must_use]
    pub fn from_fn<F: FnMut(Tile<COLS, ROWS>) -> T>(mut cb: F) -> Self {
        debug_assert!(SIZE == (COLS * ROWS) as usize);
        let arr = core::array::from_fn(|i| cb(Tile::try_from_usize(i).unwrap()));
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
        debug_assert!(SIZE == (COLS * ROWS) as usize);
        Self(inner)
    }

    #[must_use]
    #[inline]
    pub fn enumerate(&self) -> impl iter::Iterator<Item = (Tile<COLS, ROWS>, &'_ T)> {
        self.0
            .iter()
            .enumerate()
            .map(|(inner, x)| (Tile::try_from_usize(inner).unwrap(), x))
    }

    #[inline]
    pub fn swap(&mut self, p1: Tile<COLS, ROWS>, p2: Tile<COLS, ROWS>) {
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
    pub fn row(&self, row: u8) -> &[T] {
        let start = row * COLS;
        let end = start + COLS;
        &self.0[(start as usize)..(end as usize)]
    }

    #[must_use]
    #[inline]
    pub fn row_mut(&mut self, row: u8) -> &mut [T] {
        let start = row * COLS;
        let end = start + COLS;
        &mut self.0[(start as usize)..(end as usize)]
    }

    #[must_use]
    pub fn column_iter(&self, column: u8) -> impl DoubleEndedIterator<Item = &T> + '_ {
        (0..ROWS)
            .map(move |row| column + (row * COLS))
            .map(|x| &self.0[x as usize])
    }

    /// Get the scale to make the grid take up as much as possible of a given area
    #[must_use]
    pub fn get_scale(total_width: f32, total_height: f32) -> f32 {
        let x_multiplier = total_width / COLS as f32;
        let y_multiplier = total_height / ROWS as f32;

        x_multiplier.min(y_multiplier)
    }

    pub fn flip(&mut self, axes: FlipAxes) {
        match axes {
            FlipAxes::None => {}
            FlipAxes::Horizontal => {
                for y in 0..ROWS {
                    for x in 0..COLS / 2 {
                        let p1 = Tile::<COLS, ROWS>::new_unchecked(x, y);
                        let p2 = p1.flip(axes);
                        self.swap(p1, p2);
                    }
                }
            }
            FlipAxes::Vertical => {
                for y in 0..ROWS / 2 {
                    for x in 0..COLS {
                        let p1 = Tile::<COLS, ROWS>::try_new(x, y).unwrap();
                        let p2 = p1.flip(axes);
                        self.swap(p1, p2);
                    }
                }
            }
            FlipAxes::Both => {
                for y in 0..ROWS / 2 {
                    for x in 0..COLS {
                        let p1 = Tile::<COLS, ROWS>::try_new(x, y).unwrap();
                        let p2 = p1.flip(axes);
                        self.swap(p1, p2);
                    }
                }

                if COLS % 2 != 0 {
                    for x in 0..(COLS / 2) {
                        let p1 = Tile::<COLS, ROWS>::try_new(x, ROWS / 2).unwrap();
                        let p2 = p1.flip(axes);
                        self.swap(p1, p2);
                    }
                }
            }
        }
    }
}

impl<T, const L: u8, const SIZE: usize> TileMap<T, L, L, SIZE> {
    pub fn rotate(&mut self, quarter_turns: QuarterTurns) {
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
                            let p0 = Tile::new_unchecked(column, row);
                            let p1 = Tile::new_unchecked(row, o_column);
                            let p2 = Tile::new_unchecked(o_column, o_row);
                            let p3 = Tile::new_unchecked(o_row, column);

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
                            let p0 = Tile::new_unchecked(column, row);
                            let p1 = Tile::new_unchecked(row, o_column);
                            let p2 = Tile::new_unchecked(o_row, column);
                            let p3 = Tile::new_unchecked(o_column, o_row);

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
                            let p0 = Tile::new_unchecked(column, row);
                            let p1 = Tile::new_unchecked(row, o_column);
                            let p2 = Tile::new_unchecked(o_column, o_row);
                            let p3 = Tile::new_unchecked(o_row, column);

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

impl<T, const W: u8, const H: u8, const SIZE: usize> Index<Tile<W, H>> for TileMap<T, W, H, SIZE> {
    type Output = T;

    fn index(&self, index: Tile<W, H>) -> &Self::Output {
        let u: usize = index.into();
        &self.0[u]
    }
}

impl<T, const W: u8, const H: u8, const SIZE: usize> IndexMut<Tile<W, H>>
    for TileMap<T, W, H, SIZE>
{
    fn index_mut(&mut self, index: Tile<W, H>) -> &mut Self::Output {
        let u: usize = index.into();
        &mut self.0[u]
    }
}

impl<T, const W: u8, const H: u8, const SIZE: usize> AsRef<[T; SIZE]> for TileMap<T, W, H, SIZE> {
    #[inline]
    fn as_ref(&self) -> &[T; SIZE] {
        &self.0
    }
}

impl<T, const W: u8, const H: u8, const SIZE: usize> AsMut<[T; SIZE]> for TileMap<T, W, H, SIZE> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T; SIZE] {
        &mut self.0
    }
}

impl<'a, T, const W: u8, const H: u8, const SIZE: usize> IntoIterator
    for &'a TileMap<T, W, H, SIZE>
{
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T, const W: u8, const H: u8, const SIZE: usize> IntoIterator
    for &'a mut TileMap<T, W, H, SIZE>
{
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<T, const W: u8, const H: u8, const SIZE: usize> IntoIterator for TileMap<T, W, H, SIZE> {
    type Item = T;
    type IntoIter = core::array::IntoIter<T, SIZE>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.0)
    }
}

impl<T: fmt::Display, const W: u8, const H: u8, const SIZE: usize> fmt::Display
    for TileMap<T, W, H, SIZE>
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use itertools::Itertools;
    #[cfg(any(test, feature = "serde"))]
    use serde_test::{assert_tokens, Token};

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_bad_grid() {
        let mut grid: TileMap<usize, 3, 3, 10> = TileMap::default();
    }

    #[test]
    fn test_flip3() {
        for (axes, expected) in [
            (FlipAxes::None, "0|1|2\n3|4|5\n6|7|8"),
            (FlipAxes::Vertical, "6|7|8\n3|4|5\n0|1|2"),
            (FlipAxes::Horizontal, "2|1|0\n5|4|3\n8|7|6"),
            (FlipAxes::Both, "8|7|6\n5|4|3\n2|1|0"),
        ] {
            let mut grid: TileMap<usize, 3, 3, 9> = TileMap::from_fn(|x| x.into());
            grid.flip(axes);
            assert_eq!(grid.to_string(), expected);
        }
    }
    #[test]
    fn test_flip4() {
        for (axes, expected) in [
            (FlipAxes::None, "0|1|2|3\n4|5|6|7\n8|9|10|11\n12|13|14|15"),
            (
                FlipAxes::Vertical,
                "12|13|14|15\n8|9|10|11\n4|5|6|7\n0|1|2|3",
            ),
            (
                FlipAxes::Horizontal,
                "3|2|1|0\n7|6|5|4\n11|10|9|8\n15|14|13|12",
            ),
            (FlipAxes::Both, "15|14|13|12\n11|10|9|8\n7|6|5|4\n3|2|1|0"),
        ] {
            let mut grid: TileMap<usize, 4, 4, 16> = TileMap::from_fn(|x| x.into());
            grid.flip(axes);
            assert_eq!(grid.to_string(), expected);
        }
    }

    #[test]
    fn rotate_clockwise_3() {
        let mut grid: TileMap<usize, 3, 3, 9> = TileMap::from_fn(|x| x.into());

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
        let mut grid: TileMap<usize, 4, 4, 16> = TileMap::from_fn(|x| x.into());

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
        let mut grid: TileMap<usize, 3, 3, 9> = TileMap::from_fn(|x| x.into());

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
    fn rotate_half_3() {
        let mut grid: TileMap<usize, 3, 3, 9> = TileMap::from_fn(|x| x.into());

        assert_eq!(grid.to_string(), "0|1|2\n3|4|5\n6|7|8");
        grid.rotate(QuarterTurns::Two);
        assert_eq!(grid.to_string(), "8|7|6\n5|4|3\n2|1|0");
        grid.rotate(QuarterTurns::Two);
        assert_eq!(grid.to_string(), "0|1|2\n3|4|5\n6|7|8");
    }

    #[test]
    fn rotate_half_4() {
        let mut grid: TileMap<usize, 4, 4, 16> = TileMap::from_fn(|x| x.into());

        assert_eq!(grid.to_string(), "0|1|2|3\n4|5|6|7\n8|9|10|11\n12|13|14|15");
        grid.rotate(QuarterTurns::Two);
        assert_eq!(grid.to_string(), "15|14|13|12\n11|10|9|8\n7|6|5|4\n3|2|1|0");
        grid.rotate(QuarterTurns::Two);
        assert_eq!(grid.to_string(), "0|1|2|3\n4|5|6|7\n8|9|10|11\n12|13|14|15");
    }

    #[test]
    fn basic_tests() {
        let mut grid: TileMap<usize, 3, 3, 9> = TileMap::from_fn(|x| x.into());

        for i in 0..9 {
            assert_eq!(grid[Tile::<3, 3>::try_from_usize(i).unwrap()], i)
        }

        let str = grid.to_string();
        assert_eq!(str, "0|1|2\n3|4|5\n6|7|8");

        let s_flat = grid.iter().join("|");
        assert_eq!(s_flat, "0|1|2|3|4|5|6|7|8");
    }

    #[cfg(any(test, feature = "serde"))]
    #[test]
    fn test_serde() {
        let grid: TileMap<usize, 2, 2, 4> = TileMap::from_fn(|x| x.into());

        assert_tokens(
            &grid,
            &[
                Token::NewtypeStruct { name: "TileMap" },
                Token::Tuple { len: 4 },
                Token::U64(0),
                Token::U64(1),
                Token::U64(2),
                Token::U64(3),
                Token::TupleEnd,
            ],
        );
    }

    #[test]
    fn test_get_scale() {
        assert_eq!(TileMap::<usize, 3, 2, 4>::get_scale(12.0, 20.0), 4.0);
    }
}
