use core::ops::{Add, Mul, Neg};

use num_traits::{One, Zero};

use crate::{prelude::*, primitives::DynamicPrimitive, inners::PrimitiveInner};

pub trait DynamicVector: DynamicPrimitive {
    const ZERO: Self;
    const UP: Self;
    const UP_RIGHT: Self;
    const RIGHT: Self;
    const DOWN_RIGHT: Self;
    const DOWN: Self;
    const DOWN_LEFT: Self;
    const LEFT: Self;
    const UP_LEFT: Self;

    const CARDINALS: [Self; 4] = [Self::UP, Self::RIGHT, Self::DOWN, Self::LEFT];

    const UNITS: [Self; 8] = [
        Self::UP,
        Self::UP_RIGHT,
        Self::RIGHT,
        Self::DOWN_RIGHT,
        Self::DOWN,
        Self::DOWN_LEFT,
        Self::LEFT,
        Self::UP_LEFT,
    ];

    const UNIT_NAMES: [&'static str; 8] = [
        "Up",
        "Up Right",
        "Right",
        "Down Right",
        "Down",
        "Down Left",
        "Left",
        "Up Left",
    ];

    /// Returns true if this is a unit vector
    #[must_use]
    #[inline]
    fn is_unit(&self) -> bool {
        let one = <<Self::Inner as PrimitiveInner>::Absolute>::one();
        self.x().abs() <= one && self.y().abs() <= one && self != &Self::ZERO
    }

    /// Returns true if this is a diagonal vector, having both an x and a y component
    fn is_diagonal(&self) -> bool {
        !self.x().is_zero() && !self.y().is_zero()
    }
}