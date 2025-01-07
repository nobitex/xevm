use std::ops::Neg;

use crate::{
    error::{ExecError, RevertError},
    machine::Word,
};
use uint::construct_uint;

construct_uint! {
    pub struct U256(4);
}

construct_uint! {
    pub struct U512(8);
}

impl Word for U256 {
    const MAX: Self = Self::max_value();
    const ZERO: Self = Self::zero();
    const ONE: Self = Self::one();
    const BITS: usize = 256;
    fn add(self, other: Self) -> Self {
        self.overflowing_add(other).0
    }
    fn sub(self, other: Self) -> Self {
        self.overflowing_sub(other).0
    }
    fn mul(self, other: Self) -> Self {
        self.overflowing_mul(other).0
    }
    fn addmod(self, other: Self, n: Self) -> Self {
        ((self.as_u512() + other.as_u512()) % n.as_u512()).low_u256()
    }
    fn mulmod(self, other: Self, n: Self) -> Self {
        ((self.as_u512() * other.as_u512()) % n.as_u512()).low_u256()
    }
    fn and(self, other: Self) -> Self {
        self & other
    }
    fn div(self, other: Self) -> Self {
        self / other
    }
    fn lt(self, other: Self) -> bool {
        self < other
    }
    fn not(self) -> Self {
        !self
    }
    fn or(self, other: Self) -> Self {
        self | other
    }
    fn xor(self, other: Self) -> Self {
        self ^ other
    }
    fn pow(self, other: Self) -> Self {
        self.pow(other)
    }
    fn rem(self, other: Self) -> Self {
        self % other
    }
    fn shl(self, other: Self) -> Self {
        self << other
    }
    fn shr(self, other: Self) -> Self {
        self >> other
    }

    fn bit(&self, bit: usize) -> bool {
        self.bit(bit)
    }
    fn from_big_endian(slice: &[u8]) -> Self {
        Self::from_big_endian(slice)
    }
    fn to_big_endian(&self) -> Vec<u8> {
        self.to_big_endian().to_vec()
    }
    fn to_usize(&self) -> Result<usize, crate::error::ExecError> {
        if *self < Self::from(usize::MAX as u64) {
            Ok(self.low_u64() as usize)
        } else {
            Err(ExecError::Revert(RevertError::OffsetSizeTooLarge))
        }
    }
}

impl U512 {
    pub fn low_u256(&self) -> U256 {
        U256::from_little_endian(&self.to_little_endian()[..4])
    }
}

impl U256 {
    pub fn as_u512(&self) -> U512 {
        U512::from_little_endian(&self.to_little_endian())
    }
    pub fn to_usize(&self) -> Result<usize, RevertError> {
        if *self > Self::from(usize::MAX) {
            Err(RevertError::OffsetSizeTooLarge)
        } else {
            Ok(self.low_u64() as usize)
        }
    }
    pub fn is_neg(&self) -> bool {
        self.bit(255)
    }
    pub fn hex(&self) -> String {
        format!(
            "0x{}",
            self.to_big_endian().map(|b| format!("{:02x}", b)).join("")
        )
    }
}

impl Neg for U256 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        (!self) + Self::one()
    }
}
