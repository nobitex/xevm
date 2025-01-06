use std::ops::Neg;

use crate::error::RevertError;
use uint::construct_uint;

construct_uint! {
    pub struct U256(4);
}

impl U256 {
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
