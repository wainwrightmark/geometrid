use core::{
    fmt::{self, Write}, iter,
    ops::{Index, IndexMut},
};

use super::point_absolute::PointAbsolute;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Grid<T, const WIDTH: u16, const HEIGHT: u16, const SIZE: usize>([T; SIZE]);

impl<T: Default + Copy, const W: u16, const H: u16, const SIZE: usize> Default
    for Grid<T, W, H, SIZE>
{
    fn default() -> Self {
        Self([T::default(); SIZE])
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> Index<PointAbsolute<W, H>>
    for Grid<T, W, H, SIZE>
{
    type Output = T;

    fn index(&self, index: PointAbsolute<W, H>) -> &Self::Output {
        let u: usize = index.into();
        &self.0[u]
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> IndexMut<PointAbsolute<W, H>>
    for Grid<T, W, H, SIZE>
{
    fn index_mut(&mut self, index: PointAbsolute<W, H>) -> &mut Self::Output {
        let u: usize = index.into();
        &mut self.0[u]
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> Grid<T, W, H, SIZE> {
    const _ASSERTION1: usize = SIZE - ((W * H) as usize);
    const _ASSERTION2: usize = ((W * H) as usize) - SIZE;

    #[inline]
    pub fn into_inner(self) -> [T; SIZE] {
        self.0
    }

    #[inline]
    pub fn enumerate(&self) -> impl iter::Iterator<Item = (PointAbsolute<W, H>, &'_ T)> {
        self.0
            .iter()
            .enumerate()
            .map(|(inner, x)| (PointAbsolute::try_from_usize(inner).unwrap(), x))
    }

    pub fn flip_horizontal(&mut self) {
        for y in 0..H {
            for x in 0..W / 2 {
                let qa1 = PointAbsolute::<W, H>::try_new(x, y).unwrap();
                let qa2 = qa1.flip_horizontal();
                self.0.swap(qa1.into(), qa2.into());
            }
        }
    }

    /// Flip all elements vertically.
    pub fn flip_vertical(&mut self) {
        for y in 0..H / 2 {
            for x in 0..W {
                let qa1 = PointAbsolute::<W, H>::try_new(x, y).unwrap();
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

impl<T, const W: u16, const H: u16, const SIZE: usize> AsRef<[T; SIZE]> for Grid<T, W, H, SIZE> {
    #[inline]
    fn as_ref(&self) -> &[T; SIZE] {
        &self.0
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> AsMut<[T; SIZE]> for Grid<T, W, H, SIZE> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T; SIZE] {
        &mut self.0
    }
}

impl<'a, T, const W: u16, const H: u16, const SIZE: usize> IntoIterator
    for &'a Grid<T, W, H, SIZE>
{
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T, const W: u16, const H: u16, const SIZE: usize> IntoIterator
    for &'a mut Grid<T, W, H, SIZE>
{
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<T, const W: u16, const H: u16, const SIZE: usize> IntoIterator for Grid<T, W, H, SIZE> {
    type Item = T;
    type IntoIter = core::array::IntoIter<T, SIZE>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.0)
    }
}


impl<T: fmt::Display, const W: u16, const H: u16, const SIZE: usize> fmt::Display
    for Grid<T, W, H, SIZE>
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


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::rectangle::*;
    use itertools::Itertools;

    #[test]
    fn basic_tests() {
        let mut grid: Grid<usize, 3, 3, 9> = Grid::default();

        for  (i, mut m) in grid.iter_mut().enumerate(){
            *m = i;
        }

        for i in 0..9{
            assert_eq!(grid[PointAbsolute::try_from_usize(i).unwrap()], i)
        }

        let str = grid.to_string();
        assert_eq!(str, "0|1|2\n3|4|5\n6|7|8");
    }

}