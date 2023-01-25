use crate::primitive::UniformPrimitive;

pub trait Flippable{
    fn flip_horizontal(&mut self);
    fn flip_vertical(&mut self);
}


