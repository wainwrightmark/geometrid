use crate::primitive::UniformPrimitive;

pub trait Flippable{
    fn flip_horizontal(&mut self);
    fn flip_vertical(&mut self);
}


impl<T : UniformPrimitive> Flippable for T
{
    fn flip_horizontal(&mut self) {
        *self = Self::try_new(Self::MAX_COL - self.col(), self.row()).unwrap()
    }

    fn flip_vertical(&mut self) {
        *self = Self::try_new(self.col(), Self::MAX_ROW - self.row()).unwrap()
    }
}