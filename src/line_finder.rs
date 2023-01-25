use crate::{grid::*, tile::*, vector::*};

macro_rules! line_finder {
    ($line_finder_name:ident, $line_name:ident, $grid_ty:ident, $point_relative_ty:ident, $point_absolute_ty:ident, $inner:ty) => {
        impl<T, const WIDTH: $inner, const HEIGHT: $inner, const SIZE: usize>
            $grid_ty<T, WIDTH, HEIGHT, SIZE>
        {
            pub fn get_lines<'a, F: Fn(&T) -> bool>(
                &'a self,
                directions: &'a [$point_relative_ty],
                check_item: F,
                min_length: usize,
            ) -> impl Iterator<Item = $line_name<'a, T, WIDTH, HEIGHT>> {
                $line_finder_name {
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
        struct $line_finder_name<
            'a,
            T,
            const WIDTH: $inner,
            const HEIGHT: $inner,
            const SIZE: usize,
            F: Fn(&T) -> bool,
        > {
            pub grid: &'a $grid_ty<T, WIDTH, HEIGHT, SIZE>,
            pub directions: &'a [$point_relative_ty],
            pub check_item: F,
            pub position: $point_absolute_ty<WIDTH, HEIGHT>,
            pub direction_index: usize,
            pub min_length: usize,
        }

        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct $line_name<'a, T, const WIDTH: $inner, const HEIGHT: $inner> {
            pub first_item: &'a T,
            pub origin: $point_absolute_ty<WIDTH, HEIGHT>,
            pub direction: $point_relative_ty,
            pub length: usize,
        }

        impl<'a, T, const WIDTH: $inner, const HEIGHT: $inner> $line_name<'a, T, WIDTH, HEIGHT> {
            pub fn positions(
                &self,
            ) -> impl Iterator<Item = $point_absolute_ty<WIDTH, HEIGHT>> + '_ {
                (0..self.length).map(|x| (self.origin + (self.direction * x)).unwrap())
            }
        }

        impl<
                'a,
                T,
                const WIDTH: $inner,
                const HEIGHT: $inner,
                const SIZE: usize,
                F: Fn(&T) -> bool,
            > Iterator for $line_finder_name<'a, T, WIDTH, HEIGHT, SIZE, F>
        {
            type Item = $line_name<'a, T, WIDTH, HEIGHT>;

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
                                return Some($line_name {
                                    first_item: item,
                                    origin: self.position,
                                    direction,
                                    length,
                                });
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
    };
}

line_finder!(
    LineFinder64,
    Line64,
    Grid64,
    Vector64,
    Tile64,
    u64
);
line_finder!(
    LineFinder32,
    Line32,
    Grid32,
    Vector32,
    Tile32,
    u32
);
line_finder!(
    LineFinder16,
    Line16,
    Grid16,
    Vector16,
    Tile16,
    u16
);
line_finder!(
    LineFinder8,
    Line8,
    Grid8,
    Vector8,
    Tile8,
    u8
);


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use itertools::Itertools;

    #[test]
    fn basic_tests() {
        let mut grid: Grid8<usize, 3,3,9>= Grid8::default();

        for (i, mut m) in grid.iter_mut().enumerate() {
            *m = i;
        }

        let lines = grid.get_lines(&[Vector8::DOWN, Vector8::RIGHT, Vector8::DOWN_RIGHT], |x| x % 2 == 0, 3 ).collect_vec();

        assert_eq!(1, lines.len());
        let line = lines.into_iter().next().unwrap();
        let line_string = line.positions().join("|");
        assert_eq!(line_string, "(0,0)|(1,1)|(2,2)");
    }
}