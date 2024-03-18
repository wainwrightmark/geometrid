use core::{array, iter::Map};

use crate::prelude::*;

#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};

/// A general shape made of tiles.
pub trait Shape :// Flippable + Rotatable +
  IntoIterator<Item = DynamicTile>{
    type OutlineIter: Iterator<Item = DynamicVertex>;
    type RectangleIter: Iterator<Item = Rectangle>;

    fn draw_outline(&self)-> Self::OutlineIter;
    fn deconstruct_into_rectangles(&self)-> Self::RectangleIter;
}
