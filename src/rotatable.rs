use core::ops::Add;

use strum::{Display, EnumCount, EnumIter};

/// Anything that can be rotated any number of quarter turns
pub trait Rotatable {
    /// Rotate this a given number of quarter turns
    fn rotate(&mut self, quarter_turns: QuarterTurns);
}

#[derive(
    Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumCount, EnumIter,
)]
pub enum QuarterTurns {
    #[default]
    Zero,
    One,
    Two,
    Three,
}

impl Add for QuarterTurns {
    type Output = QuarterTurns;

    fn add(self, rhs: Self) -> Self::Output {
        match (self,rhs) {
            (QuarterTurns::Zero, r) => r,
            (l, QuarterTurns::Zero) => l,
            (QuarterTurns::One, QuarterTurns::One) => QuarterTurns::Two,
            (QuarterTurns::One, QuarterTurns::Two) => QuarterTurns::Three,
            (QuarterTurns::One, QuarterTurns::Three) => QuarterTurns::Zero,
            (QuarterTurns::Two, QuarterTurns::One) => QuarterTurns::Three,
            (QuarterTurns::Two, QuarterTurns::Two) =>QuarterTurns::Zero,
            (QuarterTurns::Two, QuarterTurns::Three) => QuarterTurns::One,
            (QuarterTurns::Three, QuarterTurns::One) => QuarterTurns::Zero,
            (QuarterTurns::Three, QuarterTurns::Two) => QuarterTurns::One,
            (QuarterTurns::Three, QuarterTurns::Three) => QuarterTurns::Two,
        }
    }
}