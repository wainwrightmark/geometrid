use itertools::Itertools;
use tinyvec::ArrayVec;

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

use crate::{point_relative::PointRelative8, polyomino::Polyomino};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rectangle {
    pub x: i8,
    pub y: i8,
    pub width: u8,
    pub height: u8,
}

/// Iterator for deconstructing polyominos to rectangles
pub struct RectangleIter<const P: usize> {
    shape: Polyomino<P>,
    remaining_points: ArrayVec<[PointRelative8; P]>,
}

impl<const P: usize> From<Polyomino<P>> for RectangleIter<P> {
    fn from(shape: Polyomino<P>) -> Self {
        Self {
            shape,
            remaining_points: ArrayVec::from(shape.0),
        }
    }
}

impl<const P: usize> Iterator for RectangleIter<P> {
    type Item = Rectangle;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(p1) = self.remaining_points.pop()
        else{
            return None
        };
        let mut min_x = p1.x();
        let mut max_x = p1.x();
        let mut min_y = p1.y();

        while let Some((index, &p2)) = self
            .remaining_points
            .iter()
            .find_position(|p2| p2.y() == min_y && (p2.x() == max_x + 1 || p2.x() == min_x - 1))
        {
            self.remaining_points.swap_remove(index);
            min_x = min_x.min(p2.x());
            max_x = max_x.max(p2.x());
        }
        let range = min_x..=max_x;

        let mut max_y = p1.y();

        'outer: loop {
            for is_max in [false, true] {
                let y = if is_max { max_y + 1 } else { min_y - 1 };
                let condition = |p2: &&PointRelative8| p2.y() == y && range.contains(&p2.x());
                if self.remaining_points.iter().filter(condition).count() == range.len() {
                    while let Some((position, _)) =
                        self.remaining_points.iter().find_position(condition)
                    {
                        self.remaining_points.swap_remove(position);
                    }
                    if is_max {
                        max_y += 1;
                    } else {
                        min_y -= 1;
                    }

                    continue 'outer;
                }
            }
            break 'outer;
        }

        let width = (max_x + 1 - min_x) as u8;
        let height = (max_y + 1 - min_y) as u8;
        Some(Rectangle {
            x: min_x,
            y: min_y,
            width,
            height,
        })
    }
}

impl<const P: usize> Polyomino<P> {
    pub fn deconstruct_into_rectangles(&self) -> impl Iterator<Item = Rectangle> {
        RectangleIter::from(self.clone())
    }
}
