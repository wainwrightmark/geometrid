use super::{coordinate::Qa, qgrid::QGrid, relative_coordinate::Qr};

impl<T, const WIDTH: u16, const HEIGHT: u16, const SIZE: usize> QGrid<T, WIDTH, HEIGHT, SIZE> {
    pub fn get_lines<'a, F: Fn(&T) -> bool>(
        &'a self,
        directions: &'a [Qr],
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

struct LineFinder<'a, T, const WIDTH: u16, const HEIGHT: u16, const SIZE: usize, F: Fn(&T) -> bool>
{
    pub grid: &'a QGrid<T, WIDTH, HEIGHT, SIZE>,
    pub directions: &'a [Qr],
    pub check_item: F,
    pub position: Qa<WIDTH, HEIGHT>,
    pub direction_index: usize,
    pub min_length: usize,
}

pub struct Line<'a, T, const WIDTH: u16, const HEIGHT: u16> {
    pub first_item: &'a T,
    pub origin: Qa<WIDTH, HEIGHT>,
    pub direction: Qr,
    pub length: usize,
}

impl<'a, T, const WIDTH: u16, const HEIGHT: u16> Line<'a, T, WIDTH, HEIGHT> {
    pub fn positions(&self) -> impl Iterator<Item = Qa<WIDTH, HEIGHT>> + '_ {
        (0..self.length).map(|x| (self.origin + (self.direction * x)).unwrap())
    }
}

impl<'a, T, const WIDTH: u16, const HEIGHT: u16, const SIZE: usize, F: Fn(&T) -> bool> Iterator
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
                        let Ok(next) = current + direction else{break 'len;};
                        current = next;
                        let item2 = &self.grid[next];

                        if (self.check_item)(item2) {
                            length += 1;
                        } else {
                            break 'len;
                        }
                    }
                    if length >= self.min_length {
                        return Some(Line {
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