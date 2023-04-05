use crate::prelude::*;

/// Iterates all tiles in a line between `from` and `to` in some order
pub fn line_of_sight_tiles<const COLS: u8, const ROWS: u8>(
    from: &Tile<COLS, ROWS>,
    to: &Tile<COLS, ROWS>,
) -> impl Iterator<Item = Tile<COLS, ROWS>> {
    LineOfSightTileIter {
        from: *from,
        to: *to,
        state: State::Default,
    }
}

#[derive(Debug, Clone)]
/// Iterates all tiles in a line between `from` and `to` in some order
struct LineOfSightTileIter<const COLS: u8, const ROWS: u8> {
    pub state: State,
    pub from: Tile<COLS, ROWS>,
    pub to: Tile<COLS, ROWS>,
}

impl<const COLS: u8, const ROWS: u8> Iterator for LineOfSightTileIter<COLS, ROWS> {
    type Item = Tile<COLS, ROWS>;

    fn next(&mut self) -> Option<Self::Item> {
        //println!("{self:?}");
        match self.state {
            State::Default => {
                let abs_x = self.from.col().abs_diff(self.to.col());
                let abs_y = self.from.row().abs_diff(self.to.row());

                // let vector = ;
                self.state = if abs_x == abs_y {
                    if abs_x == 0 {
                        State::Complete
                    } else {
                        let x = if self.from.col() < self.to.col() {
                            1
                        } else {
                            -1
                        };
                        let y = if self.from.row() < self.to.row() {
                            1
                        } else {
                            -1
                        };
                        State::Diagonal1(Vector { x, y })
                    }
                } else {
                    let vector = if abs_x > abs_y {
                        if self.from.col() < self.to.col() {
                            Vector::EAST
                        } else {
                            Vector::WEST
                        }
                    } else {
                        if self.from.row() < self.to.row() {
                            Vector::SOUTH
                        } else {
                            Vector::NORTH
                        }
                    };
                    State::Parallel1(vector)
                };

                return Some(self.from);
            }
            State::Parallel1(vector) => {
                let next = self.to;

                self.from = (self.from + vector).unwrap();

                self.state = if self.from == self.to {
                    State::Complete
                } else {
                    State::Default
                };

                self.to = (self.to + vector.const_neg()).unwrap();

                return Some(next);
            }
            State::Diagonal1(vector) => {
                self.state = State::Diagonal2(vector);
                let next = (self.from + vector.horizontal_component()).unwrap();
                return Some(next);
            }
            State::Diagonal2(vector) => {
                self.state = State::Default;
                let next = (self.from + vector.vertical_component()).unwrap();
                self.from = (self.from + vector).unwrap();
                return Some(next);
            }
            State::Complete => return None,
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
enum State {
    #[default]
    Default,
    Parallel1(Vector),

    Diagonal1(Vector),
    Diagonal2(Vector),

    Complete,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::*;
    use itertools::Itertools;
    use serde_test::{assert_tokens, Token};
    type Tile25 = Tile<5, 5>;

    #[test]
    fn south_east_diagonal() {
        test_line_of_sight(Tile25::NORTH_WEST, Tile25::SOUTH_EAST, "(0,0); (1,0); (0,1); (1,1); (2,1); (1,2); (2,2); (3,2); (2,3); (3,3); (4,3); (3,4); (4,4)")
    }

    #[test]
    fn north_west_diagonal() {
        test_line_of_sight(Tile25::SOUTH_EAST, Tile25::NORTH_WEST, "(0,0); (1,0); (0,1); (1,1); (2,1); (1,2); (2,2); (3,2); (2,3); (3,3); (4,3); (3,4); (4,4)")
    }

    #[test]
    fn straight_north() {
        test_line_of_sight(
            Tile25::new_const::<2, 4>(),
            Tile25::new_const::<2, 0>(),
            "(2,0); (2,1); (2,2); (2,3); (2,4)",
        )
    }

    #[test]
    fn straight_west() {
        test_line_of_sight(
            Tile25::new_const::<4, 2>(),
            Tile25::new_const::<0, 2>(),
            "(0,2); (1,2); (2,2); (3,2); (4,2)",
        )
    }

    #[test]
    fn partial_diagonal1() {
        test_line_of_sight(
            Tile25::new_const::<2, 0>(),
            Tile25::new_const::<3, 4>(),
            "(2,0); (2,1); (2,2); (3,2); (3,3); (3,4)",
        )
    }

    #[test]
    fn partial_diagonal2() {
        test_line_of_sight(
            Tile25::new_const::<2, 0>(),
            Tile25::new_const::<3, 3>(),
            "(2,0); (2,1); (3,1); (2,2); (3,2); (3,3)",
        )
    }

    fn test_line_of_sight(from: Tile25, to: Tile25, expected: &str) {
        let mut actual = line_of_sight_tiles(&from, &to).collect_vec();
        actual.sort();

        assert_eq!(actual.into_iter().join("; "), expected,)
    }
}
