use core::{iter, ops::Div};

use crate::{
     dynamic_vertex::DynamicVertex, flippable::Flippable,
    inners::{PrimitiveInner, SignedInner}, location::HasLocation, primitives::{Primitive, DynamicPrimitive}, rotatable::Rotatable,
    shape::Shape, dynamic_vector::DynamicVector,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rectangle<V: DynamicVector> where <V as Primitive>::Inner: SignedInner, {
    pub top_left: V,
    pub width: <<V as Primitive>::Inner as PrimitiveInner>::Unsigned,
    pub height: <<V as Primitive>::Inner as PrimitiveInner>::Unsigned,
}

impl<V: DynamicVector> HasLocation for Rectangle<V> where <V as Primitive>::Inner: SignedInner, {
    fn location(&self, scale: f32) -> crate::location::Location {
        let top_left = self.top_left.location(scale);

        let width = <<<V as Primitive>::Inner as PrimitiveInner>::Unsigned as Into<f32>>::into(
            self.width,
        );
        let height =
            <<<V as Primitive>::Inner as PrimitiveInner>::Unsigned as Into<f32>>::into(
                self.height,
            );
        let x = top_left.x + (width * 0.5 * scale);
        let y = top_left.y + (height * 0.5 * scale);
        crate::location::Location { x, y }
    }
}

impl<V: DynamicVector> Flippable for Rectangle<V> where <V as Primitive>::Inner: SignedInner, {
    fn flip(&mut self, axes: crate::flippable::FlipAxes) {
        //Do nothing
    }
}

impl<V: DynamicVector> Rotatable for Rectangle<V> where <V as Primitive>::Inner: SignedInner, {
    fn rotate(&mut self, quarter_turns: crate::rotatable::QuarterTurns) {
        use crate::rotatable::QuarterTurns::*;
        if quarter_turns == One || quarter_turns == Three{
            let x_diff = (self.width.to_signed() - self.height.to_signed());
            let x_diff = x_diff.div(<<<V as Primitive>::Inner as PrimitiveInner>::Signed>::TWO);
            let new_x= self.top_left.x() + <<V as Primitive>::Inner as SignedInner>::from_self(x_diff);
            
            
            let y_diff = (self.height.to_signed() - self.width.to_signed());
            let y_diff = y_diff.div(<<<V as Primitive>::Inner as PrimitiveInner>::Signed>::TWO);
            let new_y= self.top_left.y() + <<V as Primitive>::Inner as SignedInner>::from_self(y_diff);

            let new_top_left = V::new(new_x,new_y);            

            let new_rect = Rectangle{
                width: self.height,
                height: self.width,
                top_left: new_top_left
            };
            *self = new_rect
        }
    }
}

// impl<Vector: DynamicVector> Shape for Rectangle<Vector> {
//     type Vector = Vector;

//     type OutlineIter = core::array::IntoIter<DynamicVertex<Vector>, 4>;

//     type RectangleIter = iter::Once<Self>;

//     type TilesIter;

//     fn tiles_count(&self) -> usize {
//         let width =
//             <<<Vector as Primitive>::Inner as PrimitiveInner>::Absolute as Into<usize>>::into(
//                 self.width,
//             );
//         let height =
//             <<<Vector as Primitive>::Inner as PrimitiveInner>::Absolute as Into<usize>>::into(
//                 self.height,
//             );
//         width * height
//     }

//     fn tiles_iter(&self) -> Self::TilesIter {
//         todo!()
//     }

//     fn outline(&self) -> Self::OutlineIter {
//         let top_right = self.width
//         let arr = [DynamicVertex(self.top_left)];
//         arr.into_iter()
//     }

//     fn deconstruct(&self) -> Self::RectangleIter {
//         iter::once(self.clone())
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{prelude::*, vector::Vector};
    use itertools::Itertools;

    #[test]
    fn test_rotate() {
        let mut rect = Rectangle{
            top_left: Vector{x: 3i8, y: 7i8},
            width: 6u8,
            height: 2u8,
        };

        let r1 = Rectangle{
            top_left: Vector{x: 5i8, y: 5i8},
            width: 2u8,
            height: 6u8,
        };
        
        let r2 = Rectangle{
            top_left: Vector{x: 3i8, y: 7i8},
            width: 6u8,
            height: 2u8,
        };

        Rotatable::rotate(&mut rect, crate::rotatable::QuarterTurns::One);

        assert_eq!(rect, r1);

        Rotatable::rotate(&mut rect, crate::rotatable::QuarterTurns::One);
        assert_eq!(rect, r2);
    }
}