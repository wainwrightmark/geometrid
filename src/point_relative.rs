use core::{
    fmt::Debug,
    ops::{Add, Mul, Neg},
};

// use super::grid_error::GridError;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct PointRelative {
    x: i16,
    y: i16,
}

impl core::fmt::Display for PointRelative {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if *self == PointRelative::ZERO {
            write!(f, "Zero")
        } else if let Some(index) = PointRelative::UNITS.iter().position(|x| x == self) {
            let name = PointRelative::UNIT_NAMES[index];
            write!(f, "{name}")
        } else {
            f.debug_struct("RelativeCoordinate")
                .field("x", &self.x)
                .field("y", &self.y)
                .finish()
        }
    }
}

impl PointRelative {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const NORTH: Self = Self { x: 0, y: -1 };
    pub const NORTH_TWO: Self = Self { x: 0, y: -2 };
    pub const NORTHEAST: Self = Self { x: 1, y: -1 };
    pub const EAST: Self = Self { x: 1, y: 0 };
    pub const EAST_TWO: Self = Self { x: 2, y: 0 };
    pub const SOUTHEAST: Self = Self { x: 1, y: 1 };
    pub const SOUTH: Self = Self { x: 0, y: 1 };
    pub const SOUTH_TWO: Self = Self { x: 0, y: 2 };
    pub const SOUTHWEST: Self = Self { x: -1, y: 1 };
    pub const WEST: Self = Self { x: -1, y: 0 };
    pub const WEST_TWO: Self = Self { x: -2, y: 0 };
    pub const NORTHWEST: Self = Self { x: -1, y: -1 };

    pub const CARDINALS: [Self; 4] = [Self::NORTH, Self::EAST, Self::SOUTH, Self::WEST];
    pub const UNITS: [Self; 8] = [
        Self::NORTH,
        Self::NORTHEAST,
        Self::EAST,
        Self::SOUTHEAST,
        Self::SOUTH,
        Self::SOUTHWEST,
        Self::WEST,
        Self::NORTHWEST,
    ];

    pub const UNIT_NAMES: [&'static str; 8] = [
        "North",
        "North East",
        "East",
        "South East",
        "South",
        "South West",
        "West",
        "North West",
    ];

    #[inline]
    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn x(&self) -> i16 {
        self.x
    }

    #[inline]
    pub const fn y(&self) -> i16 {
        self.y
    }

    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    #[inline]
    pub const fn is_unit(&self) -> bool {
        self.x.abs() <= 1 && self.y.abs() <= 1 && !self.is_zero()
    }

    #[inline]
    pub const fn is_diagonal(&self) -> bool {
        self.x != 0 && self.y != 0
    }
    /// Flip the direction: N -> S, E -> W, etc.
    #[inline]
    pub fn flip(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }

    #[inline]
    pub fn rotate(&self, quarter_turns: u8) -> Self {
        match quarter_turns % 4 {
            1 => Self::new(self.y(), -self.x()),
            2 => Self::new(-self.x(), -self.y()),
            3 => Self::new(-self.y(), self.x()),
            _ => *self,
        }
    }
}

impl Neg for PointRelative {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.flip()
    }
}

impl Neg for &PointRelative {
    type Output = PointRelative;
    fn neg(self) -> Self::Output {
        self.flip()
    }
}

impl Add for PointRelative {
    type Output = PointRelative;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for &PointRelative {
    type Output = PointRelative;

    fn add(self, rhs: Self) -> Self::Output {
        PointRelative {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<i16> for PointRelative {
    type Output = PointRelative;

    fn mul(self, rhs: i16) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

// impl Mul<usize> for RelativeCoordinate {
//     type Output = RelativeCoordinate;

//     fn mul(self, rhs: usize) -> Self::Output {
//         Self {
//             x: self.x * (rhs as i16),
//             y: self.y * (rhs as i16),
//         }
//     }
// }
