use crate::prelude::*;
use itertools::Itertools;

#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};
use tinyvec::ArrayVec;

type V = Vector;

/// A polyomino with a fixed number of points
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(any(test, feature = "serde"), derive(Serialize, Deserialize))]
pub struct Polyomino<const TILES: usize>(
    #[cfg_attr(any(test, feature = "serde"), serde(with = "serde_arrays"))] [DynamicTile; TILES],
);

impl<const P: usize> Shape for Polyomino<P> {
    type OutlineIter = OutlineIter<P>;

    type RectangleIter = RectangleIter<P>;

    fn draw_outline(&self) -> Self::OutlineIter {
        let arr = self.0;
        OutlineIter {
            arr,
            next: Some((arr[0], Corner::NorthWest)),
        }
    }

    fn deconstruct_into_rectangles(&self) -> Self::RectangleIter {
        (*self).into()
    }
}

#[cfg(any(test, feature = "glam"))]
impl<const P: usize> HasCenter for Polyomino<P> {
    fn get_center(&self, scale: f32) -> glam::f32::Vec2 {
        let mut x = 0;
        let mut y = 0;

        for point in self.0 {
            x += point.x;
            y += point.y;
        }

        glam::f32::Vec2 {
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

const fn sort_vectors<const N: usize>(mut arr: [Vector; N]) -> [Vector; N] {
    let mut i = 1;
    while i < N {
        let mut j = i;
        while j > 0 && arr[j - 1].const_gt(&arr[j]) {
            let swap = arr[j - 1];
            arr[j - 1] = arr[j];
            arr[j] = swap;
            j -= 1;
        }
        i += 1;
    }

    arr
}

/// Translate vectors so that the minimum values of x and y are both 0
const fn normalize_vectors<const N: usize>(mut arr: [Vector; N]) -> [Vector; N] {
    let mut min_x = i8::MAX;
    let mut min_y = i8::MAX;

    let mut i = 0;
    while i < N {
        if arr[i].x < min_x {
            min_x = arr[i].x;
        }
        if arr[i].y < min_y {
            min_y = arr[i].y;
        }
        i += 1;
    }

    let mut i = 0;
    while i < N {
        arr[i].x -= min_x;
        arr[i].y -= min_y;
        i += 1;
    }
    arr
}

impl<const T: usize> Polyomino<T> {
    /// Create a new polyomino.
    /// Note that this will normalize and sort all of the vectors.
    pub const fn new(vectors: [Vector; T]) -> Self {
        let vectors = normalize_vectors(vectors);
        let vectors = sort_vectors(vectors);
        let mut arr = [DynamicTile(V::ZERO); T];

        let mut i = 0;
        while i < T {
            arr[i] = DynamicTile(vectors[i]);
            i += 1;
        }

        Self(arr)
    }

    const ASCII_TILE: u8 = b'#';
    const ASCII_SPACE: u8 = b'.';

    const fn new_from_ascii(s: &str) -> Self {
        match Self::try_new_from_ascii(s) {
            Ok(p) => p,
            Err(e) => const_panic::concat_panic!(e),
        }
    }

    /// Construct a polyomino from a string of ascii
    /// Tiles are represented by `#`. Empty tiles by `.`.
    /// Whitespace apart from newlines are ignored.
    /// Returns error if invalid
    pub const fn try_new_from_ascii(s: &str) -> Result<Self, &'static str> {
        let mut current = V::ZERO;
        let mut arr: [Vector; T] = [V::ZERO; T];
        let mut index = 0;

        let bytes = s.as_bytes();
        let mut bytes_index = 0;

        while bytes_index < bytes.len() {
            let character = bytes[bytes_index];
            if character == b'\n' {
                current.x = 0;
                current = current.const_add(&V::SOUTH);
            } else if character.is_ascii_whitespace() {
                //Ignore other ascii whitespace
            } else if character == Self::ASCII_SPACE {
                current = current.const_add(&V::EAST);
            } else if character == Self::ASCII_TILE {
                if index >= arr.len() {
                    return Err("Too Many Tiles");
                }

                arr[index] = current;
                index += 1;
                current = current.const_add(&V::EAST);
            } else {
                return Err("Unexpected Character");
            }
            bytes_index += 1;
        }

        if index != arr.len() {
            return Err("Not enough tiles");
        }

        Ok(Self::new(arr))
    }

    /// The tiles of this polyomino
    pub fn tiles(&self) -> &[DynamicTile; T] {
        &self.0
    }

    /// Write the polyomino as an ascii string.
    /// Requires `std`
    #[cfg(any(test, feature = "std"))]
    pub fn to_ascii_string(&self) -> String {
        if T == 0 {
            return "".to_string();
        }
        let min_x = self.0.iter().map(|t| t.0.x).min().unwrap();
        let max_x = self.0.iter().map(|t| t.0.x).max().unwrap();
        let min_y = self.0.iter().map(|t| t.0.y).min().unwrap();
        let max_y = self.0.iter().map(|t| t.0.y).max().unwrap();

        let width = (max_x - min_x + 1) as usize;
        let height = (max_y - min_y + 1) as usize;

        let len = (width + 1) * height - 1;
        let mut bytes: Vec<u8> = vec![Self::ASCII_SPACE; len];

        for r in 1..height {
            let index = (r * (width + 1)) - 1;
            bytes[index] = b'\n';
        }

        for tile in self.0 {
            let (x, y) = (tile.x - min_x, tile.y - min_y);
            let index = x + (y * (width + 1) as i8);
            let index = index as usize;
            bytes[index] = Self::ASCII_TILE;
        }

        String::from_utf8(bytes).unwrap()
    }
}

impl Polyomino<1> {
    pub const MONOMINO: Self = Self::new_from_ascii(
        "\
#\
",
    );
}

impl Polyomino<2> {
    pub const DOMINO: Self = Self::new_from_ascii(
        "\
##\
",
    );
}

impl Polyomino<3> {
    pub const I_TROMINO: Self = Self::new_from_ascii(
        "\
    ###\
    ",
    );
    pub const V_TROMINO: Self = Self::new_from_ascii(
        "
    ##\n\
    #.\
    ",
    );
}

impl Polyomino<4> {
    pub const I_TETROMINO: Self = Self::new_from_ascii(
        "\
####\
",
    );
    pub const O_TETROMINO: Self = Self::new_from_ascii(
        "\
##\n\
##\
",
    );
    pub const T_TETROMINO: Self = Self::new_from_ascii(
        "\
###\n\
.#.\
",
    );
    pub const J_TETROMINO: Self = Self::new_from_ascii(
        "\
###\n\
..#\
",
    );
    pub const L_TETROMINO: Self = Self::new_from_ascii(
        "\
..#\n\
###\
",
    );
    pub const S_TETROMINO: Self = Self::new_from_ascii(
        "\
.##\n\
##.\
",
    );
    pub const Z_TETROMINO: Self = Self::new_from_ascii(
        "\
##.\n\
.##\
",
    );

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
    pub const F_PENTOMINO: Self = Self::new_from_ascii(
        "\
.##\n\
##.\n\
.#.\
",
    );
    pub const I_PENTOMINO: Self = Self::new_from_ascii(
        "\
#####\
",
    );
    pub const L_PENTOMINO: Self = Self::new_from_ascii(
        "\
...#\n\
####\
",
    );
    pub const N_PENTOMINO: Self = Self::new_from_ascii(
        "\
##..\n\
.###\
",
    );
    pub const P_PENTOMINO: Self = Self::new_from_ascii(
        "\
##\n\
##\n\
#.\
",
    );
    pub const T_PENTOMINO: Self = Self::new_from_ascii(
        "\
###\n\
.#.\n\
.#.\
",
    );
    pub const U_PENTOMINO: Self = Self::new_from_ascii(
        "\
#.#\n\
###\
",
    );
    pub const V_PENTOMINO: Self = Self::new_from_ascii(
        "\
..#\n\
..#\n\
###\
",
    );
    pub const W_PENTOMINO: Self = Self::new_from_ascii(
        "\
..#\n\
.##\n\
##.\
",
    );
    pub const X_PENTOMINO: Self = Self::new_from_ascii(
        "\
.#.\n\
###\n\
.#.\
",
    );
    pub const Y_PENTOMINO: Self = Self::new_from_ascii(
        "\
..#.\n\
####\
",
    );
    pub const Z_PENTOMINO: Self = Self::new_from_ascii(
        "\
##.\n\
.#.\n\
.##\
",
    );

    pub const S_PENTOMINO: Self = Self::new_from_ascii(
        "\
.##\n\
.#.\n\
##\
",
    );

    pub const J_PENTOMINO: Self = Self::new_from_ascii(
        "\
#...\n\
####\
",
    );

    pub const Q_PENTOMINO: Self = Self::new_from_ascii(
        "\
##\n\
##\n\
.#\
",
    );

    pub const LAMBDA_PENTOMINO: Self = Self::new_from_ascii(
        "\
.#..\n\
####\
",
    );

    pub const SEVEN_PENTOMINO: Self = Self::new_from_ascii(
        "\
##.\n\
.##\n\
.#.\
",
    );

    pub const FIVE_PENTOMINO: Self = Self::new_from_ascii(
        "\
..##\n\
###.\
",
    );

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

    pub const ALL_PENTOMINOS: [Self; 18] = [
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
        Self::SEVEN_PENTOMINO,
        Self::J_PENTOMINO,
        Self::FIVE_PENTOMINO,
        Self::Q_PENTOMINO,
        Self::LAMBDA_PENTOMINO,
        Self::S_PENTOMINO,
    ];

    pub const ALL_PENTOMINO_NAMES: [&'static str; 18] = [
        "F", "I", "L", "N", "P", "T", "U", "V", "W", "X", "Y", "Z", "7", "J", "5", "Q", "λ", "S",
    ];
}

/// WARNING hexomino names are subject to change
/// //TODO more hexominos
impl Polyomino<6> {
    pub const I_HEXOMINO: Self = Self::new_from_ascii(
        "\
######\
",
    );
    pub const J_HEXOMINO: Self = Self::new_from_ascii(
        "\
#....\n\
#####\
",
    );
    pub const LONG_Y_HEXOMINO: Self = Self::new_from_ascii(
        "\
.#...\n\
#####\
",
    );
    pub const FAT_T_HEXOMINO: Self = Self::new_from_ascii(
        "\
..#..\n\
#####\
",
    );
    pub const LAMBDA_HEXOMINO: Self = Self::new_from_ascii(
        "\
##...\n\
.####\
",
    );
    pub const P_HEXOMINO: Self = Self::new_from_ascii(
        "\
##..\n\
####\
",
    );
    pub const F_HEXOMINO: Self = Self::new_from_ascii(
        "\
#.#.\n\
####\
",
    );
    pub const C_HEXOMINO: Self = Self::new_from_ascii(
        "\
#..#\n\
####\
",
    );
    pub const PI_HEXOMINO: Self = Self::new_from_ascii(
        "\
.##.\n\
####\
",
    );
    pub const RHO_HEXOMINO: Self = Self::new_from_ascii(
        "\
#...\n\
#...\n\
####\
",
    );
    pub const TAU_HEXOMINO: Self = Self::new_from_ascii(
        "\
#..\n\
###\n\
#..\n\
#..\
",
    );
    pub const T_HEXOMINO: Self = Self::new_from_ascii(
        "\
#...\n\
####\n\
#...\
",
    );
    pub const F2_HEXOMINO: Self = Self::new_from_ascii(
        "\
.##\n\
##.\n\
.#.\n\
.#.\
",
    );
    pub const F3_HEXOMINO: Self = Self::new_from_ascii(
        "\
.##\n\
.#.\n\
##.\n\
.#.\
",
    );
    pub const S_HEXOMINO: Self = Self::new_from_ascii(
        "\
.##\n\
.#.\n\
.#.\n\
##.\
",
    );
    pub const X_HEXOMINO: Self = Self::new_from_ascii(
        "\
.#.\n\
.##\n\
##.\n\
.#.\
",
    );
    pub const CROSS_HEXOMINO: Self = Self::new_from_ascii(
        "\
.#.\n\
###\n\
.#.\n\
.#.\
",
    );
    pub const FOUR_HEXOMINO: Self = Self::new_from_ascii(
        "\
.#.\n\
###\n\
#..\n\
#..\
",
    );
    pub const N_HEXOMINO: Self = Self::new_from_ascii(
        "\
.#\n\
##\n\
#.\n\
##\
",
    );
    pub const Z_HEXOMINO: Self = Self::new_from_ascii(
        "\
.#\n\
.#\n\
##\n\
#.\n\
#.\
",
    );
    pub const EIGHT_HEXOMINO: Self = Self::new_from_ascii(
        "\
.#\n\
##\n\
##\n\
#.\
",
    );
    pub const O_HEXOMINO: Self = Self::new_from_ascii(
        "\
##\n\
##\n\
##\
",
    );
    pub const SEVEN_HEXOMINO: Self = Self::new_from_ascii(
        "\
..#\n\
###\n\
.#.\n\
.#.\
",
    );
    pub const D_HEXOMINO: Self = Self::new_from_ascii(
        "\
###\n\
.##\n\
.#.\
",
    );
    pub const THREE_HEXOMINO: Self = Self::new_from_ascii(
        "\
..#\n\
.##\n\
##.\n\
.#.\
",
    );
    pub const TWO_HEXOMINO: Self = Self::new_from_ascii(
        "\
..#\n\
###\n\
#..\n\
#..\
",
    );
    pub const W_HEXOMINO: Self = Self::new_from_ascii(
        "\
.##\n\
##.\n\
#..\n\
#..\
",
    );
    pub const U_HEXOMINO: Self = Self::new_from_ascii(
        "\
###\n\
#.#\n\
#..\
",
    );
    pub const ONE_HEXOMINO: Self = Self::new_from_ascii(
        "\
#.#\n\
###\n\
#..\
",
    );
    pub const Y_HEXOMINO: Self = Self::new_from_ascii(
        "\
#.#\n\
###\n\
.#.\
",
    );
    pub const FIVE_HEXOMINO: Self = Self::new_from_ascii(
        "\
.##\n\
.#.\n\
##.\n\
#..\
",
    );
    pub const V_HEXOMINO: Self = Self::new_from_ascii(
        "\
#..\n\
##.\n\
###\
",
    );
    pub const AMPERSAND_HEXOMINO: Self = Self::new_from_ascii(
        "\
.#.\n\
###\n\
##.\
",
    );
    pub const Q_HEXOMINO: Self = Self::new_from_ascii(
        "\
..#\n\
###\n\
##.\
",
    );
    pub const M_HEXOMINO: Self = Self::new_from_ascii(
        "\
..#\n\
.##\n\
##.\n\
#..\
",
    );

    pub const FREE_HEXOMINOS: [Self; 2] = [Self::I_HEXOMINO, Self::J_HEXOMINO];
}

pub struct OutlineIter<const POINTS: usize> {
    arr: [DynamicTile; POINTS],
    next: Option<(DynamicTile, Corner)>,
}

impl Corner {
    pub const fn clockwise_direction(&self) -> Vector {
        match self {
            Corner::NorthWest => V::NORTH,
            Corner::NorthEast => V::EAST,
            Corner::SouthEast => V::SOUTH,
            Corner::SouthWest => V::WEST,
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
    remaining_tiles: ArrayVec<[DynamicTile; P]>,
}

impl<const P: usize> From<Polyomino<P>> for RectangleIter<P> {
    fn from(shape: Polyomino<P>) -> Self {
        Self {
            remaining_tiles: ArrayVec::from(shape.0),
        }
    }
}

impl<const P: usize> Iterator for RectangleIter<P> {
    type Item = Rectangle;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(p1) = self.remaining_tiles.pop() else {
            return None;
        };
        let mut min_x = p1.x;
        let mut max_x = p1.x;
        let mut min_y = p1.y;

        while let Some((index, &p2)) = self
            .remaining_tiles
            .iter()
            .find_position(|p2| p2.y == min_y && (p2.x == max_x + 1 || p2.x == min_x - 1))
        {
            let _ = self.remaining_tiles.swap_remove(index);
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
                        let _ = self.remaining_tiles.swap_remove(position);
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
    use super::*;
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
        for (shape, name) in Polyomino::ALL_PENTOMINOS
            .iter()
            .zip(Polyomino::ALL_PENTOMINO_NAMES)
        {
            test_outline(shape, (name.to_string() + " pentomino outline").as_str())
        }
    }

    #[test]
    fn test_pentomino_rectangles() {
        for (shape, name) in Polyomino::ALL_PENTOMINOS
            .iter()
            .zip(Polyomino::ALL_PENTOMINO_NAMES)
        {
            test_deconstruct_into_rectangles(
                shape,
                (name.to_string() + " pentomino rectangles").as_str(),
            )
        }
    }

    #[test]
    fn test_pentomino_ascii_strings() {
        for (shape, name) in Polyomino::ALL_PENTOMINOS
            .iter()
            .zip(Polyomino::ALL_PENTOMINO_NAMES)
        {
            let ascii = shape.to_ascii_string();
            let rt = match Polyomino::<5>::try_new_from_ascii(&ascii) {
                Ok(rt) => rt,
                Err(err) => {
                    println!("{name}");
                    println!("{ascii}");
                    panic!("{err}");
                }
            };

            if &rt != shape {
                println!("{name}");
                println!("{ascii}");

                let rt_ascii = rt.to_ascii_string();

                println!("vs");
                println!("{rt_ascii}");

                panic!("Shape {name} did not roundtrip correctly")
            }
        }
    }

    #[test]
    fn test_hexomino_ascii_strings() {
        for (index, shape) in Polyomino::FREE_HEXOMINOS.iter().enumerate() {
            let ascii = shape.to_ascii_string();
            let rt = match Polyomino::<6>::try_new_from_ascii(&ascii) {
                Ok(rt) => rt,
                Err(err) => {
                    println!("Hexomino {index}");
                    println!("{ascii}");
                    panic!("{err}");
                }
            };

            if &rt != shape {
                println!("Hexomino {index}");
                println!("{ascii}");

                let rt_ascii = rt.to_ascii_string();

                println!("vs");
                println!("{rt_ascii}");

                panic!("Hexomino {index} did not roundtrip correctly")
            }
        }
    }

    fn test_outline<P: Shape + HasCenter>(shape: &'static P, name: &str) {
        let outline: Vec<_> = shape.draw_outline().take(100).collect();
        assert!(outline.len() < 100);
        let max_x = outline.iter().map(|q| q.x).max().unwrap() as f32;
        let max_y = outline.iter().map(|q| q.y).max().unwrap() as f32;

        let min_x = outline.iter().map(|q| q.x).min().unwrap() as f32;
        let min_y = outline.iter().map(|q| q.y).min().unwrap() as f32;

        let glam::f32::Vec2 {
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
