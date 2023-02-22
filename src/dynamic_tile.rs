use num_traits::One;
use num_traits::Zero;
use core::fmt::Debug;
use core::ops::Add;
use core::ops::Mul;
use core::ops::Neg;

use crate::corner::Corner;
use crate::dynamic_vector::DynamicVector;
use crate::dynamic_vertex::DynamicVertex;
use crate::flippable::FlipAxes;
use crate::flippable::Flippable;
use crate::inners::PrimitiveInner;
use crate::inners::UnsignedInner;
use crate::location::HasLocation;
use crate::location::Location;
use crate::primitives::DynamicPrimitive;
use crate::primitives::Primitive;
use crate::primitives::Tile;
use crate::primitives::UniformPrimitive;
use crate::rotatable::QuarterTurns;
use crate::rotatable::Rotatable;
use crate::side::Orientation;
use crate::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter, Display};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DynamicTile<V : DynamicVector>(pub V);

impl<V : DynamicVector> Primitive for DynamicTile<V>{
    type Inner = V::Inner;

    const CENTER: Self = Self(V::CENTER) ;

    fn x(&self) -> Self::Inner {
        self.0.x()
    }

    fn y(&self) -> Self::Inner {
        self.0.y()
    }
}

impl<V : DynamicVector> Zero for DynamicTile<V>{
    fn zero() -> Self {
        Self(V::ZERO)
    }

    fn is_zero(&self) -> bool {
        self.0 == V::ZERO
    }
}

impl<V : DynamicVector> Add for DynamicTile<V>{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<V : DynamicVector> Neg for DynamicTile<V>{
    type Output= Self;

    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}

impl<V : DynamicVector> Mul<isize> for DynamicTile<V>{
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0.mul(rhs))
    }
}

impl<V : DynamicVector> Mul<usize> for DynamicTile<V>{
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0.mul(rhs))
    }
}

impl<V : DynamicVector> Flippable for DynamicTile<V>{
    fn flip(&mut self, axes: FlipAxes) {
        self.0.flip(axes)
    }
}

impl<V : DynamicVector> Rotatable for DynamicTile<V>{
    fn rotate(&mut self, quarter_turns: QuarterTurns) {
        self.0.rotate(quarter_turns)
    }
}

impl<V : DynamicVector> HasLocation for DynamicTile<V>{
    fn location(&self, scale: f32) -> Location {
        let x = (self.x().into() + 0.5) * scale;
        let y = (self.y().into() + 0.5) * scale;
        Location { x, y }
    }
}



impl<V : DynamicVector> DynamicPrimitive for DynamicTile<V>{
    fn with_x(&self, x: Self::Inner) -> Self {
        Self(self.0.with_x(x))
    }

    fn with_y(&self, y: Self::Inner) -> Self {
        Self(self.0.with_y(y))
    }

    fn new(x: Self::Inner, y: Self::Inner)-> Self {
        Self(V::new(x, y))
    }


}

impl<V : DynamicVector> UniformPrimitive for DynamicTile<V>{

}

impl <V : DynamicVector> Tile for DynamicTile<V>{
    type Vertex = DynamicVertex<V>;

    fn get_vertex(&self, corner: Corner) -> Self::Vertex {
        let offset = match corner{
            Corner::TopLeft => V::UP_LEFT,
            Corner::TopRight => V::UP_RIGHT,
            Corner::BottomLeft => V::DOWN_LEFT,
            Corner::BottomRight => V::DOWN_RIGHT,
        };

        DynamicVertex(self.0 + offset)
    }
}