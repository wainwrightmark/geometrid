use core::{fmt, ops::Add};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)] //TODO make inner type generic
pub struct AbsoluteCoordinate<const WIDTH: u16, const HEIGHT: u16> {
    inner: u16,
}


impl<const W: u16, const H: u16> fmt::Display for AbsoluteCoordinate<W, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x(), self.y())
    }
}

impl<const W: u16, const H: u16> AbsoluteCoordinate<W, H> {
    pub const WIDTH: u16 = W;
    pub const HEIGHT: u16 = H;
    pub const SIZE: usize = W as usize * H as usize;
    pub const CENTER: AbsoluteCoordinate<W, H> = Self::new_unchecked(W / 2, H / 2);

    pub const TOP_LEFT: AbsoluteCoordinate<W, H> = Self::new_const::<0, 0>();
    pub const TOP_RIGHT: AbsoluteCoordinate<W, H> = Self::new_unchecked(W - 1, 0);
    pub const BOTTOM_LEFT: AbsoluteCoordinate<W, H> = Self::new_unchecked(0, H - 1);
    pub const BOTTOM_RIGHT: AbsoluteCoordinate<W, H> = Self::new_unchecked(W - 1, H - 1);

    pub const fn new_const<const X: u16, const Y: u16>() -> Self {
        assert!(X < W);
        assert!(Y < H);
        Self::new_unchecked(X, Y)
    }

    #[inline]
    const fn new_unchecked(x: u16, y: u16) -> Self {
        Self { inner: x + (W * y) }
    }

    #[inline]
    pub const fn try_new(x: u16, y: u16) -> Option<Self> {
        if x >= W || y >= H {
            None
        } else {
            Some(Self::new_unchecked(x, y))
        }
    }

    pub const fn try_from_usize(value: usize)-> Option<Self>{
        if value < Self::SIZE{
            Some(Self{inner: value as u16})
        }else{
            None
        }
    }

    pub const fn try_next(&self) -> Option<Self> {
        let new_index = self.inner + 1;
        if new_index >= W * H {
            None
        } else {
            Some(Self { inner: new_index })
        }
    }

    #[inline]
    pub const fn x(&self) -> u16 {
        self.inner % W
    }

    #[inline]
    pub const fn y(&self) -> u16 {
        self.inner / W
    }

    #[inline]
    pub const fn is_corner(&self) -> bool {
        (self.x() == 0 || self.x() == W - 1) && (self.y() == 0 || self.y() == H - 1)
    }

    #[inline]
    pub const fn is_edge(&self) -> bool {
        self.x() == 0 || self.x() == W - 1 || self.y() == 0 || self.y() == H - 1
    }

    #[inline]
    pub const fn flip_horizontal(&self) -> AbsoluteCoordinate<W, H> {
        Self::new_unchecked(W - self.x() - 1, self.y())
    }

    #[inline]
    pub const fn flip_vertical(&self) -> AbsoluteCoordinate<W, H> {
        Self::new_unchecked(self.x(), H - self.y() - 1)
    }

    #[inline]
    /// The distance, disallowing, diagonal moves
    pub const fn manhattan_distance(&self, other: &Self) -> u16 {
        let dx = u16::abs_diff(self.x(), other.x());
        let dy = u16::abs_diff(self.y(), other.y());
        dx + dy
    }

    #[cfg(std)]
    #[inline]
    /// The distance, allowing diagonal moves
    /// Requires std
    pub fn distance(&self, other: &Self) -> f32 {
        let dx: f32 = u16::abs_diff(self.x(), other.x()) as f32;
        let dy: f32 = u16::abs_diff(self.y(), other.y()) as f32;
        f32::sqrt((dx * dx) + (dy * dy))
    }

    pub fn get_length_multiplier(total_width: f32, total_height: f32) -> f32 {
        let x_multiplier = total_width / W as f32;
        let y_multiplier = total_height / H as f32;

        x_multiplier.min(y_multiplier)
    }

    pub fn get_location(&self, multiplier: f32, x_ratio: f32, y_ratio: f32) -> (f32, f32) {
        let x = multiplier * ((self.x() as f32) + x_ratio);
        let y = multiplier * ((self.y() as f32) + y_ratio);

        (x, y)
    }
}

impl<const L: u16> AbsoluteCoordinate<L, L> {
    /// Rotate the coordinate clockwise
    #[inline]
    pub fn rotate(&self, quarter_turns: usize) -> Self {
        match quarter_turns % 4 {
            1 => Self::new_unchecked(L - 1 - self.y(), self.x()),
            2 => Self::new_unchecked(L - 1 - self.x(), L - 1 - self.y()),
            3 => Self::new_unchecked(self.y(), L - 1 - self.x()),
            _ => *self,
        }
    }
}

// impl<const W: u16, const H: u16> TryFrom<usize> for Qa<W, H> {
//     type Error = GridError;

//     fn try_from(value: usize) -> Result<Self, Self::Error> {
//         if value >= Self::SIZE {
//             Err(GridError::OutOfBounds)
//         } else {
//             Ok(Self {
//                 inner: value as u16,
//             })
//         }
//     }
// }

impl<const W: u16, const H: u16> From<AbsoluteCoordinate<W, H>> for usize {
    fn from(val: AbsoluteCoordinate<W, H>) -> Self {
        val.inner as usize
    }
}

impl<const W: u16, const H: u16> From<&AbsoluteCoordinate<W, H>> for usize {
    fn from(val: &AbsoluteCoordinate<W, H>) -> Self {
        val.inner as usize
    }
}

impl<const W: u16, const H: u16> Add<super::relative_coordinate::RelativeCoordinate> for AbsoluteCoordinate<W, H> {
    type Output = Option<Self>;

    fn add(self, rhs: super::relative_coordinate::RelativeCoordinate) -> Self::Output {
        let x = (self.x() as i16) + rhs.x();
        let y = (self.y() as i16) + rhs.y();
        if x >= 0 && y >= 0 {
            Self::try_new(x as u16, y as u16)
        } else {
            None
        }
    }
}
