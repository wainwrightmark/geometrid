use core::ops::Add;

use crate::corner::Corner;
use crate::dynamic_vector::DynamicVector;
use crate::flippable::{Flippable, FlipAxes};
use crate::inners::SignedInner;
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
pub struct DynamicVertex<V : DynamicVector>(pub V) where <V as Primitive>::Inner: SignedInner,;

impl<V : DynamicVector> Primitive for DynamicVertex<V> where <V as Primitive>::Inner: SignedInner,{
    type Inner = V::Inner;

    const CENTER: Self = Self(V::CENTER) ;

    fn x(&self) -> Self::Inner {
        self.0.x()
    }

    fn y(&self) -> Self::Inner {
        self.0.y()
    }
}

impl<V : DynamicVector> Zero for DynamicVertex<V> where <V as Primitive>::Inner: SignedInner,{
    fn zero() -> Self {
        Self(V::ZERO)
    }

    fn is_zero(&self) -> bool {
        self.0 == V::ZERO
    }
}

impl<V : DynamicVector> Add for DynamicVertex<V> where <V as Primitive>::Inner: SignedInner,{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<V : DynamicVector> Neg for DynamicVertex<V> where <V as Primitive>::Inner: SignedInner,{
    type Output= Self;

    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}

impl<V : DynamicVector> Mul<isize> for DynamicVertex<V> where <V as Primitive>::Inner: SignedInner,{
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0.mul(rhs))
    }
}

impl<V : DynamicVector> Mul<usize> for DynamicVertex<V> where <V as Primitive>::Inner: SignedInner,{
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0.mul(rhs))
    }
}

impl<V : DynamicVector> Flippable for DynamicVertex<V> where <V as Primitive>::Inner: SignedInner,{
    fn flip(&mut self, axes: FlipAxes) {
        self.0.flip(axes)
    }
}

impl<V : DynamicVector> Rotatable for DynamicVertex<V> where <V as Primitive>::Inner: SignedInner,{
    fn rotate(&mut self, quarter_turns: QuarterTurns) {
        self.0.rotate(quarter_turns)
    }
}

impl<V : DynamicVector> HasLocation for DynamicVertex<V> where <V as Primitive>::Inner: SignedInner,{
    fn location(&self, scale: f32) -> Location {
        let x = (<<V as Primitive>::Inner as Into<f32> >::into(self.x())) * scale;
        let y = (<<V as Primitive>::Inner as Into<f32> >::into(self.y())) * scale;
        Location { x, y }
    }
}



impl<V : DynamicVector> DynamicPrimitive for DynamicVertex<V> where <V as Primitive>::Inner: SignedInner,{
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

impl<V : DynamicVector> UniformPrimitive for DynamicVertex<V> where <V as Primitive>::Inner: SignedInner,{

}

impl <V : DynamicVector> Vertex for DynamicVertex<V> where <V as Primitive>::Inner: SignedInner,{
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