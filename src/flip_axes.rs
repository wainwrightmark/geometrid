#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter};

/// The axes across which to flip
#[derive(
    Default,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    EnumCount,
    EnumIter,
    EnumIs,
)]
#[cfg_attr(any(test, feature = "serde"), derive(Serialize, Deserialize))]
pub enum FlipAxes {
    #[default]
    None,
    Horizontal,
    Vertical,
    Both,
}
