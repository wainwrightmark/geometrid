use core::iter::Map;

use crate::prelude::*;

use num_traits::One;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
/// A general shape made of tiles. 
/// These are represented by vectors
pub trait Shape :// Flippable + Rotatable +
 HasLocation + IntoIterator<Item = Self::Vector>{
    type Vector: Vector;
    type Vertex: Vertex;
    
    // type OutlineIter: Iterator<Item = Self:: Vertex>;
    // type RectangleIter: Iterator<Item = Rectangle<Self::Vector>>;
    // //todo flippable
    // //todo rotatable

    // fn draw_outline(&self)-> Self::OutlineIter; 
    // fn deconstruct_into_rectangles(&self)-> Self::RectangleIter; 
    //todo draw outline
    //todo deconstruct into rectangles
    
}


// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)] //TODO make inner type generic
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// pub struct Rectangle<V : Vector>{
//     pub top_left: V,
//     pub bottom_right: V
// }

// impl<V : Vector> Shape for Rectangle<V> {
//     type Vector = V;
// }
// impl<V : Vector> HasLocation for Rectangle<V>{
//     fn location(&self, scale: f32) -> Location {
//         let x = (self.top_left.x() + self.bottom_right.x()).into() * 0.5 * scale; 
//         let y = (self.top_left.y() + self.bottom_right.y()).into() * 0.5 * scale; 

//         Location { x, y }
//     }
// }

// impl<V : Vector> Flippable for Rectangle<V>{
//     fn flip_horizontal(&mut self) {
//         //Do nothing
//     }

//     fn flip_vertical(&mut self) {
//         //Do nothing
//     }
// }


// impl<V : Vector> IntoIterator for Rectangle<V>{
//     type Item = V;
//     type IntoIter = RectangleIterator<V>;
    

//     fn into_iter(self) -> Self::IntoIter {
//         let next = Some(self.top_left);
//         RectangleIterator{rectangle: self, next}
//     }

    
// }

// #[derive(Debug, Clone,  PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub struct RectangleIterator<V : Vector>{
//     pub rectangle : Rectangle<V>,
//     pub next: Option<V>
// }

// impl<V : Vector> Iterator for RectangleIterator<V>{
//     type Item = V;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.next {
//             Some(next) => {
                
//                 if next.x() >= self.rectangle.bottom_right.x(){
//                     if next.y() >= self.rectangle.bottom_right.y(){
//                         self.next = None;
//                     }
//                     else{
//                         self.next = Some(V::new(self.rectangle.top_left.x(), (next + V::DOWN).y()))
//                     }
//                 }
//                 else{
//                     self.next= Some( next + V::RIGHT);
//                 }
//                 Some(next)
//             },
//             None => None,
//         }
//     }
// }

//todo polyomino
//todo line
//todo rectangle