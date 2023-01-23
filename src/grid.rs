use core::{
    fmt::{self, Write},
    iter,
    ops::{Index, IndexMut},
};
use std::mem::MaybeUninit;

use crate::point_absolute::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

macro_rules! grid {
    ($name:ident, $point_ty:ident, $inner:ty) => {
        #[must_use]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct $name<T, const WIDTH: $inner, const HEIGHT: $inner, const SIZE: usize>(
            #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
            #[cfg_attr(feature = "serde", serde(bound(serialize = "T: Serialize")))]
            #[cfg_attr(feature = "serde", serde(bound(deserialize = "T: Deserialize<'de>")))]
            pub [T; SIZE],
        );

        impl<T: Default + Copy, const W: $inner, const H: $inner, const SIZE: usize> Default
            for $name<T, W, H, SIZE>
        {
            fn default() -> Self {
                Self([T::default(); SIZE])
            }
        }

        impl<T, const W: $inner, const H: $inner, const SIZE: usize> $name<T, W, H, SIZE> {


            pub fn from_fn<F : FnMut($point_ty::<W, H>) -> T>(mut cb: F)->Self{
                let arr = core::array::from_fn(|i| cb($point_ty::try_from_usize(i).unwrap()) );
                Self(arr)
            }

            #[must_use]
            #[inline]
            pub fn into_inner(self) -> [T; SIZE] {
                self.0
            }

            #[must_use]
            #[inline]
            pub fn enumerate(&self) -> impl iter::Iterator<Item = ($point_ty<W, H>, &'_ T)> {
                self.0
                    .iter()
                    .enumerate()
                    .map(|(inner, x)| ($point_ty::try_from_usize(inner).unwrap(), x))
            }

            /// Flip along the y-axis.
            /// Th leftmost column becomes the rightmost column
            pub fn flip_horizontal(&mut self) {
                for y in 0..H {
                    for x in 0..W / 2 {
                        let p1 = $point_ty::<W, H>::new_unchecked(x,y);
                        let p2 = p1.flip_horizontal();
                        self.swap(p1, p2);
                    }
                }
            }

            /// Flip along the x axis.
            /// The top row becomes the bottom row
            pub fn flip_vertical(&mut self) {
                for y in 0..H / 2 {
                    for x in 0..W {
                        let p1 = $point_ty::<W, H>::try_new(x, y).unwrap();
                        let p2 = p1.flip_vertical();
                        self.swap(p1, p2);
                    }
                }
            }

            pub fn swap(&mut self, p1: $point_ty<W, H>, p2: $point_ty<W, H>) {
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
        }

        impl<T, const L: $inner, const SIZE: usize> $name<T, L, L, SIZE> {
            pub fn rotate_clockwise(&mut self) {
                for row in 0..=(L / 2) {
                    for column in row..=(L / 2) {
                        let o_row = L - (1 + row);
                        let o_column = L - (1 + column);
                        if row != o_row || column != o_column {
                            let p0 = $point_ty::new_unchecked(column, row);
                            let p1 = $point_ty::new_unchecked(row, o_column);
                            let p2 = $point_ty::new_unchecked(o_column, o_row);
                            let p3 = $point_ty::new_unchecked(o_row, column);

                            //0123
                            self.swap(p0, p3); //3120
                            self.swap(p0, p1); //1320
                            self.swap(p1, p2); //1230
                        }
                    }
                }
            }

            pub fn rotate_anticlockwise(&mut self) {
                for row in 0..=(L / 2) {
                    for column in row..=(L / 2) {
                        let o_row = L - (1 + row);
                        let o_column = L - (1 + column);
                        if row != o_row || column != o_column {
                            let p0 = $point_ty::new_unchecked(column, row);
                            let p1 = $point_ty::new_unchecked(row, o_column);
                            let p2 = $point_ty::new_unchecked(o_column, o_row);
                            let p3 = $point_ty::new_unchecked(o_row, column);

                            //0123
                            self.swap(p1, p2); //0213
                            self.swap(p0, p1); //2013
                            self.swap(p0, p3); //3012
                        }
                    }
                }
            }
        }

        impl<T, const W: $inner, const H: $inner, const SIZE: usize> Index<$point_ty<W, H>>
            for $name<T, W, H, SIZE>
        {
            type Output = T;

            fn index(&self, index: $point_ty<W, H>) -> &Self::Output {
                let u: usize = index.into();
                &self.0[u]
            }
        }

        impl<T, const W: $inner, const H: $inner, const SIZE: usize> IndexMut<$point_ty<W, H>>
            for $name<T, W, H, SIZE>
        {
            fn index_mut(&mut self, index: $point_ty<W, H>) -> &mut Self::Output {
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

grid!(Grid64, PointAbsolute64, u64);
grid!(Grid32, PointAbsolute32, u32);
grid!(Grid16, PointAbsolute16, u16);
grid!(Grid8, PointAbsolute8, u8);


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{point_absolute::*, rectangle::*};
    use itertools::Itertools;

    #[test]
    fn test_flip_vertical(){
        let mut grid: Grid8<usize, 3, 3, 9> = Grid8::from_fn(|x| x.into());

        assert_eq!(grid.to_string(), "0|1|2\n3|4|5\n6|7|8");
        grid.flip_vertical();
        assert_eq!(grid.to_string(), "6|7|8\n3|4|5\n0|1|2");
    }

    #[test]
    fn test_flip_horizontal(){
        let mut grid: Grid8<usize, 3, 3, 9> = Grid8::from_fn(|x| x.into());

        assert_eq!(grid.to_string(), "0|1|2\n3|4|5\n6|7|8");
        grid.flip_horizontal();
        assert_eq!(grid.to_string(), "2|1|0\n5|4|3\n8|7|6");
    }

    #[test]
    fn rotate_clockwise_3() {
        let mut grid: Grid8<usize, 3, 3, 9> = Grid8::from_fn(|x| x.into());

        assert_eq!(grid.to_string(), "0|1|2\n3|4|5\n6|7|8");
        grid.rotate_clockwise();
        assert_eq!(grid.to_string(), "6|3|0\n7|4|1\n8|5|2");
        grid.rotate_clockwise();
        assert_eq!(grid.to_string(), "8|7|6\n5|4|3\n2|1|0");
        grid.rotate_clockwise();
        assert_eq!(grid.to_string(), "2|5|8\n1|4|7\n0|3|6");
        grid.rotate_clockwise();
        assert_eq!(grid.to_string(), "0|1|2\n3|4|5\n6|7|8");
    }

    #[test]
    fn rotate_clockwise_4() {
        let mut grid: Grid8<usize, 4, 4, 16> = Grid8::from_fn(|x| x.into());

        assert_eq!(grid.to_string(), "0|1|2|3\n4|5|6|7\n8|9|10|11\n12|13|14|15");
        grid.rotate_clockwise();
        assert_eq!(grid.to_string(), "12|8|4|0\n13|6|10|1\n14|5|9|2\n15|11|7|3");
        grid.rotate_clockwise();
        assert_eq!(grid.to_string(), "15|14|13|12\n11|10|9|8\n7|6|5|4\n3|2|1|0");
        grid.rotate_clockwise();
        assert_eq!(grid.to_string(), "3|7|11|15\n2|9|5|14\n1|10|6|13\n0|4|8|12");
        grid.rotate_clockwise();
        assert_eq!(grid.to_string(), "0|1|2|3\n4|5|6|7\n8|9|10|11\n12|13|14|15");
    }

    #[test]
    fn rotate_anticlockwise_3() {
        let mut grid: Grid8<usize, 3, 3, 9> = Grid8::from_fn(|x| x.into());

        assert_eq!(grid.to_string(), "0|1|2\n3|4|5\n6|7|8");
        grid.rotate_anticlockwise();
        assert_eq!(grid.to_string(), "2|5|8\n1|4|7\n0|3|6");
        grid.rotate_anticlockwise();
        assert_eq!(grid.to_string(), "8|7|6\n5|4|3\n2|1|0");
        grid.rotate_anticlockwise();
        assert_eq!(grid.to_string(), "6|3|0\n7|4|1\n8|5|2");
        grid.rotate_anticlockwise();
        assert_eq!(grid.to_string(), "0|1|2\n3|4|5\n6|7|8");
    }

    #[test]
    fn basic_tests() {
        let mut grid: Grid8<usize, 3, 3, 9> = Grid8::from_fn(|x|x.into());

        for i in 0..9 {
            assert_eq!(grid[PointAbsolute8::<3, 3>::try_from_usize(i).unwrap()], i)
        }

        let str = grid.to_string();
        assert_eq!(str, "0|1|2\n3|4|5\n6|7|8");

        let s_flat = grid.iter().join("|");
        assert_eq!(s_flat, "0|1|2|3|4|5|6|7|8");
    }
}
