use core::{fmt, ops::Add};

use itertools::Itertools;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::point_relative::PointRelative8;

macro_rules! point_absolute {
    ($name:ident, $inner:ty) => {
        #[must_use]
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)] //TODO make inner type generic
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct $name<const WIDTH: $inner, const HEIGHT: $inner>($inner);

        impl<const W: $inner, const H: $inner> fmt::Display for $name<W, H> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({},{})", self.x(), self.y())
            }
        }

        impl<const W: $inner, const H: $inner> $name<W, H> {
            pub const WIDTH: $inner = W;
            pub const HEIGHT: $inner = H;
            pub const SIZE: $inner = W as $inner * H as $inner;
            pub const CENTER: Self = Self::new_unchecked(W / 2, H / 2);

            pub const TOP_LEFT: Self = Self::new_const::<0, 0>();
            pub const TOP_RIGHT: Self = Self::new_unchecked(W - 1, 0);
            pub const BOTTOM_LEFT: Self = Self::new_unchecked(0, H - 1);
            pub const BOTTOM_RIGHT: Self = Self::new_unchecked(W - 1, H - 1);

            #[must_use]
            pub const fn new_const<const X: $inner, const Y: $inner>() -> Self {
                assert!(X < W);
                assert!(Y < H);
                Self::new_unchecked(X, Y)
            }

            #[must_use]
            #[inline]
            pub(crate) const fn new_unchecked(x: $inner, y: $inner) -> Self {
                Self((x + (W * y)))
            }

            #[must_use]
            #[inline]
            pub const fn try_new(x: $inner, y: $inner) -> Option<Self> {
                if x >= W || y >= H {
                    None
                } else {
                    Some(Self::new_unchecked(x, y))
                }
            }

            #[must_use]
            #[inline]
            pub const fn try_from_inner(inner: $inner) -> Option<Self> {
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
            pub const fn try_next(&self) -> Option<Self> {
                let new_index = self.0 + 1;
                if new_index >= W * H {
                    None
                } else {
                    Some(Self(new_index))
                }
            }

            #[must_use]
            #[inline]
            pub const fn x(&self) -> $inner {
                self.0 % W
            }

            #[must_use]
            #[inline]
            pub const fn y(&self) -> $inner {
                self.0 / W
            }

            #[must_use]
            #[inline]
            pub const fn is_corner(&self) -> bool {
                (self.x() == 0 || self.x() == W - 1) && (self.y() == 0 || self.y() == H - 1)
            }

            #[must_use]
            #[inline]
            pub const fn is_edge(&self) -> bool {
                self.x() == 0 || self.x() == W - 1 || self.y() == 0 || self.y() == H - 1
            }

            #[must_use]
            #[inline]
            pub const fn flip_horizontal(&self) -> Self {
                Self::new_unchecked(W - self.x() - 1, self.y())
            }

            #[must_use]
            #[inline]
            pub const fn flip_vertical(&self) -> Self {
                Self::new_unchecked(self.x(), H - self.y() - 1)
            }

            #[must_use]
            #[inline]
            /// The distance, disallowing, diagonal moves
            pub const fn manhattan_distance(&self, other: &Self) -> $inner {
                let dx = <$inner>::abs_diff(self.x(), other.x());
                let dy = <$inner>::abs_diff(self.y(), other.y());
                dx + dy
            }

            #[cfg(std)]
            #[inline]
            #[must_use]
            /// The angle to the other point, allowing diagonal moves
            /// Requires std
            pub fn angle_to(&self, other: &Self) -> f32 {
                let x_diff = other.x() as f32 - self.x() as f32;
                let y_diff = other.y() as f32 - self.y() as f32;

                (y_diff).atan2(x_diff).to_degrees()
            }

            #[cfg(std)]
            #[inline]
            #[must_use]
            /// The distance to the other point, allowing diagonal moves
            /// Requires std
            pub fn distance(&self, other: &Self) -> f32 {
                let dx: f32 = <$inner>::abs_diff(self.x(), other.x()) as f32;
                let dy: f32 = <$inner>::abs_diff(self.y(), other.y()) as f32;
                f32::sqrt((dx * dx) + (dy * dy))
            }

            /// Get the length to multiply by to make the grid take up as much as possible of a given area
            #[must_use]
            pub fn get_length_multiplier(total_width: f32, total_height: f32) -> f32 {
                let x_multiplier = total_width / W as f32;
                let y_multiplier = total_height / H as f32;

                x_multiplier.min(y_multiplier)
            }

            #[must_use]
            pub fn get_location(&self, multiplier: f32, x_ratio: f32, y_ratio: f32) -> (f32, f32) {
                let x = multiplier * ((self.x() as f32) + x_ratio);
                let y = multiplier * ((self.y() as f32) + y_ratio);

                (x, y)
            }

            ///True if two coordinates are orthogonal or diagonal
            #[must_use]
            pub fn is_adjacent(self, other: Self) -> bool {
                if self == other {
                    return false;
                };

                self.y().abs_diff(other.y()) <= 1 && self.x().abs_diff(other.x()) <= 1
            }

            ///True if two coordinates are orthogonal (adjacent but not diagonal)
            #[must_use]
            pub fn is_orthogonal(self, other: Self) -> bool {
                if self == other {
                    return false;
                };

                let y_diff = self.y().abs_diff(other.y());
                let x_diff = self.x().abs_diff(other.y());

                y_diff <= 1 && x_diff <= 1 && (y_diff == 0 || x_diff == 0)
            }

            #[must_use]
            pub fn points_by_row() -> impl Iterator<Item = Self> {
                (0..H)
                    .cartesian_product(0..W)
                    .map(|(y, x)| Self::new_unchecked(x, y))
            }

            #[must_use]
            pub fn points_by_col() -> impl Iterator<Item = Self> {
                (0..W)
                    .cartesian_product(0..H)
                    .map(|(x, y)| Self::new_unchecked(x, y))
            }
        }

        impl<const L: $inner> $name<L, L> {
            /// Rotate the coordinate clockwise
            #[inline]
            pub fn rotate(&self, quarter_turns: $inner) -> Self {
                match quarter_turns % 4 {
                    1 => Self::new_unchecked(L - 1 - self.y(), self.x()),
                    2 => Self::new_unchecked(L - 1 - self.x(), L - 1 - self.y()),
                    3 => Self::new_unchecked(self.y(), L - 1 - self.x()),
                    _ => *self,
                }
            }
        }

        impl<const W: $inner, const H: $inner> From<$name<W, H>> for $inner {
            fn from(val: $name<W, H>) -> Self {
                val.0 as $inner
            }
        }

        impl<const W: $inner, const H: $inner> From<&$name<W, H>> for $inner {
            fn from(val: &$name<W, H>) -> Self {
                val.0 as $inner
            }
        }

        impl<const W: $inner, const H: $inner> From<$name<W, H>> for usize {
            fn from(val: $name<W, H>) -> Self {
                val.0 as usize
            }
        }

        impl<const W: $inner, const H: $inner> From<&$name<W, H>> for usize {
            fn from(val: &$name<W, H>) -> Self {
                val.0 as usize
            }
        }
    };
}

point_absolute!(PointAbsolute64, u64);
point_absolute!(PointAbsolute32, u32);
point_absolute!(PointAbsolute16, u16);
point_absolute!(PointAbsolute8, u8);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point_absolute::*;
    use itertools::Itertools;

    #[test]
    fn test_points_by_row() {
        let str = PointAbsolute8::<3, 4>::points_by_row().join("|");

        assert_eq!(
            "(0,0)|(1,0)|(2,0)|(0,1)|(1,1)|(2,1)|(0,2)|(1,2)|(2,2)|(0,3)|(1,3)|(2,3)",
            str
        )
    }

    #[test]
    fn test_points_by_col() {
        let str = PointAbsolute8::<3, 4>::points_by_col().join("|");

        assert_eq!(
            "(0,0)|(0,1)|(0,2)|(0,3)|(1,0)|(1,1)|(1,2)|(1,3)|(2,0)|(2,1)|(2,2)|(2,3)",
            str
        )
    }
}
