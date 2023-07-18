use core::{array, iter::Map};

use crate::prelude::*;

#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};

/// A rectangle in a 2d space
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(any(test, feature = "serde"), derive(Serialize, Deserialize))]
pub struct Rectangle {
    /// The north vest vertex of the rectangle
    pub north_west: DynamicVertex,
    /// The number of tiles wide
    pub width: u8,
    /// The number of tiles tall
    pub height: u8,
}

impl Rectangle {
    pub fn new(north_west: DynamicVertex, width: u8, height: u8) -> Self {
        Self {
            north_west,
            width,
            height,
        }
    }

    /// The total number of tiles of the rectangle
    pub fn area(&self) -> usize {
        self.width as usize * self.height as usize
    }
}

#[cfg(any(test, feature = "glam"))]
impl HasCenter for Rectangle {
    fn get_center(&self, scale: f32) -> glam::f32::Vec2 {
        let mut center = self.north_west.get_center(scale);
        center.x += (self.width as f32 * 0.5 * scale);
        center.y += (self.height as f32 * 0.5 * scale);
        center
    }
}

impl Shape for Rectangle {
    type OutlineIter = array::IntoIter<DynamicVertex, 4>;

    type RectangleIter = array::IntoIter<Rectangle, 1>;

    fn draw_outline(&self) -> Self::OutlineIter {
        [
            self.north_west,
            Vector {
                x: self.north_west.x.saturating_add_unsigned(self.width),
                y: self.north_west.y,
            }
            .into(),
            Vector {
                x: self.north_west.x,
                y: self.north_west.y.saturating_add_unsigned(self.height),
            }
            .into(),
            Vector {
                x: self.north_west.x.saturating_add_unsigned(self.width),
                y: self.north_west.y.saturating_add_unsigned(self.height),
            }
            .into(),
        ]
        .into_iter()
    }

    fn deconstruct_into_rectangles(&self) -> Self::RectangleIter {
        [self.clone()].into_iter()
    }
}

impl IntoIterator for Rectangle {
    type Item = DynamicTile;
    type IntoIter = RectangleIterator;

    fn into_iter(self) -> Self::IntoIter {
        RectangleIterator {
            rectangle: self,
            next: Some(self.north_west.get_tile(&Corner::SouthEast)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RectangleIterator {
    pub rectangle: Rectangle,
    pub next: Option<DynamicTile>,
}

impl Iterator for RectangleIterator {
    type Item = DynamicTile;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            Some(next) => {
                if next.x
                    >= self
                        .rectangle
                        .north_west
                        .x
                        .saturating_add_unsigned(self.rectangle.width)
                {
                    if next.y
                        >= self
                            .rectangle
                            .north_west
                            .y
                            .saturating_add_unsigned(self.rectangle.height)
                    {
                        self.next = None;
                    } else {
                        self.next = Some(
                            Vector {
                                x: self.rectangle.north_west.x,
                                y: (next + Vector::SOUTH).y,
                            }
                            .into(),
                        )
                    }
                } else {
                    self.next = Some(next + Vector::EAST);
                }
                Some(next)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::*;
    use itertools::Itertools;
    use serde_test::{assert_tokens, Token};

    #[test]
    pub fn test_center() {
        let rect = Rectangle::new(Vector::NORTH_EAST.into(), 2, 4);

        let center = rect.get_center(3.0);
        assert_eq!(center, glam::f32::Vec2::new(6.0, 3.0))
    }

    #[test]
    pub fn test_outline() {
        let rect = Rectangle::new(Vector::NORTH_EAST.into(), 2, 4);

        let outline = rect.draw_outline().collect_vec();

        assert_eq!(
            outline.into_iter().join("; "),
            "(1,-1); (3,-1); (1,3); (3,3)"
        );
    }

    #[test]
    pub fn test_deconstruct() {
        let rect = Rectangle::new(Vector::NORTH_EAST.into(), 2, 4);
        let deconstructed = rect.deconstruct_into_rectangles().collect_vec();

        assert_eq!(deconstructed, [rect])
    }
    #[test]
    pub fn test_iter() {
        let rect = Rectangle::new(Vector::NORTH_EAST.into(), 2, 4);

        let tiles = rect.into_iter().collect_vec();

        assert_eq!(tiles.iter().join(";"), "(1,-1);(2,-1);(3,-1);(1,0);(2,0);(3,0);(1,1);(2,1);(3,1);(1,2);(2,2);(3,2);(1,3);(2,3);(3,3)")
    }
}
