#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![allow(warnings, dead_code, unused_imports, unused_mut)]
#![warn(clippy::pedantic)]

pub mod corner;
pub mod dynamic_tile;
pub mod dynamic_vector;
pub mod dynamic_vertex;
pub mod flippable;
pub mod location;
pub mod rotatable;
pub mod side;
pub mod vector;
pub mod primitive_iterator;
pub mod primitives;
pub mod inners;
pub mod fixed_tile;
pub mod fixed_vertex;
pub mod tile_grid;
pub mod shape;
pub mod rectangle;

use core::fmt::Debug;
use core::ops::Mul;
use core::ops::Neg;

pub mod prelude {
    use crate::corner::*;
    use crate::dynamic_vector::*;
    use crate::flippable::*;
    use crate::location::*;
    use crate::rotatable::*;
    use crate::side::*;
    use crate::vector::*;
    use crate::primitives::*;
    use crate::inners::*;
    use crate::primitive_iterator::*;
}
