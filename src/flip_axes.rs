#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};

use strum::{Display, EnumCount, EnumIter};

#[derive(
    Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumCount, EnumIter,
)]
#[cfg_attr(any(test, feature = "serde"), derive(Serialize, Deserialize))]
pub enum FlipAxes {
    #[default]
    None,
    Horizontal,
    Vertical,
    Both,
}
