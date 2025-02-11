use crate::{error::RevertError, machine::Word};

use alloy_primitives::primitives::Address;
pub use alloy_primitives::primitives::U256;

impl Word for U256 {
    type Addr = Address;
    fn from_addr(addr: Self::Addr) -> Self {
        Self::from_big_endian(addr.as_slice())
    }
    fn to_addr(self) -> Result<Self::Addr, RevertError> {
        let max_addr = (U256::ONE << U256::from(160)) - U256::ONE;
        if self < max_addr {
            Ok(Address::from_slice(&self.to_big_endian()[12..]))
        } else {
            Err(RevertError::AddressTooLarge)
        }
    }
    const BITS: usize = 256;
    const MAX: Self = U256::MAX;
    const ONE: Self = U256::from_limbs([1, 0, 0, 0]);
    const ZERO: Self = U256::ZERO;
    fn hex(&self) -> String {
        format!(
            "0x{}",
            self.to_be_bytes_vec()
                .into_iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join("")
        )
    }
    fn low_u64(&self) -> u64 {
        self.as_limbs()[0]
    }
    fn from_u64(val: u64) -> Self {
        U256::from(val)
    }
    fn add(self, other: Self) -> Self {
        self.wrapping_add(other)
    }
    fn addmod(self, other: Self, n: Self) -> Self {
        self.add_mod(other, n)
    }
    fn and(self, other: Self) -> Self {
        self & other
    }
    fn bit(&self, bit: usize) -> bool {
        !(self & (U256::from(1) << bit)).is_zero()
    }
    fn div(self, other: Self) -> Self {
        self.wrapping_div(other)
    }
    fn from_big_endian(slice: &[u8]) -> Self {
        U256::from_be_slice(slice)
    }
    fn lt(self, other: Self) -> bool {
        self < other
    }
    fn mul(self, other: Self) -> Self {
        self.wrapping_mul(other)
    }
    fn mulmod(self, other: Self, n: Self) -> Self {
        self.mul_mod(other, n)
    }
    fn not(self) -> Self {
        !self
    }
    fn or(self, other: Self) -> Self {
        self | other
    }
    fn pow(self, other: Self) -> Self {
        self.wrapping_pow(other)
    }
    fn rem(self, other: Self) -> Self {
        self.wrapping_rem(other)
    }
    fn shl(self, other: Self) -> Self {
        self.wrapping_shl(other.as_limbs()[0] as usize)
    }
    fn shr(self, other: Self) -> Self {
        self.wrapping_shr(other.as_limbs()[0] as usize)
    }
    fn sub(self, other: Self) -> Self {
        self.wrapping_sub(other)
    }
    fn to_big_endian(&self) -> Vec<u8> {
        self.to_be_bytes_vec()
    }
    fn to_usize(&self) -> Result<usize, RevertError> {
        if *self < Self::from_u64(usize::MAX as u64) {
            Ok(self.as_limbs()[0] as usize)
        } else {
            Err(RevertError::OffsetSizeTooLarge)
        }
    }
    fn xor(self, other: Self) -> Self {
        self ^ other
    }
}
