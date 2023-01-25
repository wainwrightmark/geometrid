use crate::location::*;
pub struct Vertex8<const W: u8, const H: u8>(u8);

impl<const W: u8, const H: u8> Vertex8<W, H> {
    pub const WIDTH: u8 = W;
    pub const HEIGHT: u8 = H;
    pub const SIZE: u8 = W * H;
    pub const COUNT: u8 = (W + 1) * (H + 1);
    pub const CENTER: Self = Self::new_unchecked((W + 1) / 2, (H + 1) / 2);

    pub const TOP_LEFT: Self = Self::new_const::<0, 0>();
    pub const TOP_RIGHT: Self = Self::new_const::<W, 0>();
    pub const BOTTOM_LEFT: Self = Self::new_const::<0, H>();
    pub const BOTTOM_RIGHT: Self = Self::new_const::<W, H>();

    #[must_use]
    pub const fn new_const<const X: u8, const Y: u8>() -> Self {
        Self::new_unchecked(X, Y)
    }

    #[must_use]
    #[inline]
    pub(crate) const fn new_unchecked(x: u8, y: u8) -> Self {
        debug_assert!(x <= Self::WIDTH);
        debug_assert!(y <= Self::HEIGHT);
        debug_assert!(Self::COUNT <= <u8>::MAX);
        Self((x + (W * y)))
    }

    #[must_use]
    #[inline]
    pub const fn try_new(x: u8, y: u8) -> Option<Self> {
        if x > W || y > H {
            None
        } else {
            Some(Self::new_unchecked(x, y))
        }
    }

    #[must_use]
    #[inline]
    pub const fn try_from_inner(inner: u8) -> Option<Self> {
        if inner <= Self::BOTTOM_RIGHT.0 {
            Some(Self(inner))
        } else {
            None
        }
    }

    #[must_use]
    #[inline]
    pub fn try_from_usize(inner: usize) -> Option<Self> {
        let Ok(inner) = inner.try_into() else{
                                                            return None;
                                                        };

        Self::try_from_inner(inner)
    }

    #[must_use]
    #[inline]
    pub const fn x(&self) -> u8 {
        self.0 % (W + 1)
    }

    #[must_use]
    #[inline]
    pub const fn y(&self) -> u8 {
        self.0 / (W + 1)
    }

    #[must_use]
    #[inline]
    pub const fn flip_horizontal(&self) -> Self {
        Self::new_unchecked(W - self.x(), self.y())
    }

    #[must_use]
    #[inline]
    pub const fn flip_vertical(&self) -> Self {
        Self::new_unchecked(self.x(), H - self.y())
    }

    #[must_use]
    #[inline]
    /// The distance, disallowing, diagonal moves
    pub const fn manhattan_distance(&self, other: &Self) -> u8 {
        let dx = u8::abs_diff(self.x(), other.x());
        let dy = u8::abs_diff(self.y(), other.y());
        dx + dy
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> HasLocation for Vertex8<WIDTH, HEIGHT> {
    fn location(&self, scale: f32) -> Location {
        let x = scale * (self.x() as f32);
        let y = scale * (self.y() as f32);

        Location { x, y }
    }
}
