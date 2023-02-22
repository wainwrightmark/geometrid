use core::ops::Add;

use crate::corner::Corner;
use crate::dynamic_vector::DynamicVector;
use crate::flippable::{Flippable, FlipAxes};
use crate::location::{HasLocation, Location};
use crate::primitives::{Primitive, DynamicPrimitive, UniformPrimitive, Vertex};
use crate::rotatable::{Rotatable, QuarterTurns};
use crate::vector::*;
use crate::{prelude::*, dynamic_tile::DynamicTile};
use crate::*;

use num_traits::Zero;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter, Display};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DynamicVertex<V : DynamicVector>(pub V);

impl<V : DynamicVector> Primitive for DynamicVertex<V>{
    type Inner = V::Inner;

    const CENTER: Self = Self(V::CENTER) ;

    fn x(&self) -> Self::Inner {
        self.0.x()
    }

    fn y(&self) -> Self::Inner {
        self.0.y()
    }
}

impl<V : DynamicVector> Zero for DynamicVertex<V>{
    fn zero() -> Self {
        Self(V::ZERO)
    }

    fn is_zero(&self) -> bool {
        self.0 == V::ZERO
    }
}

impl<V : DynamicVector> Add for DynamicVertex<V>{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<V : DynamicVector> Neg for DynamicVertex<V>{
    type Output= Self;

    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}

impl<V : DynamicVector> Mul<isize> for DynamicVertex<V>{
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0.mul(rhs))
    }
}

impl<V : DynamicVector> Mul<usize> for DynamicVertex<V>{
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0.mul(rhs))
    }
}

impl<V : DynamicVector> Flippable for DynamicVertex<V>{
    fn flip(&mut self, axes: FlipAxes) {
        self.0.flip(axes)
    }
}

impl<V : DynamicVector> Rotatable for DynamicVertex<V>{
    fn rotate(&mut self, quarter_turns: QuarterTurns) {
        self.0.rotate(quarter_turns)
    }
}

impl<V : DynamicVector> HasLocation for DynamicVertex<V>{
    fn location(&self, scale: f32) -> Location {
        let x = (self.x().into() + 0.5) * scale;
        let y = (self.y().into() + 0.5) * scale;
        Location { x, y }
    }
}



impl<V : DynamicVector> DynamicPrimitive for DynamicVertex<V>{
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

impl<V : DynamicVector> UniformPrimitive for DynamicVertex<V>{

}

impl <V : DynamicVector> Vertex for DynamicVertex<V>{
    type Tile = DynamicTile<V>;

    fn try_get_tile(&self, corner: Corner) -> Option<Self::Tile> {
        let offset = match corner{
            Corner::TopLeft => V::UP_LEFT,
            Corner::TopRight => V::UP_RIGHT,
            Corner::BottomLeft => V::DOWN_LEFT,
            Corner::BottomRight => V::DOWN_RIGHT,
        };

        Some(DynamicTile(self.0 + offset))
    }


}