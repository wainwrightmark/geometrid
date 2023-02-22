use num_traits::CheckedAdd;
use num_traits::One;
use num_traits::Zero;
use core::fmt::Debug;
use core::ops::Mul;
use core::ops::Neg;

use crate::corner::Corner;
use crate::flippable::Flippable;
use crate::inners::PrimitiveInner;
use crate::inners::UnsignedInner;
use crate::location::HasLocation;
use crate::primitives::*;
use crate::rotatable::Rotatable;
use crate::side::Orientation;
pub struct PrimitiveIter<P: FixedPrimitive>(Option<P>)
where
    <P as Primitive>::Inner: UnsignedInner;

impl<P: FixedPrimitive> Iterator for PrimitiveIter<P>
where
    <P as Primitive>::Inner: UnsignedInner,
{
    type Item = P;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.clone() {
            Some(next) => {
                let one = <<P as Primitive>::Inner as One>::one();
                let next_next =
                    <<P as Primitive>::Inner as CheckedAdd>::checked_add(&next.into(), &one);
                self.0 = next_next.and_then(|i| P::try_from_inner(i));
                Some(next)
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.0 {
            Some(inner) => {
                let count: usize = P::COUNT.into();
                let index: <P as Primitive>::Inner = (*inner).into();
                let index: usize = index.into();
                let size = (count - index) + 1usize;
                (size, Some(size))
            }
            None => (0, Some(0)),
        }
    }
}
