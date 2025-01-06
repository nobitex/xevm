use std::ops::Neg;

use crate::error::RevertError;
use uint::construct_uint;

construct_uint! {
    pub struct U256(4);
}

construct_uint! {
    pub struct U512(8);
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
