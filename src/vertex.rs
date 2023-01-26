use core::{fmt, ops::Add};

use itertools::Itertools;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::prelude::*;
pub trait Vertex: UniformPrimitive + HasLocation + Flippable {}

macro_rules! vertex {
    ($name:ident, $inner:ty) => {
        #[must_use]
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)] //TODO make inner type generic
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct $name<const WIDTH: $inner, const HEIGHT: $inner>($inner);

        impl<const W: $inner, const H: $inner> fmt::Display for $name<W, H> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({},{})", self.col(), self.row())
            }
        }

        impl<const COLS: $inner, const ROWS: $inner> $name<COLS, ROWS> {
            #[must_use]
            pub const fn new_const<const X: $inner, const Y: $inner>() -> Self {
                Self::new_unchecked(X, Y)
            }

            #[must_use]
            #[inline]
            pub(crate) const fn new_unchecked(x: $inner, y: $inner) -> Self {
                debug_assert!(x <= Self::COLUMNS);
                debug_assert!(y <= Self::ROWS);
                debug_assert!(Self::COUNT.saturating_sub(1) <= <$inner>::MAX);
                Self((x + ((Self::COLUMNS + 1) * y)))
            }
        }

        impl<const COLS: $inner, const ROWS: $inner> Vertex for $name<COLS, ROWS> {}

        impl<const COLS: $inner, const ROWS: $inner> GridAligned for $name<COLS, ROWS> {
            type Inner = $inner;

            const COLUMNS: Self::Inner = COLS;

            const ROWS: Self::Inner = ROWS;

            const TOTAL_TILES: Self::Inner = COLS * ROWS;
        }

        impl<const COLS: $inner, const ROWS: $inner> Primitive for $name<COLS, ROWS> {
            const COUNT: <Self as GridAligned>::Inner = (COLS + 1) * (ROWS + 1);

            const FIRST: Self = Self(0);

            const LAST: Self = Self(Self::COUNT - 1);

            fn inner(&self) -> <Self as GridAligned>::Inner {
                self.0
            }

            fn try_from_inner(inner: <Self as GridAligned>::Inner) -> Option<Self> {
                if inner <= Self::LAST.inner() {
                    Some(Self(inner))
                } else {
                    None
                }
            }

            fn col(&self) -> <Self as GridAligned>::Inner {
                self.0 % (Self::COLUMNS + 1)
            }

            fn row(&self) -> <Self as GridAligned>::Inner {
                self.0 / (Self::COLUMNS + 1)
            }
        }

        impl<const COLS: $inner, const ROWS: $inner> UniformPrimitive for $name<COLS, ROWS> {
            const CENTER: Self = Self::new_unchecked(COLS / 2, ROWS / 2);

            const TOP_RIGHT: Self = Self::new_unchecked(COLS - 1, 0);
            const BOTTOM_LEFT: Self = Self::new_unchecked(0, ROWS - 1);

            const MAX_COL: <Self as GridAligned>::Inner = COLS;

            const MAX_ROW: <Self as GridAligned>::Inner = ROWS;

            fn try_new(
                col: <Self as GridAligned>::Inner,
                row: <Self as GridAligned>::Inner,
            ) -> Option<Self> {
                let i1 = row.checked_mul(COLS + 1)?;
                let i2 = i1.checked_add(col)?;

                Self::try_from_inner(i2)
            }
        }

        impl<const W: $inner, const H: $inner> Flippable for $name<W, H> {
            fn flip(&mut self, axes: FlipAxes) {
                match axes {
                    FlipAxes::None => {}
                    FlipAxes::Horizontal => {
                        *self = Self::try_new(Self::MAX_COL - self.col(), self.row()).unwrap()
                    }
                    FlipAxes::Vertical => {
                        *self = Self::try_new(self.col(), Self::MAX_ROW - self.row()).unwrap()
                    }
                    FlipAxes::Both => {
                        *self =
                            Self::try_new(Self::MAX_COL - self.col(), Self::MAX_ROW - self.row())
                                .unwrap()
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
                        Self::new_unchecked(L - self.row(), self.col())
                    }
                    crate::rotatable::QuarterTurns::Two => {
                        Self::new_unchecked(L - self.col(), L - self.row())
                    }
                    crate::rotatable::QuarterTurns::Three => {
                        Self::new_unchecked(self.row(), L - self.col())
                    }
                }
            }
        }

        impl<const W: $inner, const H: $inner> HasLocation for $name<W, H> {
            #[must_use]
            fn location(&self, scale: f32) -> Location {
                let x = scale * (self.col() as f32);
                let y = scale * (self.row() as f32);

                Location { x, y }
            }
        }

        impl<const W: $inner, const H: $inner> From<$name<W, H>> for $inner {
            fn from(val: $name<W, H>) -> Self {
                val.0
            }
        }

        impl<const W: $inner, const H: $inner> From<&$name<W, H>> for $inner {
            fn from(val: &$name<W, H>) -> Self {
                val.0
            }
        }

        impl<const W: $inner, const H: $inner> From<$name<W, H>> for usize {
            fn from(val: $name<W, H>) -> Self {
                val.0.into()
            }
        }

        impl<const W: $inner, const H: $inner> From<&$name<W, H>> for usize {
            fn from(val: &$name<W, H>) -> Self {
                val.0.into()
            }
        }
    };
}

vertex!(Vertex16, u16);
vertex!(Vertex8, u8);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vertex::*;
    use itertools::Itertools;
    #[cfg(feature = "serde")]
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_iter_by_row() {
        let str = Vertex8::<2, 3>::iter_by_row().join("|");

        assert_eq!(
            "(0,0)|(1,0)|(2,0)|(0,1)|(1,1)|(2,1)|(0,2)|(1,2)|(2,2)|(0,3)|(1,3)|(2,3)",
            str
        )
    }

    #[test]
    fn test_from() {
        for vertex in Vertex8::<3, 4>::iter_by_row() {
            let n = Vertex8::try_new(vertex.col(), vertex.row()).unwrap();
            assert_eq!(vertex, n)
        }
    }

    #[test]
    fn test_flip_vertical() {
        let str = Vertex8::<2, 2>::iter_by_row()
            .map(|x| {
                x.flipped(FlipAxes::Vertical)
            })
            .join("|");

        assert_eq!("(0,2)|(1,2)|(2,2)|(0,1)|(1,1)|(2,1)|(0,0)|(1,0)|(2,0)", str)
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        let tile: Vertex8<3, 3> = Vertex8(2);

        assert_tokens(
            &tile,
            &[Token::NewtypeStruct { name: "Vertex8" }, Token::U8(2)],
        );
    }
}
