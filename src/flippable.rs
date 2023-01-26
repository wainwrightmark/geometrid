use strum::{Display, EnumCount, EnumIter};

use crate::primitive::UniformPrimitive;

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
    fn flipped(&self, axes: FlipAxes) -> Self {
        let mut s = self.clone();
        s.flip(axes);
        s
    }
}

impl<T: Flippable + Copy> CopyFlippable for T {}
