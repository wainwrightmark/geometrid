use core::ops::Sub;

use crate::prelude::*;
use itertools::Itertools;
use strum::Display;

#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};
use tinyvec::ArrayVec;

/// A polyomino with a fixed number of points
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(any(test, feature = "serde"), derive(Serialize, Deserialize))]
pub struct Polyomino<const POINTS: usize>(
    #[cfg_attr(any(test, feature = "serde"), serde(with = "serde_arrays"))]
    pub  [DynamicTile; POINTS],
);

impl<const P: usize> Shape for Polyomino<P> {
    type OutlineIter = OutlineIter<P>;

    type RectangleIter = RectangleIter<P>;

    fn draw_outline(&self) -> Self::OutlineIter {
        let mut arr = self.0;
        arr.sort_unstable();
        OutlineIter {
            arr,
            next: Some((arr[0], Corner::NorthWest)),
        }
    }

    fn deconstruct_into_rectangles(&self) -> Self::RectangleIter {
        (*self).into()
    }
}

impl<const P: usize> HasCenter for Polyomino<P> {
    fn get_center(&self, scale: f32) -> Location {
        let mut x = 0;
        let mut y = 0;

        for point in self.0 {
            x += point.x;
            y += point.y;
        }

        Location {
            x: (0.5 + ((x as f32) / (P as f32))) * scale,
            y: (0.5 + ((y as f32) / (P as f32))) * scale,
        }
    }
}

impl<const P: usize> IntoIterator for Polyomino<P> {
    type Item = DynamicTile;

    type IntoIter = core::array::IntoIter<DynamicTile, P>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<const P: usize> Polyomino<P> {
    const fn new(vectors: [Vector; P]) -> Self {
        let mut arr = [DynamicTile(Vector::ZERO); P];

        let mut i = 0;
        while i < P {
            arr[i] = DynamicTile(vectors[i]);
            i += 1;
        }
        Self(arr)
    }
}

impl Polyomino<1> {
    pub const MONOMINO: Self = Self::new([Vector::ZERO]);
}

impl Polyomino<2> {
    pub const DOMINO: Self = Self::new([Vector::ZERO, Vector::NORTH]);
}

impl Polyomino<3> {
    pub const I_TROMINO: Self = Self::new([Vector::EAST, Vector::ZERO, Vector::WEST]);
    pub const V_TROMINO: Self = Self::new([Vector::EAST, Vector::ZERO, Vector::NORTH]);
}

impl Polyomino<4> {
    pub const I_TETROMINO: Self = Self::new([
        Vector::EAST,
        Vector::ZERO,
        Vector::WEST,
        Vector::WEST.const_mul(2),
    ]);
    pub const O_TETROMINO: Self = Self::new([
        Vector::ZERO,
        Vector::EAST,
        Vector::NORTH_EAST,
        Vector::NORTH,
    ]);
    pub const T_TETROMINO: Self =
        Self::new([Vector::EAST, Vector::ZERO, Vector::WEST, Vector::SOUTH]);
    pub const J_TETROMINO: Self = Self::new([
        Vector::WEST,
        Vector::ZERO,
        Vector::NORTH,
        Vector::NORTH.const_mul(2),
    ]);
    pub const L_TETROMINO: Self = Self::new([
        Vector::EAST,
        Vector::ZERO,
        Vector::NORTH,
        Vector::NORTH.const_mul(2),
    ]);
    pub const S_TETROMINO: Self = Self::new([
        Vector::WEST,
        Vector::ZERO,
        Vector::NORTH,
        Vector::NORTH_EAST,
    ]);
    pub const Z_TETROMINO: Self = Self::new([
        Vector::EAST,
        Vector::ZERO,
        Vector::NORTH,
        Vector::NORTH_WEST,
    ]);

    pub const TETROMINOS: [Self; 7] = [
        Self::I_TETROMINO,
        Self::O_TETROMINO,
        Self::T_TETROMINO,
        Self::J_TETROMINO,
        Self::L_TETROMINO,
        Self::S_TETROMINO,
        Self::Z_TETROMINO,
    ];

    pub const TETROMINO_NAMES: [&'static str; 7] = ["I", "O", "T", "J", "L", "S", "Z"];

    pub const FREE_TETROMINOS: [Self; 5] = [
        Self::I_TETROMINO,
        Self::O_TETROMINO,
        Self::T_TETROMINO,
        Self::L_TETROMINO,
        Self::S_TETROMINO,
    ];

    pub const FREE_TETROMINO_NAMES: [&'static str; 5] = ["I", "O", "T", "L", "S"];
}

impl Polyomino<5> {
    pub const F_PENTOMINO: Self = Self::new([
        Vector::ZERO,
        Vector::NORTH,
        Vector::NORTH_EAST,
        Vector::WEST,
        Vector::SOUTH,
    ]);
    pub const I_PENTOMINO: Self = Self::new([
        Vector::ZERO,
        Vector::NORTH,
        Vector::NORTH.const_mul(2),
        Vector::SOUTH,
        Vector::SOUTH.const_mul(2),
    ]);
    pub const L_PENTOMINO: Self = Self::new([
        Vector::ZERO,
        Vector::NORTH,
        Vector::NORTH.const_mul(2),
        Vector::SOUTH,
        Vector::SOUTH_EAST,
    ]);
    pub const N_PENTOMINO: Self = Self::new([
        Vector::ZERO,
        Vector::NORTH,
        Vector::NORTH.const_mul(2),
        Vector::WEST,
        Vector::SOUTH_WEST,
    ]);
    pub const P_PENTOMINO: Self = Self::new([
        Vector::NORTH,
        Vector::ZERO,
        Vector::NORTH_EAST,
        Vector::EAST,
        Vector::SOUTH,
    ]);
    pub const T_PENTOMINO: Self = Self::new([
        Vector::ZERO,
        Vector::NORTH,
        Vector::NORTH_EAST,
        Vector::NORTH_WEST,
        Vector::SOUTH,
    ]);
    pub const U_PENTOMINO: Self = Self::new([
        Vector::ZERO,
        Vector::NORTH_EAST,
        Vector::EAST,
        Vector::NORTH_WEST,
        Vector::WEST,
    ]);
    pub const V_PENTOMINO: Self = Self::new([
        Vector::ZERO,
        Vector::NORTH,
        Vector::NORTH.const_mul(2),
        Vector::WEST,
        Vector::WEST.const_mul(2),
    ]);
    pub const W_PENTOMINO: Self = Self::new([
        Vector::ZERO,
        Vector::EAST,
        Vector::NORTH_EAST,
        Vector::SOUTH,
        Vector::SOUTH_WEST,
    ]);
    pub const X_PENTOMINO: Self = Self::new([
        Vector::ZERO,
        Vector::NORTH,
        Vector::EAST,
        Vector::SOUTH,
        Vector::WEST,
    ]);
    pub const Y_PENTOMINO: Self = Self::new([
        Vector::ZERO,
        Vector::NORTH,
        Vector::EAST,
        Vector::WEST,
        Vector::WEST.const_mul(2),
    ]);
    pub const Z_PENTOMINO: Self = Self::new([
        Vector::ZERO,
        Vector::NORTH,
        Vector::NORTH_WEST,
        Vector::SOUTH,
        Vector::SOUTH_EAST,
    ]);

    pub const FREE_PENTOMINOS: [Self; 12] = [
        Self::F_PENTOMINO,
        Self::I_PENTOMINO,
        Self::L_PENTOMINO,
        Self::N_PENTOMINO,
        Self::P_PENTOMINO,
        Self::T_PENTOMINO,
        Self::U_PENTOMINO,
        Self::V_PENTOMINO,
        Self::W_PENTOMINO,
        Self::X_PENTOMINO,
        Self::Y_PENTOMINO,
        Self::Z_PENTOMINO,
    ];

    pub const FREE_PENTOMINO_NAMES: [&'static str; 12] =
        ["F", "I", "L", "N", "P", "T", "U", "V", "W", "X", "Y", "Z"];
}

pub struct OutlineIter<const POINTS: usize> {
    arr: [DynamicTile; POINTS],
    next: Option<(DynamicTile, Corner)>,
}

impl Corner {
    pub const fn clockwise_direction(&self) -> Vector {
        match self {
            Corner::NorthWest => Vector::NORTH,
            Corner::NorthEast => Vector::EAST,
            Corner::SouthEast => Vector::SOUTH,
            Corner::SouthWest => Vector::WEST,
        }
    }

    pub const fn clockwise(&self) -> Self {
        use Corner::*;

        match self {
            NorthWest => NorthEast,
            NorthEast => SouthEast,
            SouthEast => SouthWest,
            SouthWest => NorthWest,
        }
    }

    pub fn anticlockwise(&self) -> Self {
        use Corner::*;
        match self {
            NorthWest => SouthWest,
            SouthWest => SouthEast,
            SouthEast => NorthEast,
            NorthEast => NorthWest,
        }
    }
}

impl<const POINTS: usize> Iterator for OutlineIter<POINTS> {
    type Item = DynamicVertex;

    fn next(&mut self) -> Option<Self::Item> {
        let mut direction_so_far: Option<Vector> = None;
        let (coordinate_to_return, corner_to_return) = self.next?;

        let mut next_coordinate = coordinate_to_return;
        let mut next_corner = corner_to_return;

        'line: loop {
            'equivalency: loop {
                let equivalent = next_coordinate + next_corner.clockwise_direction();
                if self.arr.contains(&equivalent) {
                    //perform an equivalency
                    next_coordinate = equivalent;
                    next_corner = next_corner.anticlockwise();
                    if next_coordinate == coordinate_to_return {
                        panic!("Infinite loop found in shape.")
                    }
                    if next_corner == Corner::NorthWest && next_coordinate == self.arr[0] {
                        break 'line;
                    }
                } else {
                    break 'equivalency;
                }
            }

            match direction_so_far {
                None => {
                    direction_so_far = Some(next_corner.clockwise_direction());
                    next_corner = next_corner.clockwise();
                }
                Some(d) => {
                    if d == next_corner.clockwise_direction() {
                        next_corner = next_corner.clockwise();
                    } else {
                        break 'line;
                    }
                }
            }
            if next_corner == Corner::NorthWest && next_coordinate == self.arr[0] {
                break 'line;
            }
        }

        if next_corner == Corner::NorthWest && next_coordinate == self.arr[0] {
            self.next = None;
        } else {
            self.next = Some((next_coordinate, next_corner));
        }

        let r = coordinate_to_return.get_vertex(&corner_to_return);

        Some(r)
    }
}

/// Iterator for deconstructing polyominos to rectangles
pub struct RectangleIter<const P: usize> {
    shape: Polyomino<P>,
    remaining_tiles: ArrayVec<[DynamicTile; P]>,
}

impl<const P: usize> From<Polyomino<P>> for RectangleIter<P> {
    fn from(shape: Polyomino<P>) -> Self {
        Self {
            shape,
            remaining_tiles: ArrayVec::from(shape.0),
        }
    }
}

impl<const P: usize> Iterator for RectangleIter<P> {
    type Item = Rectangle;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(p1) = self.remaining_tiles.pop()
        else{
            return None
        };
        let mut min_x = p1.x;
        let mut max_x = p1.x;
        let mut min_y = p1.y;

        while let Some((index, &p2)) = self
            .remaining_tiles
            .iter()
            .find_position(|p2| p2.y == min_y && (p2.x == max_x + 1 || p2.x == min_x - 1))
        {
            self.remaining_tiles.swap_remove(index);
            min_x = min_x.min(p2.x);
            max_x = max_x.max(p2.x);
        }
        let range = min_x..=max_x;

        let mut max_y = p1.y;

        'outer: loop {
            for is_max in [false, true] {
                let y = if is_max { max_y + 1 } else { min_y - 1 };
                let condition = |p2: &&DynamicTile| p2.y == y && range.contains(&p2.x);
                if self.remaining_tiles.iter().filter(condition).count() == range.len() {
                    while let Some((position, _)) =
                        self.remaining_tiles.iter().find_position(condition)
                    {
                        self.remaining_tiles.swap_remove(position);
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

        let north_west = Vector { x: min_x, y: min_y }.into();
        let width: u8 = max_x.abs_diff(min_x) + 1;
        let height: u8 = max_y.abs_diff(min_y) + 1;

        Some(Rectangle {
            north_west,
            height,
            width,
        })
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::prelude::*;
    use itertools::Itertools;

    #[test]
    fn test_basic_outlines() {
        test_outline(&Polyomino::MONOMINO, "Square outline");
        test_outline(&Polyomino::DOMINO, "Domino outline");
    }

    #[test]
    fn test_tetromino_outlines() {
        for (shape, name) in Polyomino::TETROMINOS.iter().zip(Polyomino::TETROMINO_NAMES) {
            test_outline(shape, (name.to_string() + " tetromino outline").as_str())
        }
    }

    #[test]
    fn test_pentomino_outlines() {
        for (shape, name) in Polyomino::FREE_PENTOMINOS
            .iter()
            .zip(Polyomino::FREE_PENTOMINO_NAMES)
        {
            test_outline(shape, (name.to_string() + " pentomino outline").as_str())
        }
    }

    #[test]
    fn test_pentomino_rectangles() {
        for (shape, name) in Polyomino::FREE_PENTOMINOS
            .iter()
            .zip(Polyomino::FREE_PENTOMINO_NAMES)
        {
            test_deconstruct_into_rectangles(
                shape,
                (name.to_string() + " pentomino rectangles").as_str(),
            )
        }
    }

    fn test_outline<P: Shape + HasCenter>(shape: &'static P, name: &str) {
        let outline: Vec<_> = shape.draw_outline().take(100).collect();
        assert!(outline.len() < 100);
        let max_x = outline.iter().map(|q| q.x).max().unwrap() as f32;
        let max_y = outline.iter().map(|q| q.y).max().unwrap() as f32;

        let min_x = outline.iter().map(|q| q.x).min().unwrap() as f32;
        let min_y = outline.iter().map(|q| q.y).min().unwrap() as f32;

        let Location {
            x: centre_x,
            y: centre_y,
        } = shape.get_center(1.0);

        assert!(centre_x < max_x);
        assert!(centre_y < max_y);

        assert!(centre_x > min_x);
        assert!(centre_y > min_y);

        insta::assert_json_snapshot!(name, outline);
    }

    fn test_deconstruct_into_rectangles<const P: usize>(shape: &'static Polyomino<P>, name: &str) {
        let rectangles = shape.deconstruct_into_rectangles().collect_vec();
        let sum: usize = rectangles.iter().map(|x| x.area()).sum();
        assert_eq!(sum, P);

        insta::assert_json_snapshot!(name, rectangles);
    }
}
