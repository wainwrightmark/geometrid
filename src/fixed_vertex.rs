use core::fmt::Debug;
use core::fmt::Display;
use core::ops::Add;
use core::ops::Mul;
use core::ops::Neg;
use num_traits::One;
use num_traits::Zero;

use crate::corner::Corner;
use crate::dynamic_vector::DynamicVector;
use crate::dynamic_vertex::DynamicVertex;
use crate::fixed_tile::*;
use crate::flippable::FlipAxes;
use crate::flippable::Flippable;
use crate::inners::PrimitiveInner;
use crate::inners::UnsignedInner;
use crate::location::HasLocation;
use crate::location::Location;
use crate::prelude::*;
use crate::fixed_tile::*;
use crate::primitives::DynamicPrimitive;
use crate::primitives::FixedPrimitive;
use crate::primitives::FixedUniformPrimitive;
use crate::primitives::Primitive;
use crate::primitives::Tile;
use crate::primitives::UniformPrimitive;
use crate::primitives::Vertex;
use crate::rotatable::QuarterTurns;
use crate::rotatable::Rotatable;
use crate::side::Orientation;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter};

macro_rules! fixed_vertex {
    ($name:ident, $inner:ty, $tile:ident) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct $name<const WIDTH: $inner, const HEIGHT: $inner>($inner);

        impl<const WIDTH: $inner, const HEIGHT: $inner> Display for $name<WIDTH, HEIGHT> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "({},{})", self.x(), self.y())
            }
        }

        impl<const WIDTH: $inner, const HEIGHT: $inner> $name<WIDTH, HEIGHT> {
            #[must_use]
            pub const fn new_const<const X: $inner, const Y: $inner>() -> Self {
                Self::new_unchecked(X, Y)
            }

            #[must_use]
            #[inline]
            pub(crate) const fn new_unchecked(x: $inner, y: $inner) -> Self {
                debug_assert!(x <= WIDTH);
                debug_assert!(y <= HEIGHT);
                debug_assert!(Self::COUNT.saturating_sub(1) <= <$inner>::MAX);
                Self((x + ((WIDTH + 1) * y)))
            }
        }

        impl<const WIDTH: $inner, const HEIGHT: $inner> Into<$inner> for $name<WIDTH, HEIGHT> {
            fn into(self) -> $inner {
                self.0
            }
        }

        impl<const WIDTH: $inner, const HEIGHT: $inner> Into<usize> for $name<WIDTH, HEIGHT> {
            fn into(self) -> usize {
                self.0.into()
            }
        }

        impl<const WIDTH: $inner, const HEIGHT: $inner> FixedPrimitive for $name<WIDTH, HEIGHT> {
            const COUNT: Self::Inner = (WIDTH + 1) * (HEIGHT + 1);

            const TILE_COUNT: Self::Inner = WIDTH * HEIGHT;

            fn try_from_inner(inner: Self::Inner) -> Option<Self> {
                if inner <= Self::BOTTOM_RIGHT.0 {
                    Some(Self(inner))
                } else {
                    None
                }
            }

            fn try_from_usize(inner: usize) -> Option<Self> {
                if inner <= Self::BOTTOM_RIGHT.0.into() {
                    Some(Self(inner as $inner))
                } else {
                    None
                }
            }

            fn try_with_x(&self, x: Self::Inner) -> Option<Self> {
                if x <= WIDTH {
                    Some(Self::new_unchecked(x, self.y()))
                } else {
                    None
                }
            }

            fn try_with_y(&self, y: Self::Inner) -> Option<Self> {
                if y <= HEIGHT {
                    Some(Self::new_unchecked(self.x(), y))
                } else {
                    None
                }
            }
        }

        impl<const WIDTH: $inner, const HEIGHT: $inner> FixedUniformPrimitive
            for $name<WIDTH, HEIGHT>
        {
            const TOP_LEFT: Self = Self(0);

            const TOP_RIGHT: Self = Self::new_unchecked(WIDTH - 1, 0);

            const BOTTOM_LEFT: Self = Self::new_unchecked(0, HEIGHT - 1);

            const BOTTOM_RIGHT: Self = Self(Self::COUNT - 1);

            fn try_new(x: Self::Inner, y: Self::Inner) -> Option<Self> {
                let i1 = y.checked_mul(WIDTH + 1)?;
                let i2 = i1.checked_add(x)?;

                Self::try_from_inner(i2)
            }
        }

        impl<const WIDTH: $inner, const HEIGHT: $inner> HasLocation for $name<WIDTH, HEIGHT> {
            fn location(&self, scale: f32) -> Location {
                let x = scale * f32::from(self.x());
                let y = scale * f32::from(self.y());

                Location { x, y }
            }
        }

        impl<const WIDTH: $inner, const HEIGHT: $inner> Flippable for $name<WIDTH, HEIGHT> {
            fn flip(&mut self, axes: FlipAxes) {
                match axes {
                    FlipAxes::None => {}
                    FlipAxes::Horizontal => *self = Self::new_unchecked(WIDTH - self.x(), self.y()),
                    FlipAxes::Vertical => *self = Self::new_unchecked(self.x(), HEIGHT - self.y()),
                    FlipAxes::Both => {
                        *self = Self::new_unchecked(WIDTH - self.x(), HEIGHT - self.y())
                    }
                }
            }
        }

        impl<const L: $inner> Rotatable for $name<L, L> {
            fn rotate(&mut self, quarter_turns: QuarterTurns) {
                *self = match quarter_turns {
                    crate::rotatable::QuarterTurns::Zero => {
                        return;
                    }
                    crate::rotatable::QuarterTurns::One => {
                        Self::new_unchecked(L - self.x(), self.y())
                    }
                    crate::rotatable::QuarterTurns::Two => {
                        Self::new_unchecked(L - self.y(), L - self.x())
                    }
                    crate::rotatable::QuarterTurns::Three => {
                        Self::new_unchecked(self.x(), L - self.y())
                    }
                }
            }
        }

        impl<const WIDTH: $inner, const HEIGHT: $inner> Primitive for $name<WIDTH, HEIGHT> {
            type Inner = $inner;

            const CENTER: Self = Self::new_unchecked(WIDTH / 2, HEIGHT / 2);

            fn x(&self) -> Self::Inner {
                self.0 % (WIDTH + 1)
            }

            fn y(&self) -> Self::Inner {
                self.0 / (WIDTH + 1)
            }
        }

        impl<const WIDTH: $inner, const HEIGHT: $inner> UniformPrimitive for $name<WIDTH, HEIGHT> {}

        impl<const WIDTH: $inner, const HEIGHT: $inner> Vertex for $name<WIDTH, HEIGHT> {
            type Tile = $tile<WIDTH, HEIGHT>;

            fn try_get_tile(&self, corner: Corner) -> Option<Self::Tile> {
                let (x, y) = match corner {
                    Corner::TopLeft => {
                        let x = self.x().checked_sub(1)?;
                        let y = self.y().checked_sub(1)?;
                        (x, y)
                    }
                    Corner::TopRight => {
                        let x = 0;
                        let y = self.y().checked_sub(1)?;
                        (x, y)
                    }
                    Corner::BottomLeft => {
                        let x = self.x().checked_sub(1)?;
                        let y = 0;
                        (x, y)
                    }
                    Corner::BottomRight => {
                        let x = 0;
                        let y = 0;
                        (x, y)
                    }
                };
                Self::Tile::try_new(x, y)
            }
        }
    };
}

fixed_vertex!(FixedVertex8, u8, FixedTile8);
fixed_vertex!(FixedVertex16, u16, FixedTile16);