pub use crate::prelude::*;

impl<T, const WIDTH: u8, const HEIGHT: u8, const SIZE: usize> TileMap<T, WIDTH, HEIGHT, SIZE> {
    pub fn get_lines<'a, F: Fn(&T) -> bool>(
        &'a self,
        directions: &'a [Vector],
        check_item: F,
        min_length: usize,
    ) -> impl Iterator<Item = Line<'a, T, WIDTH, HEIGHT>> {
        LineFinder {
            grid: self,
            directions,
            position: Default::default(),
            direction_index: 0,
            check_item,
            min_length,
        }
    }
}

#[derive(Debug, Clone)]
struct LineFinder<'a, T, const WIDTH: u8, const HEIGHT: u8, const SIZE: usize, F: Fn(&T) -> bool> {
    pub grid: &'a TileMap<T, WIDTH, HEIGHT, SIZE>,
    pub directions: &'a [Vector],
    pub check_item: F,
    pub position: Tile<WIDTH, HEIGHT>,
    pub direction_index: usize,
    pub min_length: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Line<'a, T, const WIDTH: u8, const HEIGHT: u8> {
    pub first_item: &'a T,
    pub origin: Tile<WIDTH, HEIGHT>,
    pub direction: Vector,
    pub length: usize,
}

impl<'a, T, const WIDTH: u8, const HEIGHT: u8> Line<'a, T, WIDTH, HEIGHT> {
    pub fn positions(&self) -> impl Iterator<Item = Tile<WIDTH, HEIGHT>> + '_ {
        (0..self.length).map(|x| (self.origin + (self.direction * x)).unwrap())
    }
}

impl<'a, T, const WIDTH: u8, const HEIGHT: u8, const SIZE: usize, F: Fn(&T) -> bool> Iterator
    for LineFinder<'a, T, WIDTH, HEIGHT, SIZE, F>
{
    type Item = Line<'a, T, WIDTH, HEIGHT>;

    fn next(&mut self) -> Option<Self::Item> {
        'items: loop {
            let item = &self.grid[self.position];
            if (self.check_item)(item) {
                while self.direction_index < self.directions.len() {
                    let direction = self.directions[self.direction_index];
                    self.direction_index += 1;
                    let mut length = 1;
                    let mut current = self.position;
                    'len: loop {
                        let Some(next) = current + direction else{break 'len;};
                        current = next;
                        let item2 = &self.grid[next];

                        if (self.check_item)(item2) {
                            length += 1;
                        } else {
                            break 'len;
                        }
                    }
                    if length >= self.min_length {
                        let line = Line {
                            first_item: item,
                            origin: self.position,
                            direction,
                            length,
                        };
                        return Some(line);
                    }
                }
                self.direction_index = 0;
            }
            let Some(new_position) = self.position.try_next() else{break 'items;};
            self.position = new_position
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::*;
    use itertools::Itertools;
    use serde_test::{assert_tokens, Token};

    #[test]
    pub fn test_line_finder_none() {
        let mut map: TileMap<bool, 4, 4, 16> = Default::default();

        map[Tile::new_const::<0, 1>()] = true;
        map[Tile::new_const::<1, 1>()] = true;
        map[Tile::new_const::<2, 0>()] = true;
        map[Tile::new_const::<3, 0>()] = true;

        let lines = map
            .get_lines(&[Vector::EAST, Vector::WEST], |x| *x, 4)
            .collect_vec();

        assert_eq!(lines.len(), 0);
    }

    #[test]
    pub fn test_line_finder_4() {
        let mut map: TileMap<bool, 4, 4, 16> = Default::default();
        map[Tile::new_const::<0, 0>()] = true;
        map[Tile::new_const::<1, 1>()] = true;
        map[Tile::new_const::<2, 2>()] = true;
        map[Tile::new_const::<3, 3>()] = true;

        let lines = map
            .get_lines(
                &[
                    Vector::SOUTH,
                    Vector::EAST,
                    Vector::SOUTH_EAST,
                    Vector::NORTH_EAST,
                ],
                |x| *x,
                4,
            )
            .collect_vec();

        assert_eq!(lines.len(), 1);

        let line = lines[0].clone();

        assert_eq!(line.first_item, &true);
        assert_eq!(line.length, 4);
        assert_eq!(line.origin, Tile::new_const::<0, 0>());
        assert_eq!(line.direction, Vector::SOUTH_EAST);
    }
}
