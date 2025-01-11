use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::context::{Context, Info};
use crate::error::{ExecError, RevertError};
use crate::opcodes::*;

#[derive(Debug, Clone, Default)]
pub struct CallInfo<W: Word> {
    pub origin: W::Addr,
    pub caller: W::Addr,
    pub call_value: W,
    pub calldata: Vec<u8>,
}

pub trait Word: Clone + Debug + Default + Copy + PartialEq + Eq + PartialOrd + Ord + Hash {
    type Addr: Clone + Debug + Default + Copy;
    const MAX: Self;
    const ZERO: Self;
    const ONE: Self;
    const BITS: usize;
    fn from_addr(addr: Self::Addr) -> Self;
    fn to_addr(self) -> Self::Addr;
    fn hex(&self) -> String;
    fn low_u64(&self) -> u64;
    fn from_u64(val: u64) -> Self;
    fn bit(&self, bit: usize) -> bool;
    fn is_neg(&self) -> bool {
        self.bit(Self::BITS - 1)
    }
    fn to_usize(&self) -> Result<usize, ExecError>;
    fn from_big_endian(slice: &[u8]) -> Self;
    fn to_big_endian(&self) -> Vec<u8>;
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
    fn mul(self, other: Self) -> Self;
    fn div(self, other: Self) -> Self;
    fn rem(self, other: Self) -> Self;
    fn shl(self, other: Self) -> Self;
    fn shr(self, other: Self) -> Self;
    fn and(self, other: Self) -> Self;
    fn or(self, other: Self) -> Self;
    fn xor(self, other: Self) -> Self;
    fn pow(self, other: Self) -> Self;
    fn not(self) -> Self;
    fn neg(self) -> Self {
        self.not().add(Self::ONE)
    }
    fn lt(self, other: Self) -> bool;
    fn gt(self, other: Self) -> bool {
        other.lt(self)
    }
    fn addmod(self, other: Self, n: Self) -> Self;
    fn mulmod(self, other: Self, n: Self) -> Self;
}

#[derive(Debug, Clone, Default)]
pub struct Machine<W: Word> {
    pub address: W::Addr,
    pub code: Vec<u8>,
    pub pc: usize,
    pub gas: usize,
    pub stack: Vec<W>,
    pub memory: Vec<u8>,
    pub transient: HashMap<W, W>,
    pub last_return: Option<Vec<u8>>,
}

impl<W: Word> Machine<W> {
    pub fn new(address: W::Addr, code: Vec<u8>, gas: usize) -> Self {
        Self {
            address,
            code,
            pc: 0,
            gas,
            stack: Vec::new(),
            memory: Vec::new(),
            transient: HashMap::new(),
            last_return: None,
        }
    }
    pub fn run<C: Context<W>>(
        mut self,
        ctx: &mut C,
        call_info: &CallInfo<W>,
    ) -> Result<ExecutionResult, ExecError> {
        let mut opcode_table: HashMap<u8, Box<dyn OpcodeHandler<W, C>>> = HashMap::new();
        opcode_table.insert(0x00, Box::new(OpcodeHalt));
        opcode_table.insert(0x01, Box::new(OpcodeBinaryOp::Add));
        opcode_table.insert(0x02, Box::new(OpcodeBinaryOp::Mul));
        opcode_table.insert(0x03, Box::new(OpcodeBinaryOp::Sub));
        opcode_table.insert(0x04, Box::new(OpcodeBinaryOp::Div));
        opcode_table.insert(0x05, Box::new(OpcodeBinaryOp::Sdiv));
        opcode_table.insert(0x06, Box::new(OpcodeBinaryOp::Mod));
        opcode_table.insert(0x07, Box::new(OpcodeBinaryOp::Smod));
        opcode_table.insert(0x08, Box::new(OpcodeModularOp::AddMod));
        opcode_table.insert(0x09, Box::new(OpcodeModularOp::MulMod));
        opcode_table.insert(0x0a, Box::new(OpcodeBinaryOp::Exp));
        opcode_table.insert(0x0b, Box::new(OpcodeBinaryOp::SignExtend));
        opcode_table.insert(0x10, Box::new(OpcodeBinaryOp::Lt));
        opcode_table.insert(0x11, Box::new(OpcodeBinaryOp::Gt));
        opcode_table.insert(0x12, Box::new(OpcodeBinaryOp::Slt));
        opcode_table.insert(0x13, Box::new(OpcodeBinaryOp::Sgt));
        opcode_table.insert(0x14, Box::new(OpcodeBinaryOp::Eq));
        opcode_table.insert(0x15, Box::new(OpcodeUnaryOp::IsZero));
        opcode_table.insert(0x16, Box::new(OpcodeBinaryOp::And));
        opcode_table.insert(0x17, Box::new(OpcodeBinaryOp::Or));
        opcode_table.insert(0x18, Box::new(OpcodeBinaryOp::Xor));
        opcode_table.insert(0x19, Box::new(OpcodeUnaryOp::Not));
        opcode_table.insert(0x1a, Box::new(OpcodeBinaryOp::Byte));
        opcode_table.insert(0x1b, Box::new(OpcodeBinaryOp::Shl));
        opcode_table.insert(0x1c, Box::new(OpcodeBinaryOp::Shr));
        opcode_table.insert(0x1d, Box::new(OpcodeBinaryOp::Sar));
        opcode_table.insert(0x20, Box::new(OpcodeKeccak));
        opcode_table.insert(0x30, Box::new(OpcodeAddress));
        opcode_table.insert(0x31, Box::new(OpcodeBalance));
        opcode_table.insert(0x32, Box::new(OpcodeOrigin));
        opcode_table.insert(0x33, Box::new(OpcodeCaller));
        opcode_table.insert(0x34, Box::new(OpcodeCallValue));
        opcode_table.insert(0x35, Box::new(OpcodeCalldataLoad));
        opcode_table.insert(0x36, Box::new(OpcodeCalldataSize));
        opcode_table.insert(0x37, Box::new(OpcodeCalldataCopy));
        opcode_table.insert(0x38, Box::new(OpcodeCodeSize));
        opcode_table.insert(0x39, Box::new(OpcodeCodeCopy));
        opcode_table.insert(0x3a, Box::new(OpcodeInfo(Info::GasPrice)));
        opcode_table.insert(0x3b, Box::new(OpcodeExtCodeSize));
        opcode_table.insert(0x3c, Box::new(OpcodeExtCodeCopy));
        opcode_table.insert(0x3d, Box::new(OpcodeReturnDataSize));
        opcode_table.insert(0x3e, Box::new(OpcodeReturnDataCopy));
        opcode_table.insert(0x3f, Box::new(OpcodeExtCodeHash));
        opcode_table.insert(0x40, Box::new(OpcodeBlockHash));
        opcode_table.insert(0x41, Box::new(OpcodeInfo(Info::Coinbase)));
        opcode_table.insert(0x42, Box::new(OpcodeInfo(Info::Timestamp)));
        opcode_table.insert(0x43, Box::new(OpcodeInfo(Info::Number)));
        opcode_table.insert(0x44, Box::new(OpcodeInfo(Info::PrevRandao)));
        opcode_table.insert(0x45, Box::new(OpcodeInfo(Info::GasLimit)));
        opcode_table.insert(0x46, Box::new(OpcodeInfo(Info::ChainId)));
        opcode_table.insert(0x47, Box::new(OpcodeSelfBalance));
        opcode_table.insert(0x48, Box::new(OpcodeInfo(Info::BaseFee)));
        opcode_table.insert(0x49, Box::new(OpcodeBlobHash));
        opcode_table.insert(0x4a, Box::new(OpcodeInfo(Info::BlobBaseFee)));
        opcode_table.insert(0x50, Box::new(OpcodePop));
        opcode_table.insert(0x51, Box::new(OpcodeMload));
        opcode_table.insert(0x52, Box::new(OpcodeMstore));
        opcode_table.insert(0x53, Box::new(OpcodeMstore8));
        opcode_table.insert(0x54, Box::new(OpcodeSload));
        opcode_table.insert(0x55, Box::new(OpcodeSstore));
        opcode_table.insert(0x56, Box::new(OpcodeJump));
        opcode_table.insert(0x57, Box::new(OpcodeJumpi));
        opcode_table.insert(0x58, Box::new(OpcodePc));
        opcode_table.insert(0x59, Box::new(OpcodeMsize));
        opcode_table.insert(0x5a, Box::new(OpcodeGas));
        opcode_table.insert(0x5b, Box::new(OpcodeJumpDest));
        opcode_table.insert(0x5c, Box::new(OpcodeTload));
        opcode_table.insert(0x5d, Box::new(OpcodeTstore));
        opcode_table.insert(0x5e, Box::new(OpcodeMcopy));
        for sz in 0..=32 {
            opcode_table.insert(0x5f + sz, Box::new(OpcodePush(sz)));
        }
        for sz in 0..16 {
            opcode_table.insert(0x80 + sz, Box::new(OpcodeDup(sz)));
        }
        for sz in 0..16 {
            opcode_table.insert(0x90 + sz, Box::new(OpcodeSwap(sz)));
        }
        for sz in 0..5 {
            opcode_table.insert(0xa0 + sz, Box::new(OpcodeLog(sz)));
        }
        opcode_table.insert(0xf0, Box::new(OpcodeCreate::Create));
        opcode_table.insert(0xf1, Box::new(OpcodeCall::Call));
        opcode_table.insert(0xf2, Box::new(OpcodeUnsupported(0xf2)));
        opcode_table.insert(0xf3, Box::new(OpcodeReturn));
        opcode_table.insert(0xf2, Box::new(OpcodeCall::DelegateCall));
        opcode_table.insert(0xf2, Box::new(OpcodeCreate::Create2));
        opcode_table.insert(0xfa, Box::new(OpcodeCall::StaticCall));
        opcode_table.insert(0xfd, Box::new(OpcodeRevert));
        opcode_table.insert(0xff, Box::new(OpcodeSelfDestruct));

        while self.pc < self.code.len() {
            self.consume_gas(3)?; // Consume gas to prevent infinite loops
            let opcode = self.code[self.pc];
            if let Some(opcode_fn) = opcode_table.get(&opcode) {
                if let Some(res) = opcode_fn.call(ctx, &mut self, call_info)? {
                    return Ok(res);
                }
            } else {
                return Err(RevertError::UnknownOpcode(opcode).into());
            }
        }
        Ok(ExecutionResult::Halted)
    }
    pub fn consume_gas(&mut self, gas: usize) -> Result<(), RevertError> {
        if self.gas < gas {
            Err(RevertError::InsufficientGas)
        } else {
            Ok(())
        }
    }
    pub fn mem_put(
        &mut self,
        target_offset: usize,
        source: &[u8],
        source_offset: usize,
        len: usize,
    ) -> Result<(), RevertError> {
        if source_offset >= source.len() {
            return Ok(());
        }
        let source_end = std::cmp::min(source_offset + len, source.len());
        let src = &source[source_offset..source_end];
        let expected_len = target_offset + src.len();
        if expected_len > self.memory.len() {
            let extension_len = expected_len - self.memory.len();
            self.consume_gas(extension_len * 3)?;
            self.memory.resize(expected_len, 0);
        }
        self.memory[target_offset..target_offset + src.len()].copy_from_slice(src);
        Ok(())
    }
    pub fn mem_get(&mut self, offset: usize, size: usize) -> Vec<u8> {
        let mut ret = vec![0u8; size];
        if offset < self.memory.len() {
            let sz = std::cmp::min(self.memory.len().saturating_sub(offset), size);
            ret[..sz].copy_from_slice(&self.memory[offset..offset + sz]);
        }
        ret
    }
    pub fn push_stack(&mut self, value: W) -> Result<(), RevertError> {
        if self.stack.len() >= 1024 {
            return Err(RevertError::StackFull);
        }
        self.stack.push(value);
        Ok(())
    }
    pub fn pop_stack(&mut self) -> Result<W, ExecError> {
        Ok(self
            .stack
            .pop()
            .ok_or(RevertError::NotEnoughValuesOnStack)?)
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::primitives::{Address, U256};

    use crate::machine::Machine;

    #[test]
    fn test_mem_put() {
        let mut m = Machine::<U256>::new(Address::ZERO, vec![], 10000000);
        assert_eq!(m.memory, vec![]);
        m.mem_put(2, &[1, 2, 3], 1, 10).unwrap();
        assert_eq!(m.memory, vec![0, 0, 2, 3]);
        m.mem_put(1, &[4, 5, 6], 5, 10).unwrap();
        assert_eq!(m.memory, vec![0, 0, 2, 3]);
        m.mem_put(1, &[4, 5, 6], 1, 1).unwrap();
        assert_eq!(m.memory, vec![0, 5, 2, 3]);
        m.mem_put(1, &[7, 8, 9], 1, 0).unwrap();
        assert_eq!(m.memory, vec![0, 5, 2, 3]);
        m.mem_put(3, &[7, 8, 9], 2, 10).unwrap();
        assert_eq!(m.memory, vec![0, 5, 2, 9]);
        m.mem_put(4, &[10, 11, 12, 13], 0, 2).unwrap();
        assert_eq!(m.memory, vec![0, 5, 2, 9, 10, 11]);
        m.mem_put(4, &[10, 11, 12, 13], 0, 100).unwrap();
        assert_eq!(m.memory, vec![0, 5, 2, 9, 10, 11, 12, 13]);
        m.mem_put(10, &[10, 11, 12, 13], 2, 100).unwrap();
        assert_eq!(m.memory, vec![0, 5, 2, 9, 10, 11, 12, 13, 0, 0, 12, 13]);
    }

    #[test]
    fn test_mem_get() {
        let mut m = Machine::<U256>::new(Address::ZERO, vec![], 10000000);
        m.memory = vec![0, 10, 20, 30, 40, 50];
        assert_eq!(m.mem_get(1, 3), vec![10, 20, 30]);
        assert_eq!(
            m.mem_get(0, 100),
            vec![
                0, 10, 20, 30, 40, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
        assert_eq!(m.mem_get(100, 2), vec![0, 0]);
        assert_eq!(m.mem_get(5, 2), vec![50, 0]);
    }
}
