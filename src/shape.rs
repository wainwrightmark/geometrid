use crate::{
    dynamic_tile::DynamicTile, dynamic_vector::DynamicVector, dynamic_vertex::DynamicVertex,
    flippable::Flippable, inners::{PrimitiveInner, SignedInner}, location::HasLocation, primitives::Primitive,
    rectangle::Rectangle, rotatable::Rotatable,
};

pub trait Shape: HasLocation + Flippable + Rotatable
where <<Self as Shape>::Vector as Primitive>::Inner: SignedInner,

 {
    type Vector: DynamicVector ;
    type OutlineIter: Iterator<Item = DynamicVertex<Self::Vector>>;
    type RectangleIter: Iterator<Item = Rectangle<Self::Vector>>;
    type TilesIter: Iterator<Item = DynamicTile<Self::Vector>>;

    /// The number of tiles this shape takes up
    fn tiles_count(&self) -> usize;

    fn tiles_iter(&self) -> Self::TilesIter;

    fn outline(&self) -> Self::OutlineIter;

    fn deconstruct(&self) -> Self::RectangleIter;
}
