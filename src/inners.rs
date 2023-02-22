use crate::corner::*;
use crate::flippable::*;
use crate::location::*;
use crate::rotatable::*;
use crate::side::*;
use num_traits::{CheckedAdd, CheckedMul, One, PrimInt, Signed, Unsigned, Zero};
use core::fmt::Debug;
use core::ops::Mul;
use core::ops::Neg;

pub trait PrimitiveInner:
    PrimInt
    + Into<f32>
    + Clone
    + Copy
    + CheckedMul<Output = Self>
    + CheckedAdd
    + core::hash::Hash
    + Debug
    + TryFrom<usize>
{
    type Absolute: UnsignedInner;
    type Signed: SignedInner;

    fn to_signed(self)-> Self::Signed;

    fn abs(self) -> Self::Absolute;
    fn abs_diff(self, other: Self) -> Self::Absolute;

    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
}

pub trait UnsignedInner: PrimitiveInner + Unsigned + Into<usize> {

}

pub trait SignedInner: PrimitiveInner + Signed + TryFrom<isize> + Into<isize> + Neg {
    const MINUS_ONE: Self;
}

impl PrimitiveInner for u8 {
    type Absolute = Self;
    type Signed = i8;

    fn abs(self) -> Self::Absolute {
        self
    }

    fn abs_diff(self, other: Self) -> Self::Absolute {
        self.abs_diff(other)
    }
    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;

    fn to_signed(self)-> Self::Signed {
        self as i8
    }
}
impl PrimitiveInner for u16 {
    type Absolute = Self;
    type Signed = i16;

    fn abs(self) -> Self::Absolute {
        self
    }

    fn abs_diff(self, other: Self) -> Self::Absolute {
        self.abs_diff(other)
    }

    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;

    fn to_signed(self)-> Self::Signed {
        self as i16
    }
}

impl UnsignedInner for u8 {}
impl UnsignedInner for u16 {}

impl PrimitiveInner for i8 {
    type Absolute = u8;
    type Signed = Self;

    fn abs(self) -> Self::Absolute {
        self.abs() as u8
    }

    fn abs_diff(self, other: Self) -> Self::Absolute {
        self.abs_diff(other)
    }

    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;

    fn to_signed(self)-> Self::Signed {
        self
    }
}
impl PrimitiveInner for i16 {
    type Absolute = u16;
    type Signed = Self;

    fn abs(self) -> Self::Absolute {
        self.abs() as u16
    }

    fn abs_diff(self, other: Self) -> Self::Absolute {
        self.abs_diff(other)
    }

    const ZERO: Self = 0;
    const ONE: Self = 1;
    const TWO: Self = 2;

    fn to_signed(self)-> Self::Signed {
        self
    }
}

impl SignedInner for i8 {
    const MINUS_ONE: Self = -1;
}
impl SignedInner for i16 {
    const MINUS_ONE: Self = -1;
}