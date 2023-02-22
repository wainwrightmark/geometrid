use core::fmt::Debug;
use core::ops::{Add, Mul, Neg};
use num_traits::One;
use num_traits::Zero;

use crate::corner::Corner;
use crate::dynamic_vector::DynamicVector;
use crate::flippable::FlipAxes;
use crate::flippable::Flippable;
use crate::inners::PrimitiveInner;
use crate::inners::SignedInner;
use crate::inners::UnsignedInner;
use crate::location::HasLocation;
use crate::prelude::*;
use crate::primitives::DynamicPrimitive;
use crate::primitives::Primitive;
use crate::rotatable::QuarterTurns;
use crate::rotatable::Rotatable;
use crate::side::Orientation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector<Inner: SignedInner> {
    pub x: Inner,
    pub y: Inner,
}

impl<Inner: SignedInner> DynamicVector for Vector<Inner> {
    const ZERO: Self = Self {
        x: Inner::ZERO,
        y: Inner::ZERO,
    };

    const UP: Self = Self {
        x: Inner::ZERO,
        y: Inner::ONE,
    };

    const UP_RIGHT: Self = Self {
        x: Inner::ONE,
        y: Inner::ONE,
    };

    const RIGHT: Self = Self {
        x: Inner::ONE,
        y: Inner::ZERO,
    };

    const DOWN_RIGHT: Self = Self {
        x: Inner::ONE,
        y: Inner::MINUS_ONE,
    };

    const DOWN: Self = Self {
        x: Inner::ZERO,
        y: Inner::MINUS_ONE,
    };

    const DOWN_LEFT: Self = Self {
        x: Inner::MINUS_ONE,
        y: Inner::MINUS_ONE,
    };

    const LEFT: Self = Self {
        x: Inner::MINUS_ONE,
        y: Inner::ZERO,
    };

    const UP_LEFT: Self = Self {
        x: Inner::MINUS_ONE,
        y: Inner::ONE,
    };
}

impl<Inner: SignedInner> Neg for Vector<Inner> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
        }
    }
}

impl<Inner: SignedInner> Mul<isize> for Vector<Inner> {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        let rhs = rhs.try_into().ok().unwrap();

        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<Inner: SignedInner> Mul<usize> for Vector<Inner> {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        let rhs = rhs.try_into().ok().unwrap();
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<Inner: SignedInner> Add for Vector<Inner> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<Inner: SignedInner> Zero for Vector<Inner> {
    fn zero() -> Self {
        Self {
            x: Inner::zero(),
            y: Inner::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
}

impl<Inner: SignedInner> HasLocation for Vector<Inner> {
    fn location(&self, scale: f32) -> crate::location::Location {
        crate::location::Location {
            x: Into::<f32>::into(self.x) * scale,
            y: Into::<f32>::into(self.y) * scale,
        }
    }
}

impl<Inner: SignedInner> Flippable for Vector<Inner> {
    fn flip(&mut self, axes: crate::flippable::FlipAxes) {
        match axes {
            FlipAxes::None => {}
            FlipAxes::Horizontal => self.x = self.x.neg(),
            FlipAxes::Vertical => self.y = self.y.neg(),
            FlipAxes::Both => {
                self.x = self.x.neg();
                self.y = self.y.neg();
            }
        }
    }
}

impl<Inner: SignedInner> Rotatable for Vector<Inner> {
    fn rotate(&mut self, quarter_turns: crate::rotatable::QuarterTurns) {
        *self = match quarter_turns {
            QuarterTurns::Zero => {
                return;
            }
            QuarterTurns::One => Self {
                x: self.y(),
                y: -self.x(),
            },
            QuarterTurns::Two => Self {
                x: -self.x(),
                y: -self.y(),
            },
            QuarterTurns::Three => Self {
                x: -self.y(),
                y: self.x(),
            },
        }
    }
}

impl<Inner: SignedInner> Primitive for Vector<Inner> {
    type Inner = Inner;

    const CENTER: Self = Self {
        x: Inner::ZERO,
        y: Inner::ZERO,
    };

    fn x(&self) -> Self::Inner {
        self.x
    }

    fn y(&self) -> Self::Inner {
        self.y
    }
}

impl<Inner: SignedInner> DynamicPrimitive for Vector<Inner> {
    fn with_x(&self, x: Self::Inner) -> Self {
        Self { x, y: self.y }
    }

    fn with_y(&self, y: Self::Inner) -> Self {
        Self { y, x: self.x }
    }

    fn new(x: Self::Inner, y: Self::Inner)-> Self {
        Self { x, y }
    }
}
