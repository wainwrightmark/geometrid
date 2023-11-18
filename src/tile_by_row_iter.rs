#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TileByRowIter<const WIDTH: u8, const HEIGHT: u8>{
    inner: u8
}

impl<const WIDTH: u8, const HEIGHT: u8> ExactSizeIterator for TileByRowIter<WIDTH, HEIGHT> {}

impl<const WIDTH: u8, const HEIGHT: u8> Iterator for TileByRowIter<WIDTH, HEIGHT> {
    type Item = crate::tile::Tile<WIDTH, HEIGHT>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = crate::tile::Tile::<WIDTH, HEIGHT>::try_from_inner(self.inner);
        self.inner = self.inner.saturating_add(1);
        ret

    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let c =  crate::tile::Tile::<WIDTH, HEIGHT>::COUNT.saturating_sub(self.inner as usize);
        (c, Some(c))
    }
}