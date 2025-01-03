use sha3::{Digest, Keccak256};

pub fn keccak(inp: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(inp);
    let result = hasher.finalize();
    result.into()
}
