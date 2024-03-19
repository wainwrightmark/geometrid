use crate::prelude::*;

/// A general shape made of tiles.
pub trait Shape :// Flippable + Rotatable +
  IntoIterator<Item = DynamicTile>{
    type OutlineIter: Iterator<Item = DynamicVertex>;
    type RectangleIter: Iterator<Item = Rectangle>;

    fn draw_outline(&self)-> Self::OutlineIter;
    fn deconstruct_into_rectangles(&self)-> Self::RectangleIter;
}
