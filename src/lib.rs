#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![doc(html_root_url = "https://docs.rs/geometrid/0.10.0")]
// #![deny(missing_docs)]
#![deny(warnings, dead_code, unused_imports, unused_mut)]
#![warn(clippy::pedantic)]

pub mod corner;
pub mod dynamic_tile;
pub mod dynamic_vertex;
pub mod flip_axes;
pub mod has_center;
pub mod line_finder;
pub mod line_of_sight;
pub mod polyomino;
pub mod quarter_turns;
pub mod rectangle;
pub mod shape;
pub mod tile;
pub mod tile_map;
pub mod tile_set;
#[cfg(any(test, feature = "u256"))]
pub mod tile_set256;
pub mod vector;
pub mod vertex;

pub mod prelude {
    pub use crate::corner::*;
    pub use crate::dynamic_tile::*;
    pub use crate::dynamic_vertex::*;
    pub use crate::flip_axes::*;
    #[cfg(any(test, feature = "glam"))]
    pub use crate::has_center::*;
    pub use crate::line_finder::*;
    pub use crate::line_of_sight::*;
    pub use crate::polyomino::*;
    pub use crate::quarter_turns::*;
    pub use crate::rectangle::*;
    pub use crate::shape::*;
    pub use crate::tile::*;
    pub use crate::tile_map::*;
    pub use crate::tile_set::*;
    #[cfg(any(test, feature = "u256"))]
    pub use crate::tile_set256::*;
    pub use crate::vector::*;
    pub use crate::vertex::*;
}
