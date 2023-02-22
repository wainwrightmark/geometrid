use strum::{Display, EnumCount, EnumIter};


pub trait Flippable {
    fn flip(&mut self, axes: FlipAxes);
}

#[derive(
    Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumCount, EnumIter,
)]
pub enum FlipAxes {
    #[default]
    None,
    Horizontal,
    Vertical,
    Both,
}

pub trait CopyFlippable: Flippable + Copy {
    #[must_use]
    fn flipped(&self, axes: FlipAxes) -> Self {
        let mut s = self.clone();
        s.flip(axes);
        s
    }
}

impl<T: Flippable + Copy> CopyFlippable for T {}
