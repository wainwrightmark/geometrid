use core::{iter, ops::Div};

use crate::{
    dynamic_vector::DynamicVector, dynamic_vertex::DynamicVertex, flippable::Flippable,
    inners::PrimitiveInner, location::HasLocation, primitives::Primitive, rotatable::Rotatable,
    shape::Shape,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rectangle<Vector: DynamicVector> {
    pub top_left: Vector,
    pub width: <<Vector as Primitive>::Inner as PrimitiveInner>::Absolute,
    pub height: <<Vector as Primitive>::Inner as PrimitiveInner>::Absolute,
}

impl<Vector: DynamicVector> HasLocation for Rectangle<Vector> {
    fn location(&self, scale: f32) -> crate::location::Location {
        let top_left = self.top_left.location(scale);

        let width = <<<Vector as Primitive>::Inner as PrimitiveInner>::Absolute as Into<f32>>::into(
            self.width,
        );
        let height =
            <<<Vector as Primitive>::Inner as PrimitiveInner>::Absolute as Into<f32>>::into(
                self.height,
            );
        let x = top_left.x + (width * 0.5 * scale);
        let y = top_left.y + (height * 0.5 * scale);
        crate::location::Location { x, y }
    }
}

impl<Vector: DynamicVector> Flippable for Rectangle<Vector> {
    fn flip(&mut self, axes: crate::flippable::FlipAxes) {
        //Do nothing
    }
}

impl<Vector: DynamicVector> Rotatable for Rectangle<Vector> {
    fn rotate(&mut self, quarter_turns: crate::rotatable::QuarterTurns) {
        use crate::rotatable::QuarterTurns::*;
        if quarter_turns == One || quarter_turns == Three{
            let new_x= self.top_left.x() + (self.width.to_signed() - self.height.to_signed()).div(<<Vector as Primitive>::Inner>::TWO);

            //let new_top_left = Vector::n

            let new_rect = Rectangle{
                width: self.height,
                height: self.width,
                top_left: new_top_left
            };
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
