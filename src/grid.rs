use core::{
    fmt::{self, Write}, iter,
    ops::{Index, IndexMut},
};

use super::absolute_coordinate::AbsoluteCoordinate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QGrid<T, const WIDTH: u16, const HEIGHT: u16, const SIZE: usize>([T; SIZE]);

impl<T: Default + Copy, const W: u16, const H: u16, const SIZE: usize> Default
    for QGrid<T, W, H, SIZE>
{
    fn default() -> Self {
        Self([T::default(); SIZE])
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> Index<AbsoluteCoordinate<W, H>>
    for QGrid<T, W, H, SIZE>
{
    type Output = T;

    fn index(&self, index: AbsoluteCoordinate<W, H>) -> &Self::Output {
        let u: usize = index.into();
        &self.0[u]
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> IndexMut<AbsoluteCoordinate<W, H>>
    for QGrid<T, W, H, SIZE>
{
    fn index_mut(&mut self, index: AbsoluteCoordinate<W, H>) -> &mut Self::Output {
        let u: usize = index.into();
        &mut self.0[u]
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> QGrid<T, W, H, SIZE> {
    const _ASSERTION1: usize = SIZE - ((W * H) as usize);
    const _ASSERTION2: usize = ((W * H) as usize) - SIZE;

    #[inline]
    pub fn into_inner(self) -> [T; SIZE] {
        self.0
    }

    #[inline]
    pub fn enumerate(&self) -> impl iter::Iterator<Item = (AbsoluteCoordinate<W, H>, &'_ T)> {
        self.0
            .iter()
            .enumerate()
            .map(|(inner, x)| (AbsoluteCoordinate::try_from_usize(inner).unwrap(), x))
    }

    pub fn flip_horizontal(&mut self) {
        for y in 0..H {
            for x in 0..W / 2 {
                let qa1 = AbsoluteCoordinate::<W, H>::try_new(x, y).unwrap();
                let qa2 = qa1.flip_horizontal();
                self.0.swap(qa1.into(), qa2.into());
            }
        }
    }

    /// Flip all elements vertically.
    pub fn flip_vertical(&mut self) {
        for y in 0..H / 2 {
            for x in 0..W {
                let qa1 = AbsoluteCoordinate::<W, H>::try_new(x, y).unwrap();
                let qa2 = qa1.flip_vertical();
                self.0.swap(qa1.into(), qa2.into());
            }
        }
    }

    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.0.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.0.iter_mut()
    }

    #[inline]
    pub fn row(&self, row: u16) -> &[T] {
        let start = row * W;
        let end = start + W;
        &self.0[(start as usize)..(end as usize)]
    }

    #[inline]
    pub fn row_mut(&mut self, row: u16) -> &mut [T] {
        let start = row * W;
        let end = start + W;
        &mut self.0[(start as usize)..(end as usize)]
    }

    pub fn column_iter(&self, column: u16) -> impl DoubleEndedIterator<Item = &T> + '_ {
        (0..H)
            .map(move |row| column + (row * W))
            .map(|x| &self.0[x as usize])
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> AsRef<[T; SIZE]> for QGrid<T, W, H, SIZE> {
    #[inline]
    fn as_ref(&self) -> &[T; SIZE] {
        &self.0
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> AsMut<[T; SIZE]> for QGrid<T, W, H, SIZE> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T; SIZE] {
        &mut self.0
    }
}

impl<'a, T, const W: u16, const H: u16, const SIZE: usize> IntoIterator
    for &'a QGrid<T, W, H, SIZE>
{
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T, const W: u16, const H: u16, const SIZE: usize> IntoIterator
    for &'a mut QGrid<T, W, H, SIZE>
{
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> IntoIterator for QGrid<T, W, H, SIZE> {
    type Item = T;
    type IntoIter = core::array::IntoIter<T, SIZE>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.0)
    }
}


impl<T: fmt::Display, const W: u16, const H: u16, const SIZE: usize> fmt::Display
    for QGrid<T, W, H, SIZE>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.0.iter().enumerate();

        for (i,e) in iter{
            if i == 0{

            }else if i % (W as usize) == 0{
                f.write_char('\n');
            }else{
                f.write_char('|');
            }

            e.fmt(f)?;
        }

        Ok(())
    }
}
