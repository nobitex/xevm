use std::collections::HashMap;
use std::error::Error;

use crate::error::XevmError;
use crate::opcodes::*;
use crate::{u256::U256, ExecutionResult, OpcodeHandler};

pub trait Context {
    fn address(&self) -> Result<U256, Box<dyn Error>>;
    fn balance(&self, address: U256) -> Result<U256, Box<dyn Error>>;
    fn sload(&self, address: U256) -> Result<U256, Box<dyn Error>>;
    fn sstore(&mut self, address: U256, value: U256) -> Result<(), Box<dyn Error>>;
    fn log(&self, topics: Vec<U256>, data: Vec<u8>) -> Result<(), Box<dyn Error>>;
}

pub struct CallInfo {
    pub origin: U256,
    pub caller: U256,
    pub call_value: U256,
    pub calldata: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct Machine {
    pub code: Vec<u8>,
    pub pc: usize,
    pub gas_used: usize,
    pub stack: Vec<U256>,
    pub memory: Vec<u8>,
    pub transient: HashMap<U256, U256>,
}

impl Machine {
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            code,
            pc: 0,
            gas_used: 0,
            stack: Vec::new(),
            memory: Vec::new(),
            transient: HashMap::new(),
        }
    }
    pub fn run<C: Context>(
        mut self,
        ctx: &mut C,
        call_info: &CallInfo,
    ) -> Result<ExecutionResult, XevmError> {
        let code = self.code.clone();
        let mut opcode_table: HashMap<u8, Box<dyn OpcodeHandler<C>>> = HashMap::new();
        opcode_table.insert(0x00, Box::new(OpcodeHalt));
        opcode_table.insert(0x01, Box::new(OpcodeAdd));
        opcode_table.insert(0x02, Box::new(OpcodeMul));

        opcode_table.insert(0x03, Box::new(OpcodeSub));
        /*opcode_table.insert(0x04, Box::new(OpcodeDiv));
        opcode_table.insert(0x05, Box::new(OpcodeSdiv));
        opcode_table.insert(0x06, Box::new(OpcodeMod));
        opcode_table.insert(0x07, Box::new(OpcodeSmod));
        opcode_table.insert(0x08, Box::new(OpcodeAddMod));
        opcode_table.insert(0x09, Box::new(OpcodeMulMod));
        opcode_table.insert(0x0a, Box::new(OpcodeExp));
        opcode_table.insert(0x0b, Box::new(OpcodeSignExtend));*/

        opcode_table.insert(0x10, Box::new(OpcodeLt));
        opcode_table.insert(0x11, Box::new(OpcodeGt));
        opcode_table.insert(0x12, Box::new(OpcodeSlt));
        opcode_table.insert(0x13, Box::new(OpcodeSgt));
        opcode_table.insert(0x14, Box::new(OpcodeEq));
        opcode_table.insert(0x15, Box::new(OpcodeIsZero));
        opcode_table.insert(0x16, Box::new(OpcodeAnd));
        opcode_table.insert(0x17, Box::new(OpcodeOr));
        opcode_table.insert(0x18, Box::new(OpcodeXor));
        opcode_table.insert(0x19, Box::new(OpcodeNot));
        opcode_table.insert(0x1a, Box::new(OpcodeByte));
        opcode_table.insert(0x1b, Box::new(OpcodeShl));
        opcode_table.insert(0x1c, Box::new(OpcodeShr));
        /*opcode_table.insert(0x1d, Box::new(OpcodeSar));*/

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
        /*opcode_table.insert(0x3a, Box::new(OpcodeGasPrice));
        opcode_table.insert(0x3b, Box::new(OpcodeExtCodeSize));
        opcode_table.insert(0x3c, Box::new(OpcodeExtCodeCopy));
        opcode_table.insert(0x3d, Box::new(OpcodeReturnDataSize));
        opcode_table.insert(0x3e, Box::new(OpcodeReturnDataCopy));
        opcode_table.insert(0x3f, Box::new(OpcodeExtCodeHash));
        opcode_table.insert(0x40, Box::new(OpcodeBlockHash));
        opcode_table.insert(0x41, Box::new(OpcodeCoinbase));
        opcode_table.insert(0x42, Box::new(OpcodeTimestamp));
        opcode_table.insert(0x43, Box::new(OpcodeNumber));
        opcode_table.insert(0x44, Box::new(OpcodePrevRandao));
        opcode_table.insert(0x45, Box::new(OpcodeGasLimit));
        opcode_table.insert(0x46, Box::new(OpcodeChainId));
        opcode_table.insert(0x47, Box::new(OpcodeSelfBalance));
        opcode_table.insert(0x48, Box::new(OpcodeBaseFee));
        opcode_table.insert(0x49, Box::new(OpcodeBlobHash));
        opcode_table.insert(0x4a, Box::new(OpcodeBlobBaseFee));*/

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

        //opcode_table.insert(0xf0, Box::new(OpcodeCreate));
        //opcode_table.insert(0xf1, Box::new(OpcodeCall));
        //opcode_table.insert(0xf2, Box::new(OpcodeCallCode));
        opcode_table.insert(0xf3, Box::new(OpcodeReturn));
        //opcode_table.insert(0xf2, Box::new(OpcodeDelegateCall));
        //opcode_table.insert(0xf2, Box::new(OpcodeCreate2));
        //opcode_table.insert(0xfa, Box::new(OpcodeStaticCall));
        opcode_table.insert(0xfd, Box::new(OpcodeRevert));
        //opcode_table.insert(0xfa, Box::new(OpcodeSelfDestruct));

        while self.pc < self.code.len() {
            let opcode = self.code[self.pc];
            //println!("0x{:x}", opcode);
            if let Some(opcode_fn) = opcode_table.get(&opcode) {
                if let Some(res) = opcode_fn.call(ctx, &mut self, &code, call_info)? {
                    return Ok(res);
                }
            } else {
                return Err(XevmError::UnknownOpcode(opcode));
            }
        }
        Err(XevmError::DidntFinish)
    }

    pub fn mem_put(&mut self, offset: usize, data: &[u8]) {
        let expected_len = offset + data.len();
        if expected_len > self.memory.len() {
            self.memory.resize(expected_len, 0);
        }
        self.memory[offset..offset + data.len()].copy_from_slice(data);
    }
    pub fn pop_stack(&mut self) -> Result<U256, XevmError> {
        Ok(self
            .stack
            .pop()
            .ok_or(XevmError::Other("Stack empty!".into()))?)
    }
}
