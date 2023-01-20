use super::relative_coordinate::RelativeCoordinate;
use strum::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Polyomino<const POINTS: usize>(pub [RelativeCoordinate; POINTS]);

impl<const P: usize> IntoIterator for Polyomino<P> {
    type Item = RelativeCoordinate;

    type IntoIter = core::array::IntoIter<RelativeCoordinate, P>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Polyomino<1> {
    pub const MONOMINO: Self = Self([RelativeCoordinate::ZERO]);
}

impl Polyomino<2> {
    pub const DOMINO: Self = Self([RelativeCoordinate::ZERO, RelativeCoordinate::NORTH]);
}

impl Polyomino<3> {
    pub const I_TROMINO: Self = Self([RelativeCoordinate::EAST, RelativeCoordinate::ZERO, RelativeCoordinate::WEST]);
    pub const V_TROMINO: Self = Self([RelativeCoordinate::EAST, RelativeCoordinate::ZERO, RelativeCoordinate::NORTH]);
}

impl Polyomino<4> {
    pub const I_TETROMINO: Self = Self([RelativeCoordinate::EAST, RelativeCoordinate::ZERO, RelativeCoordinate::WEST, RelativeCoordinate::WEST_TWO]);
    pub const O_TETROMINO: Self = Self([RelativeCoordinate::ZERO, RelativeCoordinate::EAST, RelativeCoordinate::NORTHEAST, RelativeCoordinate::NORTH]);
    pub const T_TETROMINO: Self = Self([RelativeCoordinate::EAST, RelativeCoordinate::ZERO, RelativeCoordinate::WEST, RelativeCoordinate::SOUTH]);
    pub const J_TETROMINO: Self = Self([RelativeCoordinate::WEST, RelativeCoordinate::ZERO, RelativeCoordinate::NORTH, RelativeCoordinate::NORTH_TWO]);
    pub const L_TETROMINO: Self = Self([RelativeCoordinate::EAST, RelativeCoordinate::ZERO, RelativeCoordinate::NORTH, RelativeCoordinate::NORTH_TWO]);
    pub const S_TETROMINO: Self = Self([RelativeCoordinate::WEST, RelativeCoordinate::ZERO, RelativeCoordinate::NORTH, RelativeCoordinate::NORTHEAST]);
    pub const Z_TETROMINO: Self = Self([RelativeCoordinate::EAST, RelativeCoordinate::ZERO, RelativeCoordinate::NORTH, RelativeCoordinate::NORTHWEST]);

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
    pub const F_PENTOMINO: Self = Self([RelativeCoordinate::ZERO, RelativeCoordinate::NORTH, RelativeCoordinate::NORTHEAST, RelativeCoordinate::WEST, RelativeCoordinate::SOUTH]);
    pub const I_PENTOMINO: Self =
        Self([RelativeCoordinate::ZERO, RelativeCoordinate::NORTH, RelativeCoordinate::NORTH_TWO, RelativeCoordinate::SOUTH, RelativeCoordinate::SOUTH_TWO]);
    pub const L_PENTOMINO: Self =
        Self([RelativeCoordinate::ZERO, RelativeCoordinate::NORTH, RelativeCoordinate::NORTH_TWO, RelativeCoordinate::SOUTH, RelativeCoordinate::SOUTHEAST]);
    pub const N_PENTOMINO: Self =
        Self([RelativeCoordinate::ZERO, RelativeCoordinate::NORTH, RelativeCoordinate::NORTH_TWO, RelativeCoordinate::WEST, RelativeCoordinate::SOUTHWEST]);
    pub const P_PENTOMINO: Self = Self([RelativeCoordinate::NORTH, RelativeCoordinate::ZERO, RelativeCoordinate::NORTHEAST, RelativeCoordinate::EAST, RelativeCoordinate::SOUTH]);
    pub const T_PENTOMINO: Self =
        Self([RelativeCoordinate::ZERO, RelativeCoordinate::NORTH, RelativeCoordinate::NORTHEAST, RelativeCoordinate::NORTHWEST, RelativeCoordinate::SOUTH]);
    pub const U_PENTOMINO: Self =
        Self([RelativeCoordinate::ZERO, RelativeCoordinate::NORTHEAST, RelativeCoordinate::EAST, RelativeCoordinate::NORTHWEST, RelativeCoordinate::WEST]);
    pub const V_PENTOMINO: Self =
        Self([RelativeCoordinate::ZERO, RelativeCoordinate::NORTH, RelativeCoordinate::NORTH_TWO, RelativeCoordinate::WEST, RelativeCoordinate::WEST_TWO]);
    pub const W_PENTOMINO: Self =
        Self([RelativeCoordinate::ZERO, RelativeCoordinate::EAST, RelativeCoordinate::NORTHEAST, RelativeCoordinate::SOUTH, RelativeCoordinate::SOUTHWEST]);
    pub const X_PENTOMINO: Self = Self([RelativeCoordinate::ZERO, RelativeCoordinate::NORTH, RelativeCoordinate::EAST, RelativeCoordinate::SOUTH, RelativeCoordinate::WEST]);
    pub const Y_PENTOMINO: Self = Self([RelativeCoordinate::ZERO, RelativeCoordinate::NORTH, RelativeCoordinate::EAST, RelativeCoordinate::WEST, RelativeCoordinate::WEST_TWO]);
    pub const Z_PENTOMINO: Self =
        Self([RelativeCoordinate::ZERO, RelativeCoordinate::NORTH, RelativeCoordinate::NORTHWEST, RelativeCoordinate::SOUTH, RelativeCoordinate::SOUTHEAST]);

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

pub trait PolyominoShape {
    type OutlineIter: Iterator<Item = RelativeCoordinate>;
    fn draw_outline(&self) -> Self::OutlineIter;

    fn get_centre(&self) -> (f32, f32);

    fn first_point(&self) -> RelativeCoordinate;
}

impl<const P: usize> PolyominoShape for Polyomino<P> {
    type OutlineIter = OutlineIter<P>;

    fn draw_outline(&self) -> Self::OutlineIter {
        let mut arr = self.0;
        arr.sort_unstable();
        OutlineIter {
            arr,
            next: Some((arr[0], Corner::NorthWest)),
        }
    }

    fn get_centre(&self) -> (f32, f32) {
        let mut x = 0;
        let mut y = 0;

        for point in self.0 {
            x += point.x();
            y += point.y();
        }

        (
            0.5 + ((x as f32) / (P as f32)),
            0.5 + ((y as f32) / (P as f32)),
        )
    }

    fn first_point(&self) -> RelativeCoordinate {
        self.0[0]
    }
}

pub struct OutlineIter<const POINTS: usize> {
    arr: [RelativeCoordinate; POINTS],
    next: Option<(RelativeCoordinate, Corner)>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
enum Corner {
    NorthWest,
    NorthEast,
    SouthEast,
    SouthWest,
}

impl Corner {
    pub fn clockwise_direction(&self) -> RelativeCoordinate {
        match self {
            Corner::NorthWest => RelativeCoordinate::NORTH,
            Corner::NorthEast => RelativeCoordinate::EAST,
            Corner::SouthEast => RelativeCoordinate::SOUTH,
            Corner::SouthWest => RelativeCoordinate::WEST,
        }
    }

    pub fn clockwise(&self) -> Self {
        use Corner::*;
        match self {
            Corner::NorthWest => NorthEast,
            Corner::NorthEast => SouthEast,
            Corner::SouthEast => SouthWest,
            Corner::SouthWest => NorthWest,
        }
    }

    pub fn anticlockwise(&self) -> Self {
        use Corner::*;
        match self {
            Corner::NorthWest => SouthWest,
            Corner::NorthEast => NorthWest,
            Corner::SouthEast => NorthEast,
            Corner::SouthWest => SouthEast,
        }
    }

    pub fn direction_of_northwest_corner(&self) -> RelativeCoordinate {
        match self {
            Corner::NorthWest => RelativeCoordinate::ZERO,
            Corner::NorthEast => RelativeCoordinate::EAST,
            Corner::SouthEast => RelativeCoordinate::SOUTHEAST,
            Corner::SouthWest => RelativeCoordinate::SOUTH,
        }
    }
}

impl<const POINTS: usize> Iterator for OutlineIter<POINTS> {
    type Item = RelativeCoordinate;

    fn next(&mut self) -> Option<Self::Item> {
        let mut direction_so_far: Option<RelativeCoordinate> = None;
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

        //println!("{} {}", coordinate_to_return, corner_to_return);
        let r = coordinate_to_return + corner_to_return.direction_of_northwest_corner();

        Some(r)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::rectangle::*;
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
    fn test_pentomino_rectangles(){
        for (shape, name) in Polyomino::FREE_PENTOMINOS
        .iter()
        .zip(Polyomino::FREE_PENTOMINO_NAMES)
    {
        test_deconstruct_into_rectangles(shape, (name.to_string() + " pentomino rectangles").as_str())
    }
    }

    fn test_outline<P: PolyominoShape>(shape: &'static P, name: &str) {
        let outline: Vec<_> = shape.draw_outline().take(100).collect();
        assert!(outline.len() < 100);
        let max_x = outline.iter().map(|q| q.x()).max().unwrap() as f32;
        let max_y = outline.iter().map(|q| q.y()).max().unwrap() as f32;

        let min_x = outline.iter().map(|q| q.x()).min().unwrap() as f32;
        let min_y = outline.iter().map(|q| q.y()).min().unwrap() as f32;

        let (centre_x, centre_y) = shape.get_centre();

        assert!(centre_x < max_x);
        assert!(centre_y < max_y);

        assert!(centre_x > min_x);
        assert!(centre_y > min_y);

        insta::assert_debug_snapshot!(name, outline);
    }

    

    fn test_deconstruct_into_rectangles<const P : usize>(shape: &'static Polyomino<P>, name: &str) {

        let rectangles = shape.deconstruct_into_rectangles().collect_vec();

        insta::assert_debug_snapshot!(name, rectangles);
    }
}
