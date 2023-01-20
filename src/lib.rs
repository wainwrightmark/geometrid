// #![cfg_attr(not(any(test, feature = "std")), no_std)]
#![doc(html_root_url = "https://docs.rs/geometrid/0.1.0")]
// #![deny(missing_docs)]
#![allow(warnings, dead_code, unused_imports, unused_mut)]
#![warn(clippy::pedantic)]

//! [![github]](https://github.com/wainwrightmark/geometrid)&ensp;[![crates-io]](https://crates.io/crates/geometrid)&ensp;[![docs-rs]](https://docs.rs/geometrid)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! 2d grids and coordinates
//!
//! <br>
//!
//! ## Usage
//!
//! ```
//! use geometrid::*;
//!
//!
//! fn main()  {
//! }
//! ```
//!
//! ## Readme DocsLine
//!
//! You can find the crate's readme documentation on the
//! [crates.io] page, or alternatively in the [`README.md`] file on the GitHub project repo.
//!
//! [crates.io]: https://crates.io/crates/geometrid
//! [`README.md`]: https://github.com/wainwrightmark/geometrid

pub mod grid;
pub mod line_finder;
pub mod point_absolute;
pub mod point_relative;
pub mod polyomino;
pub mod rectangle;

pub mod prelude8 {
    pub use crate::grid::Grid8;
    pub use crate::line_finder::Line8;
    pub use crate::point_absolute::PointAbsolute8;
    pub use crate::point_relative::PointRelative8;
}
pub mod prelude16 {
    pub use crate::grid::Grid16;
    pub use crate::line_finder::Line16;
    pub use crate::point_absolute::PointAbsolute16;
    pub use crate::point_relative::PointRelative16;
}
pub mod prelude32 {
    pub use crate::grid::Grid32;
    pub use crate::line_finder::Line32;
    pub use crate::point_absolute::PointAbsolute32;
    pub use crate::point_relative::PointRelative32;
}
pub mod prelude64 {
    pub use crate::grid::Grid64;
    pub use crate::line_finder::Line64;
    pub use crate::point_absolute::PointAbsolute64;
    pub use crate::point_relative::PointRelative64;
}
