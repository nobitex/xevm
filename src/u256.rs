use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct U256(pub u128, pub u128);

impl From<u64> for U256 {
    fn from(value: u64) -> Self {
        Self(value as u128, 0)
    }
}

impl U256 {
    pub const ZERO: Self = Self(0, 0);
    pub const ONE: Self = Self(1, 0);
    pub const MAX: Self = Self(u128::MAX, u128::MAX);
    pub fn shl128(&self) -> Self {
        Self(0, self.0)
    }
    pub fn lower_usize(&self) -> usize {
        self.0 as usize
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut lower: [u8; 16] = [0; 16];
        let mut higher: [u8; 16] = [0; 16];
        let sz = std::cmp::min(16, bytes.len());
        lower[..std::cmp::min(16, bytes.len())].copy_from_slice(&bytes[..sz]);
        if bytes.len() > 16 {
            let sz = std::cmp::min(16, bytes.len() - 16);
            higher[..sz].copy_from_slice(&bytes[16..16 + sz]);
        }
        U256(u128::from_le_bytes(lower), u128::from_le_bytes(higher))
    }
}

impl Add for U256 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let (l0, carry) = self.0.overflowing_add(rhs.0);
        let l1 = self.1.wrapping_add(rhs.1).wrapping_add(carry as u128);
        U256(l0, l1)
    }
}
impl Sub for U256 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let (l0, carry) = self.0.overflowing_sub(rhs.0);
        let l1 = self.1.wrapping_sub(rhs.1).wrapping_sub(carry as u128);
        U256(l0, l1)
    }
}

fn mul_u128(a: u128, b: u128) -> U256 {
    let a_lo = (a as u64) as u128;
    let a_hi = a >> 64;
    let b_lo = (b as u64) as u128;
    let b_hi = b >> 64;
    let p0 = a_lo * b_lo;
    let p1 = a_lo * b_hi;
    let p2 = a_hi * b_lo;
    let p3 = a_hi * b_hi;
    let cy: u64 = (p0 >> 64)
        .wrapping_add((p1 as u64) as u128)
        .wrapping_add((p2 as u64) as u128)
        .wrapping_shr(64) as u64;
    U256(
        p0.wrapping_add(p1 << 64).wrapping_add(p2 << 64),
        p3.wrapping_add(p1 >> 64)
            .wrapping_add(p2 >> 64)
            .wrapping_add(cy as u128),
    )
}

impl Mul for U256 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let p0 = mul_u128(self.0, rhs.0);
        let p1 = mul_u128(self.0, rhs.1);
        let p2 = mul_u128(self.1, rhs.0);
        p0 + p1.shl128() + p2.shl128()
    }
}