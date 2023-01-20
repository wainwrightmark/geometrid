use itertools::Itertools;

use crate::{relative_coordinate::RelativeCoordinate, shape::Shape};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Rectangle{
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}


pub struct RectangleIter{

}

impl<const P: usize> Shape<P> {
    pub fn deconstruct_into_rectangles(&self) -> Vec<Rectangle> {
        let mut results = vec![];

        let mut remaining_points = self.0.to_vec();

        while let Some(p1) = remaining_points.pop() {
            let mut min_x = p1.x();
            let mut max_x = p1.x();
            let mut min_y = p1.y();

            while let Some((index, &p2)) = remaining_points
                .iter()
                .find_position(|p2| p2.y() == min_y && (p2.x() == max_x + 1 || p2.x() == min_x - 1))
            {
                remaining_points.swap_remove(index);
                min_x = min_x.min(p2.x());
                max_x = max_x.max(p2.x());
            }
            let range = min_x..=max_x;

            let mut max_y = p1.y();

            'outer: loop {
                for is_max in [false, true] {
                    let y = if is_max { max_y + 1 } else { min_y - 1 };
                    let condition = |p2: &&RelativeCoordinate| p2.y() == y && range.contains(&p2.x());
                    if remaining_points.iter().filter(condition).count() == range.len() {
                        while let Some((position, _)) =
                            remaining_points.iter().find_position(condition)
                        {
                            remaining_points.swap_remove(position);
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

            let width = (max_x + 1 - min_x) as u16;
            let height = (max_y + 1 - min_y) as u16;
            results.push(Rectangle{x: min_x, y:min_y, width, height});
        }

        results
    }
}