use crate::{vector::Vector, location::HasLocation, flippable::Flippable, rotatable::Rotatable};


/// A general shape made of tiles. 
/// These are represented by vectors
pub trait Shape : Flippable + Rotatable + HasLocation + IntoIterator<Item = Self::Vector>{
    type Vector: Vector;

    
}