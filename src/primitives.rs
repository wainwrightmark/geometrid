use num_traits::One;
use num_traits::Zero;
use core::fmt::Debug;
use core::ops::Add;
use core::ops::Mul;
use core::ops::Neg;

use crate::corner::Corner;
use crate::flippable::Flippable;
use crate::inners::PrimitiveInner;
use crate::inners::SignedInner;
use crate::inners::UnsignedInner;
use crate::location::HasLocation;
use crate::rotatable::Rotatable;
use crate::side::Orientation;

pub trait Primitive:
    Copy
    + Clone
    + Debug
    + Eq
    + PartialEq
    + Ord
    + PartialOrd
    + core::hash::Hash
    + HasLocation
    + Flippable
{
    type Inner: PrimitiveInner;

    /// The primitive at the center of the grid.
    /// If there is no central element, the one just above or to the left will be used
    const CENTER: Self;

    fn x(&self) -> Self::Inner;
    fn y(&self) -> Self::Inner;
}

/// A primitive in a fixed size grid
pub trait FixedPrimitive: Primitive + Into<<Self as Primitive>::Inner> + Into<usize>
where <Self as Primitive>::Inner: UnsignedInner,
{
    /// The total number of primitives of this type in the grid
    const COUNT: Self::Inner;

    /// The total number of tiles in the grid
    const TILE_COUNT: Self::Inner;

    /// Create a new primitive if it is in bounds
    fn try_from_inner(inner: Self::Inner) -> Option<Self>;

    /// Create a new primitive if it is in bounds
    fn try_from_usize(inner: usize) -> Option<Self>;

    /// Clone this and set the x value
    fn try_with_x(&self, x: Self::Inner) -> Option<Self>;
    /// Clone this and set the y value
    fn try_with_y(&self, y: Self::Inner) -> Option<Self>;
}

/// A primitive in a dynamically sized grid, centred at (0,0)
pub trait DynamicPrimitive:
    Primitive + Rotatable + Neg<Output = Self> + Mul<usize, Output = Self> + Mul<isize, Output = Self> + Add<Output = Self>
    where
    <Self as Primitive>::Inner: SignedInner,
{
    /// Clone this and set the x value
    fn with_x(&self, x: Self::Inner) -> Self;
    /// Clone this and set the y value
    fn with_y(&self, y: Self::Inner) -> Self;

    fn new(x: Self::Inner, y: Self::Inner)-> Self;
}

pub trait Tile: UniformPrimitive {
    type Vertex: Vertex;
    fn get_vertex(&self, corner: Corner) -> Self::Vertex;

    //TODO get_edge
}

pub trait Vertex: UniformPrimitive {
    type Tile: Tile;
    fn try_get_tile(&self, corner: Corner) -> Option<Self::Tile>;

    //TODO get_edge
}

pub trait Edge: Primitive {
    type Vertex: Vertex;
    type Tile: Tile;

    //TODO get_tile
    //TODO get_vertex
    fn orientation(&self) -> Orientation;
}

pub trait FixedUniformPrimitive: UniformPrimitive + FixedPrimitive
where
    <Self as Primitive>::Inner: UnsignedInner,
{
    const TOP_LEFT: Self;
    const TOP_RIGHT: Self;
    const BOTTOM_LEFT: Self;
    const BOTTOM_RIGHT: Self;

    /// Create a new primitive, if it is in bounds
    #[must_use]
    fn try_new(x: Self::Inner, y: Self::Inner) -> Option<Self>;

    /// Does this primitive appear at the corner of the grid
    fn is_at_corner(&self) -> bool {
        let corners = [
            Self::TOP_LEFT,
            Self::TOP_RIGHT,
            Self::BOTTOM_LEFT,
            Self::BOTTOM_RIGHT,
        ];
        corners.contains(self)
    }

    /// Does this primitive appear at the edge of the grid
    fn is_at_edge(&self) -> bool {
        self.x().is_zero()
            || self.y().is_zero()
            || self.x() == Self::TOP_RIGHT.x()
            || self.y() == Self::BOTTOM_LEFT.y()
    }
}

/// A tile or a vertex
pub trait UniformPrimitive: Primitive {
    /// The number of horizontal or vertical steps to reach the other primitive
    fn manhattan_distance(&self, other: &Self) -> <Self::Inner as PrimitiveInner>::Unsigned {
        self.x().abs_diff(other.x()) + self.y().abs_diff(other.y())
    }

    /// True if two primitives are orthogonal or diagonal, but not identical
    fn is_adjacent(self, other: Self) -> bool {
        let col_diff = self.x().abs_diff(other.x());
        if col_diff.is_zero() {
            self.y().abs_diff(other.y()).is_one()
        } else if col_diff.is_one() {
            self.y().abs_diff(other.y())
                <= <<Self::Inner as PrimitiveInner>::Unsigned as One>::one()
        } else {
            false
        }
    }

    /// True if two coordinates are orthogonal
    fn is_orthogonal(self, other: Self) -> bool {
        let col_diff = self.x().abs_diff(other.x());
        if col_diff.is_zero() {
            self.y().abs_diff(other.y()).is_one()
        } else if col_diff.is_one() {
            self.y().abs_diff(other.y()).is_zero()
        } else {
            false
        }
    }
}