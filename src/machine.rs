use std::collections::HashMap;

use crate::context::{Context, Info};
use crate::error::{ExecError, RevertError};
use crate::opcodes::*;
use crate::u256::U256;

#[derive(Debug, Clone, Default)]
pub struct CallInfo {
    pub origin: U256,
    pub caller: U256,
    pub call_value: U256,
    pub calldata: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct Machine {
    pub address: U256,
    pub code: Vec<u8>,
    pub pc: usize,
    pub gas_used: usize,
    pub stack: Vec<U256>,
    pub memory: Vec<u8>,
    pub transient: HashMap<U256, U256>,
    pub last_return: Option<Vec<u8>>,
}

impl Machine {
    pub fn new(address: U256, code: Vec<u8>) -> Self {
        Self {
            address,
            code,
            pc: 0,
            gas_used: 0,
            stack: Vec::new(),
            memory: Vec::new(),
            transient: HashMap::new(),
            last_return: None,
        }
    }
    pub fn run<C: Context>(
        mut self,
        ctx: &mut C,
        call_info: &CallInfo,
    ) -> Result<ExecutionResult, ExecError> {
        let mut opcode_table: HashMap<u8, Box<dyn OpcodeHandler<C>>> = HashMap::new();
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
        //opcode_table.insert(0x49, Box::new(OpcodeBlobHash));
        opcode_table.insert(0x4a, Box::new(OpcodeInfo(Info::BlobBaseFee)));
        opcode_table.insert(0x50, Box::new(OpcodePop));
        opcode_table.insert(0x51, Box::new(OpcodeMload));
        opcode_table.insert(0x52, Box::new(OpcodeMstore));
        opcode_table.insert(0x53, Box::new(OpcodeMstore8));
        opcode_table.insert(0x54, Box::new(OpcodeSload));
        opcode_table.insert(0x55, Box::new(OpcodeSstore));
        opcode_table.insert(0x56, Box::new(OpcodeJump));
        opcode_table.insert(0x57, Box::new(OpcodeJumpi));
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
        opcode_table.insert(0xf0, Box::new(OpcodeCreate));
        opcode_table.insert(0xf1, Box::new(OpcodeCall::Call));
        opcode_table.insert(0xf2, Box::new(OpcodeUnsupported(0xf2)));
        opcode_table.insert(0xf3, Box::new(OpcodeReturn));
        opcode_table.insert(0xf2, Box::new(OpcodeCall::DelegateCall));
        opcode_table.insert(0xf2, Box::new(OpcodeCreate2));
        opcode_table.insert(0xfa, Box::new(OpcodeCall::StaticCall));
        opcode_table.insert(0xfd, Box::new(OpcodeRevert));
        opcode_table.insert(0xff, Box::new(OpcodeUnsupported(0xff)));

        while self.pc < self.code.len() {
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

    pub fn mem_put(&mut self, offset: usize, data: &[u8]) {
        let expected_len = offset + data.len();
        if expected_len > self.memory.len() {
            self.memory.resize(expected_len, 0);
        }
        self.memory[offset..offset + data.len()].copy_from_slice(data);
    }
    pub fn mem_get(&mut self, offset: usize, size: usize) -> Vec<u8> {
        let mut ret = vec![0u8; size];
        let sz = std::cmp::min(self.memory.len().saturating_sub(offset), size);
        ret[..sz].copy_from_slice(&self.memory[offset..offset + sz]);
        ret
    }
    pub fn pop_stack(&mut self) -> Result<U256, ExecError> {
        Ok(self
            .stack
            .pop()
            .ok_or(RevertError::NotEnoughValuesOnStack)?)
    }
}
