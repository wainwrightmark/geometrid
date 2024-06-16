use core::{
    fmt::{self, Write},
    iter,
    ops::{Index, IndexMut},
};

use crate::prelude::*;

#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};

/// A grid
/// A map from tiles to values.
/// If the values are just booleans, use `TileSet` instead
#[must_use]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(any(test, feature = "serde"), derive(Serialize, Deserialize))]
pub struct TileMap<T, const WIDTH: u8, const HEIGHT: u8, const SIZE: usize>(
    #[cfg_attr(any(test, feature = "serde"), serde(with = "serde_arrays"))]
    #[cfg_attr(any(test, feature = "serde"), serde(bound(serialize = "T: Serialize")))]
    #[cfg_attr(
        any(test, feature = "serde"),
        serde(bound(deserialize = "T: Deserialize<'de>"))
    )]
    [T; SIZE],
);

impl<T: Default + Copy, const WIDTH: u8, const HEIGHT: u8, const SIZE: usize> Default
    for TileMap<T, WIDTH, HEIGHT, SIZE>
{
    fn default() -> Self {
        debug_assert!(SIZE == (WIDTH * HEIGHT) as usize);
        Self([T::default(); SIZE])
    }
}

impl<T, const WIDTH: u8, const HEIGHT: u8, const SIZE: usize> TileMap<T, WIDTH, HEIGHT, SIZE> {
    pub fn from_fn<F: FnMut(Tile<WIDTH, HEIGHT>) -> T>(mut cb: F) -> Self {
        debug_assert!(SIZE == (WIDTH * HEIGHT) as usize);
        let arr = core::array::from_fn(|i| cb(Tile::try_from_usize(i).unwrap()));
        Self(arr)
    }

    #[must_use]
    #[inline]
    pub fn into_inner(self) -> [T; SIZE] {
        self.0
    }

    #[inline]
    pub fn from_inner(inner: [T; SIZE]) -> Self {
        debug_assert!(SIZE == (WIDTH * HEIGHT) as usize);
        Self(inner)
    }

    #[must_use]
    #[inline]
    pub fn enumerate(&self) -> impl iter::Iterator<Item = (Tile<WIDTH, HEIGHT>, &'_ T)> {
        self.0
            .iter()
            .enumerate()
            .map(|(inner, x)| (Tile::try_from_usize(inner).unwrap(), x))
    }

    #[inline]
    pub fn swap(&mut self, p1: Tile<WIDTH, HEIGHT>, p2: Tile<WIDTH, HEIGHT>) {
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
    pub fn row(&self, y: u8) -> &[T] {
        let start = y * WIDTH;
        let end = start + WIDTH;
        &self.0[(start as usize)..(end as usize)]
    }

    #[must_use]
    #[inline]
    pub fn row_mut(&mut self, y: u8) -> &mut [T] {
        let start = y * WIDTH;
        let end = start + WIDTH;
        &mut self.0[(start as usize)..(end as usize)]
    }

    #[must_use]
    pub fn column_iter(&self, column: u8) -> impl DoubleEndedIterator<Item = &T> + '_ {
        (0..HEIGHT)
            .map(move |y| column + (y * WIDTH))
            .map(|x| &self.0[x as usize])
    }

    /// Get the scale to make the grid take up as much as possible of a given area
    #[must_use]
    pub fn get_scale(total_width: f32, total_height: f32) -> f32 {
        let x_multiplier = total_width / f32::from(WIDTH);
        let y_multiplier = total_height / f32::from(HEIGHT);

        x_multiplier.min(y_multiplier)
    }

    pub fn flip(&mut self, axes: FlipAxes) {
        match axes {
            FlipAxes::None => {}
            FlipAxes::Horizontal => {
                for y in 0..HEIGHT {
                    for x in 0..WIDTH / 2 {
                        let p1 = Tile::<WIDTH, HEIGHT>::new_unchecked(x, y);
                        let p2 = p1.flip(axes);
                        self.swap(p1, p2);
                    }
                }
            }
            FlipAxes::Vertical => {
                for y in 0..HEIGHT / 2 {
                    for x in 0..WIDTH {
                        let p1 = Tile::<WIDTH, HEIGHT>::new_unchecked(x, y);
                        let p2 = p1.flip(axes);
                        self.swap(p1, p2);
                    }
                }
            }
            FlipAxes::Both => {
                for y in 0..HEIGHT / 2 {
                    for x in 0..WIDTH {
                        let p1 = Tile::<WIDTH, HEIGHT>::new_unchecked(x, y);
                        let p2 = p1.flip(axes);
                        self.swap(p1, p2);
                    }
                }

                if WIDTH % 2 != 0 {
                    for x in 0..(WIDTH / 2) {
                        let p1 = Tile::<WIDTH, HEIGHT>::new_unchecked(x, HEIGHT / 2);
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
        //todo const once const swap is stabilized
        match quarter_turns {
            QuarterTurns::Zero => {}
            QuarterTurns::One => {
                let mut y = 0;
                'y: loop {
                    if y >= L / 2 {
                        break 'y;
                    }
                    let mut x = y;
                    let o_y = L - (1 + y);
                    'x: loop {
                        if x + y + 1 >= L {
                            break 'x;
                        };

                        let o_x = L - (1 + x);
                        if y != o_y || x != o_x {
                            let p0 = Tile::new_unchecked(x, y);
                            let p1 = Tile::new_unchecked(y, o_x);
                            let p2 = Tile::new_unchecked(o_x, o_y);
                            let p3 = Tile::new_unchecked(o_y, x);

                            //println!("Rotate {p0} {p1} {p2} {p3}");
                            //0123
                            self.swap(p0, p3); //3120
                            self.swap(p0, p1); //1320
                            self.swap(p1, p2); //1230
                        }
                        x += 1;
                    }
                    y += 1;
                }
            }
            QuarterTurns::Two => {
                for y in 0..((L + 1) / 2) {
                    let o_y = L - (1 + y);
                    let x_max = if (y * 2) + 1 == L { L / 2 } else { L };
                    for x in 0..x_max {
                        let o_x = L - (1 + x);
                        let p0 = Tile::new_unchecked(x, y);
                        let p3 = Tile::new_unchecked(o_x, o_y);
                        self.swap(p0, p3);
                    }
                }
            }
            QuarterTurns::Three => {
                let mut y = 0;
                'y: loop {
                    if y >= L / 2 {
                        break 'y;
                    }
                    let mut x = y;
                    let o_y = L - (1 + y);
                    'x: loop {
                        if x + y + 1 >= L {
                            break 'x;
                        };

                        let o_x = L - (1 + x);
                        if y != o_y || x != o_x {
                            let p0 = Tile::new_unchecked(x, y);
                            let p1 = Tile::new_unchecked(y, o_x);
                            let p2 = Tile::new_unchecked(o_x, o_y);
                            let p3 = Tile::new_unchecked(o_y, x);

                            //println!("Rotate {p0} {p1} {p2} {p3}");
                            //0123
                            self.swap(p1, p2); //0213
                            self.swap(p0, p1); //2013
                            self.swap(p0, p3); //3012
                        }
                        x += 1;
                    }
                    y += 1;
                }
            }
        }
    }
}

impl<T: Clone, const L: u8, const SIZE: usize> TileMap<T, L, L, SIZE> {
    #[must_use]
    pub fn with_rotate(&self, quarter_turns: QuarterTurns) -> Self {
        let mut grid = self.clone();
        grid.rotate(quarter_turns);
        grid
    }
}

impl<T: Clone, const WIDTH: u8, const HEIGHT: u8, const SIZE: usize>
    TileMap<T, WIDTH, HEIGHT, SIZE>
{
    pub fn with_flip(&self, axes: FlipAxes) -> Self {
        let mut grid = self.clone();
        grid.flip(axes);
        grid
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
            } else if !f.alternate() && i % (W as usize) == 0 {
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
    use core::usize;

    use super::*;
    use itertools::Itertools;
    #[cfg(any(test, feature = "serde"))]
    use serde_test::{assert_tokens, Token};

    #[test]
    #[should_panic(expected = "assertion failed")]
    #[allow(unused_variables)]
    fn test_bad_grid() {
        let grid: TileMap<usize, 3, 3, 10> = TileMap::default();
    }

    #[test]
    fn test_flip3() {
        for (axes, expected) in [
            (
                FlipAxes::None,
                "0|1|2\n\
                 3|4|5\n\
                 6|7|8",
            ),
            (
                FlipAxes::Vertical,
                "6|7|8\n\
                                  3|4|5\n\
                                  0|1|2",
            ),
            (
                FlipAxes::Horizontal,
                "2|1|0\n\
                                    5|4|3\n\
                                    8|7|6",
            ),
            (
                FlipAxes::Both,
                "8|7|6\n\
                              5|4|3\n\
                              2|1|0",
            ),
        ] {
            let grid: TileMap<usize, 3, 3, 9> = TileMap::from_fn(|x| x.into()).with_flip(axes);

            assert_eq!(grid.to_string(), expected);
        }
    }
    #[test]
    fn test_flip4() {
        for (axes, expected) in [
            (
                FlipAxes::None,
                "0|1|2|3\n\
                  4|5|6|7\n\
                  8|9|10|11\n\
                  12|13|14|15",
            ),
            (
                FlipAxes::Vertical,
                "12|13|14|15\n\
                 8|9|10|11\n\
                 4|5|6|7\n\
                 0|1|2|3",
            ),
            (
                FlipAxes::Horizontal,
                "3|2|1|0\n\
                 7|6|5|4\n\
                 11|10|9|8\n\
                 15|14|13|12",
            ),
            (
                FlipAxes::Both,
                "15|14|13|12\n\
                  11|10|9|8\n\
                  7|6|5|4\n\
                  3|2|1|0",
            ),
        ] {
            let grid: TileMap<usize, 4, 4, 16> = TileMap::from_fn(|x| x.into()).with_flip(axes);
            assert_eq!(grid.to_string(), expected);
        }
    }

    #[test]
    fn test_rotate_length_1() {
        test_rotation::<1, 1>("0");
    }

    #[test]
    fn test_rotate_length_2() {
        test_rotation::<2, 4>(
            "2|0\n\
            3|1",
        );
    }

    #[test]
    fn test_rotate_length_3() {
        test_rotation::<3, 9>(
            "6|3|0\n\
             7|4|1\n\
             8|5|2",
        );
    }

    #[test]
    fn test_rotate_length_4() {
        test_rotation::<4, 16>(
            "12|8|4|0\n\
        13|9|5|1\n\
        14|10|6|2\n\
        15|11|7|3",
        );
    }

    #[test]
    fn test_rotate_length_5() {
        test_rotation::<5, 25>(
            "20|15|10|5|0\n\
        21|16|11|6|1\n\
        22|17|12|7|2\n\
        23|18|13|8|3\n\
        24|19|14|9|4",
        );
    }

    #[test]
    fn test_rotate_length_6() {
        test_rotation::<6, 36>(
            "30|24|18|12|6|0\n\
            31|25|19|13|7|1\n\
            32|26|20|14|8|2\n\
            33|27|21|15|9|3\n\
            34|28|22|16|10|4\n\
            35|29|23|17|11|5",
        );
    }

    fn test_rotation<const LENGTH: u8, const SIZE: usize>(e: &str) {
        let original_grid: TileMap<usize, LENGTH, LENGTH, SIZE> = TileMap::from_fn(|x| x.into());

        let rotated_0 = original_grid.with_rotate(QuarterTurns::Zero);
        assert_eq!(original_grid, rotated_0);

        let rotated_1 = original_grid.with_rotate(QuarterTurns::One);

        assert_eq!(rotated_1.to_string(), e);

        let rotated_2 = original_grid.with_rotate(QuarterTurns::Two);
        let rotated_1x2 = original_grid
            .with_rotate(QuarterTurns::One)
            .with_rotate(QuarterTurns::One);

        assert_eq!(rotated_1x2, rotated_2);

        let rotated_3 = original_grid.with_rotate(QuarterTurns::Three);
        let rotated_1x3 = original_grid
            .with_rotate(QuarterTurns::One)
            .with_rotate(QuarterTurns::One)
            .with_rotate(QuarterTurns::One);
        assert_eq!(rotated_1x3, rotated_3);

        let rotated_1x4 = original_grid
            .with_rotate(QuarterTurns::One)
            .with_rotate(QuarterTurns::One)
            .with_rotate(QuarterTurns::One)
            .with_rotate(QuarterTurns::One);
        assert_eq!(rotated_1x4, original_grid);
    }

    #[test]
    fn basic_tests() {
        let grid: TileMap<usize, 3, 3, 9> = TileMap::from_fn(|x| x.into());

        for i in 0..9 {
            assert_eq!(grid[Tile::<3, 3>::try_from_usize(i).unwrap()], i)
        }

        let str = grid.to_string();

        assert_eq!(
            str,
            "0|1|2\n\
                        3|4|5\n\
                        6|7|8"
        );

        assert_eq!(format!("{grid:#}"), "0|1|2|3|4|5|6|7|8");

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
