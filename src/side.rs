#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter, Display};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumCount, EnumIter)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Side{
    Top,
    Right,
    Left,
    Bottom,
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumCount, EnumIter)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Orientation{
    Horizontal,
    Vertical
}