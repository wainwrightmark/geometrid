use core::fmt::Debug;
use core::ops::Add;
use core::ops::Mul;
use core::ops::Neg;
use num_traits::One;
use num_traits::Zero;

use crate::corner::Corner;
use crate::dynamic_vector::DynamicVector;
use crate::dynamic_vertex::DynamicVertex;
use crate::fixed_vertex::*;
use crate::flippable::FlipAxes;
use crate::flippable::Flippable;
use crate::inners::PrimitiveInner;
use crate::inners::UnsignedInner;
use crate::location::HasLocation;
use crate::location::Location;
use crate::prelude::*;
use crate::primitives::DynamicPrimitive;
use crate::primitives::FixedPrimitive;
use crate::primitives::FixedUniformPrimitive;
use crate::primitives::Primitive;
use crate::primitives::Tile;
use crate::primitives::UniformPrimitive;
use crate::rotatable::QuarterTurns;
use crate::rotatable::Rotatable;
use crate::side::Orientation;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter};

macro_rules! fixed_tile {
    ($name:ident, $inner:ty, $vertex:ident) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct $name<const WIDTH: $inner, const HEIGHT: $inner>($inner);

        impl<const WIDTH: $inner, const HEIGHT: $inner> $name<WIDTH, HEIGHT> {
            #[must_use]
            pub const fn new_const<const X: $inner, const Y: $inner>() -> Self {
                Self::new_unchecked(X, Y)
            }

            #[must_use]
            #[inline]
            pub(crate) const fn new_unchecked(x: $inner, y: $inner) -> Self {
                debug_assert!(x < WIDTH);
                debug_assert!(y < HEIGHT);
                debug_assert!(Self::COUNT.saturating_sub(1) <= <$inner>::MAX);
                Self((x + (WIDTH * y)))
            }
        }

        impl<const W: $inner, const H: $inner> core::fmt::Display for $name<W, H> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "({},{})", self.x(), self.y())
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
            const COUNT: Self::Inner = WIDTH * HEIGHT;

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
                if x < WIDTH {
                    Some(Self::new_unchecked(x, self.y()))
                } else {
                    None
                }
            }

            fn try_with_y(&self, y: Self::Inner) -> Option<Self> {
                if y < HEIGHT {
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

            const BOTTOM_RIGHT: Self = Self::new_unchecked(WIDTH - 1, HEIGHT - 1);

            fn try_new(x: Self::Inner, y: Self::Inner) -> Option<Self> {
                let i1 = y.checked_mul(WIDTH)?;
                let i2 = i1.checked_add(x)?;
                Self::try_from_inner(i2)
            }
        }

        impl<const WIDTH: $inner, const HEIGHT: $inner> HasLocation for $name<WIDTH, HEIGHT> {
            fn location(&self, scale: f32) -> Location {
                let x = scale * (f32::from(self.x()) + 0.5);
                let y = scale * (f32::from(self.y()) + 0.5);

                Location { x, y }
            }
        }

        impl<const WIDTH: $inner, const HEIGHT: $inner> Flippable for $name<WIDTH, HEIGHT> {
            fn flip(&mut self, axes: FlipAxes) {
                match axes {
                    FlipAxes::None => {}
                    FlipAxes::Horizontal => {
                        *self = Self::try_new(WIDTH - 1 - self.x(), self.y()).unwrap()
                    }
                    FlipAxes::Vertical => {
                        *self = Self::try_new(self.x(), HEIGHT - 1 - self.y()).unwrap()
                    }
                    FlipAxes::Both => {
                        *self = Self::try_new(WIDTH - 1 - self.x(), HEIGHT - 1 - self.y()).unwrap()
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
                        Self::new_unchecked(L - 1 - self.y(), self.x())
                    }
                    crate::rotatable::QuarterTurns::Two => {
                        Self::new_unchecked(L - 1 - self.x(), L - 1 - self.y())
                    }
                    crate::rotatable::QuarterTurns::Three => {
                        Self::new_unchecked(self.y(), L - 1 - self.x())
                    }
                }
            }
        }

        impl<const WIDTH: $inner, const HEIGHT: $inner> Primitive for $name<WIDTH, HEIGHT> {
            type Inner = $inner;

            const CENTER: Self = Self::new_unchecked(WIDTH / 2, HEIGHT / 2);

            fn x(&self) -> Self::Inner {
                self.0 % WIDTH
            }

            fn y(&self) -> Self::Inner {
                self.0 / WIDTH
            }
        }

        impl<const WIDTH: $inner, const HEIGHT: $inner> UniformPrimitive for $name<WIDTH, HEIGHT> {}

        impl<const WIDTH: $inner, const HEIGHT: $inner> Tile for $name<WIDTH, HEIGHT> {
            type Vertex = $vertex<WIDTH, HEIGHT>;

            fn get_vertex(&self, corner: Corner) -> Self::Vertex {
                match corner {
                    Corner::TopLeft => Self::Vertex::try_new(self.x(), self.y()).unwrap(),
                    Corner::TopRight => Self::Vertex::try_new(self.x() + 1, self.y()).unwrap(),
                    Corner::BottomLeft => Self::Vertex::try_new(self.x(), self.y() + 1).unwrap(),
                    Corner::BottomRight => Self::Vertex::try_new(self.x() + 1, self.y()).unwrap(),
                }
            }
        }
    };
}

fixed_tile!(FixedTile8, u8, FixedVertex8);
fixed_tile!(FixedTile16, u16, FixedVertex16);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{prelude::*, flippable::CopyFlippable};
    use itertools::Itertools;
    #[cfg(feature = "serde")]
    use serde_test::{assert_tokens, Token};
    use strum::IntoEnumIterator;

    type Tile4 = FixedTile8::<4,4>;

    #[test]
    pub fn test_flips(){
        for x in 0..4{
            for y in 0..4{
                let tile = Tile4::new_unchecked(x, y);

                let flips = FlipAxes::iter().map(|axes| tile.flipped(axes)).collect_vec();

                println!("{flips:?}");
                assert!(flips.iter().all_unique()) ;
            }
        }
    }

    #[test]
    pub fn test_x_and_y(){
        for x in 0..4{
            for y in 0..4{
                let tile = Tile4::try_new(x, y).unwrap();
                let (x1,y1) = (tile.x(), tile.y());

                assert_eq!((x,y), (x1,y1))
            }
        }
    }
}
