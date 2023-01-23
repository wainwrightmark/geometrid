use core::{
    fmt::{self, Write},
    iter,
    ops::{Index, IndexMut},
};

use crate::point_absolute::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

macro_rules! grid {
    ($name:ident, $point_ty:ident, $inner:ty) => {
        #[must_use]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct $name<T, const WIDTH: $inner, const HEIGHT: $inner, const SIZE: usize>(
            #[cfg_attr(feature = "serde",serde(with = "serde_arrays"))]
            #[cfg_attr(feature = "serde",serde(bound(serialize = "T: Serialize")))]
            #[cfg_attr(feature = "serde",serde(bound(deserialize = "T: Deserialize<'de>")))]
            pub [T; SIZE],
        );

        impl<T: Default + Copy, const W: $inner, const H: $inner, const SIZE: usize> Default
            for $name<T, W, H, SIZE>
        {
            fn default() -> Self {
                Self([T::default(); SIZE])
            }
        }

        impl<T, const W: $inner, const H: $inner, const SIZE: usize> $name<T, W, H, SIZE>
        {

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

            pub fn flip_horizontal(&mut self) {
                for y in 0..H {
                    for x in 0..W / 2 {
                        let qa1 = $point_ty::<W, H>::try_new(x, y).unwrap();
                        let qa2 = qa1.flip_horizontal();
                        self.0.swap(qa1.into(), qa2.into());
                    }
                }
            }

            /// Flip all elements vertically.
            pub fn flip_vertical(&mut self) {
                for y in 0..H / 2 {
                    for x in 0..W {
                        let qa1 = $point_ty::<W, H>::try_new(x, y).unwrap();
                        let qa2 = qa1.flip_vertical();
                        self.0.swap(qa1.into(), qa2.into());
                    }
                }
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
    fn size_test(){
        let grid: Grid8<usize,3,3,10> = Grid8::default();
    }

    #[test]
    fn basic_tests() {
        let mut grid: Grid8<usize, 3, 3, 9> = Grid8::default();

        for (i, mut m) in grid.iter_mut().enumerate() {
            *m = i;
        }

        for i in 0..9 {
            assert_eq!(grid[PointAbsolute8::<3, 3>::try_from_usize(i).unwrap()], i)
        }

        let str = grid.to_string();
        assert_eq!(str, "0|1|2\n3|4|5\n6|7|8");

        let s_flat = grid.iter().join("|");
        assert_eq!(s_flat, "0|1|2|3|4|5|6|7|8");
    }
}
