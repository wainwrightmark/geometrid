use core::ops::Add;
#[cfg(any(test, feature = "serde"))]
use serde::{Deserialize, Serialize};
use strum::*;

/// The number of quarter turns to rotate clockwise
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
pub enum QuarterTurns {
    /// Do not rotate
    #[default]
    Zero,

    One,
    Two,
    /// Corresponds to one quarter turn anticlockwise
    Three,
}

impl Add for QuarterTurns {
    type Output = QuarterTurns;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (QuarterTurns::Zero, r) => r,
            (l, QuarterTurns::Zero) => l,
            (QuarterTurns::One, QuarterTurns::One) => QuarterTurns::Two,
            (QuarterTurns::One, QuarterTurns::Two) => QuarterTurns::Three,
            (QuarterTurns::One, QuarterTurns::Three) => QuarterTurns::Zero,
            (QuarterTurns::Two, QuarterTurns::One) => QuarterTurns::Three,
            (QuarterTurns::Two, QuarterTurns::Two) => QuarterTurns::Zero,
            (QuarterTurns::Two, QuarterTurns::Three) => QuarterTurns::One,
            (QuarterTurns::Three, QuarterTurns::One) => QuarterTurns::Zero,
            (QuarterTurns::Three, QuarterTurns::Two) => QuarterTurns::One,
            (QuarterTurns::Three, QuarterTurns::Three) => QuarterTurns::Two,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vertex::*;
    use itertools::Itertools;
    #[cfg(any(test, feature = "serde"))]
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_add() {
        use QuarterTurns::*;
        let arr = [(0, Zero), (1, One), (2, Two), (3, Three)];
        for (i, qi) in arr.iter().cloned() {
            for (j, qj) in arr.iter().cloned() {
                let sum = i + j;
                let q_sum = qi + qj;

                let expected = match q_sum {
                    Zero => 0,
                    One => 1,
                    Two => 2,
                    Three => 3,
                };

                assert_eq!(sum % 4, expected);
            }
        }
    }
}
