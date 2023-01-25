use core::{marker::PhantomData, ops::Mul};

use num_traits::{One, PrimInt, Unsigned, Zero, CheckedMul, CheckedAdd};
use strum::{Display, EnumCount};

use crate::location::Location;

pub trait PrimitiveInner: PrimInt + Unsigned + TryFrom<usize> + Into<usize>
+ Zero + One + Clone + Copy + CheckedMul<Output = Self> + CheckedAdd {
    fn abs_diff(self, other: Self) -> Self;
}

impl PrimitiveInner for u8 {
    fn abs_diff(self, other: Self) -> Self {
        self.abs_diff(other)
    }
}
impl PrimitiveInner for u16 {
    fn abs_diff(self, other: Self) -> Self {
        self.abs_diff(other)
    }
}
// impl PrimitiveInner for u32 {
//     fn abs_diff(self, other: Self) -> Self {
//         self.abs_diff(other)
//     }
// }

/// A primitive of square geometry
pub trait Primitive: GridAligned + Clone {
    #[must_use]
    /// The total number of this primitive
    const COUNT: <Self as GridAligned>::Inner;

    const FIRST: Self;
    const LAST: Self;

    #[must_use]
    fn inner(&self) -> <Self as GridAligned>::Inner;

    #[must_use]
    /// Create a new primitive, if it is in bounds
    fn try_from_inner(inner: <Self as GridAligned>::Inner) -> Option<Self>;

    #[must_use]
    /// Create a new primitive, if it is in bounds
    fn try_from_usize(inner: usize) -> Option<Self> {
        let inner = <Self as GridAligned>::Inner::try_from(inner).ok()?;
        Self::try_from_inner(inner)
    }

    #[must_use]
    /// Get the next primitive in sequence, if it is in bounds
    fn try_next(&self) -> Option<Self> {
        Self::try_from_inner(self.inner() + <Self as GridAligned>::Inner::one())
    }

    #[must_use]
    /// Get the column of this primitive
    fn col(&self) -> <Self as GridAligned>::Inner;

    #[must_use]
    /// Get the row of this primitive
    fn row(&self) -> <Self as GridAligned>::Inner;

    #[must_use]
    /// Iterate all points by row
    fn iter_by_row() -> PrimitiveIter<Self> {
        PrimitiveIter(Some(Self::FIRST))
    }
}

/// A primitive of square geometry which has only one form.
/// Essentially tiles or vertices, but not edges, which can be either vertical or horizontal
pub trait UniformPrimitive: Primitive {
    /// The primitive at the center of the grid.
    /// If there is no central element, the one just above or to the left will be used
    const CENTER: Self;

    const TOP_LEFT: Self = Self::FIRST;
    const TOP_RIGHT: Self;
    const BOTTOM_LEFT: Self;
    const BOTTOM_RIGHT: Self = Self::LAST;

    /// The highest valid column index for this primitive
    const MAX_COL: <Self as GridAligned>::Inner;
    /// The highest valid row index for this primitive
    const MAX_ROW: <Self as GridAligned>::Inner;

    /// Create a new primitive, if it is in bounds
    #[must_use]
    fn try_new(
        col: <Self as GridAligned>::Inner,
        row: <Self as GridAligned>::Inner,
    ) -> Option<Self>;

    /// Does this primitive appear at the corner of the grid
    fn is_at_corner(&self) -> bool {
        [
            Self::TOP_LEFT.inner(),
            Self::TOP_RIGHT.inner(),
            Self::BOTTOM_LEFT.inner(),
            Self::BOTTOM_RIGHT.inner(),
        ]
        .contains(&self.inner())
    }

    /// Does this primitive appear at the edge of the grid
    fn is_at_edge(&self) -> bool {
        self.col().is_zero()
            || self.row().is_zero()
            || self.col() == Self::MAX_COL
            || self.row() == Self::MAX_ROW
    }

    /// The number of horizontal or vertical steps to reach the other primitive
    fn manhattan_distance(&self, other: &Self) -> <Self as GridAligned>::Inner {
        self.col().abs_diff(other.col()) + self.row().abs_diff(other.row())
    }

    /// True if two primitives are orthogonal or diagonal, but not identical
    fn is_adjacent(self, other: Self) -> bool {
        let col_diff = self.col().abs_diff(other.col());
        if col_diff.is_zero() {
            self.row().abs_diff(other.row()).is_one()
        } else if col_diff.is_one() {
            self.row().abs_diff(other.row()) <= <Self as GridAligned>::Inner::one()
        } else {
            false
        }
    }

    /// True if two coordinates are orthogonal
    fn is_orthogonal(self, other: Self) -> bool {
        let col_diff = self.col().abs_diff(other.col());
        if col_diff.is_zero() {
            self.row().abs_diff(other.row()).is_one()
        } else if col_diff.is_one() {
            self.row().abs_diff(other.row()).is_zero()
        } else {
            false
        }
    }
}

/// Anything that is associated with a fixed size square grid
pub trait GridAligned: Sized {
    type Inner: PrimitiveInner;
    /// The total number of columns in the grid
    const COLUMNS: Self::Inner;
    /// The total number of rows in the grid
    const ROWS: Self::Inner;
    /// The total number of tiles in the grid
    const TOTAL_TILES: Self::Inner;
}

pub struct PrimitiveIter<P: Primitive>(Option<P>);

impl<P: Primitive> Iterator for PrimitiveIter<P> {
    type Item = P;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.clone() {
            Some(next) => {
                self.0 = next.try_next();
                Some(next)
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.0 {
            Some(inner) => {
                let size = (P::LAST.inner().into() - inner.inner().into()) + 1usize;
                (size, Some(size))
            },
            None => (0, Some(0)),
        }
    }
}
