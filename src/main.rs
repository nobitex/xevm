use std::error::Error;
use std::{collections::HashMap, fmt::Debug};
mod u256;
use u256::U256;
mod opcodes;
use opcodes::*;

#[derive(Debug, Clone)]
enum ExecutionResult {
    Reverted(Vec<u8>),
    Returned(Vec<u8>),
    Halted,
}

#[derive(Debug)]
enum XevmError {
    UnknownOpcode(u8),
    DidntFinish,
    Other(Box<dyn Error>),
}

impl From<Box<dyn Error>> for XevmError {
    fn from(value: Box<dyn Error>) -> Self {
        Self::Other(value)
    }
}

impl std::fmt::Display for XevmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for XevmError {
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            XevmError::Other(parent) => Some(parent.as_ref()),
            _ => None,
        }
    }
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            XevmError::Other(parent) => Some(parent.as_ref()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Machine {
    code: Vec<u8>,
    pc: usize,
    gas_used: usize,
    stack: Vec<U256>,
    memory: Vec<u8>,
    transient: HashMap<U256, U256>,
}

impl Machine {
    fn new(code: Vec<u8>) -> Self {
        Self {
            code,
            pc: 0,
            gas_used: 0,
            stack: Vec::new(),
            memory: Vec::new(),
            transient: HashMap::new(),
        }
    }
    fn run<C: Context>(
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
        opcode_table.insert(0x08, Box::new(OpcodeAddmod));
        opcode_table.insert(0x09, Box::new(OpcodeMulmod));
        opcode_table.insert(0x0a, Box::new(OpcodeExp));
        opcode_table.insert(0x0b, Box::new(OpcodeSignextend));*/

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
        /*opcode_table.insert(0x3a, Box::new(OpcodeGasprice));
        opcode_table.insert(0x3b, Box::new(OpcodeExtcodesize));
        opcode_table.insert(0x3c, Box::new(OpcodeExtcodecopy));
        opcode_table.insert(0x3d, Box::new(OpcodeReturndatasize));
        opcode_table.insert(0x3e, Box::new(OpcodeReturndatacopy));
        opcode_table.insert(0x3f, Box::new(OpcodeExtcodehash));
        opcode_table.insert(0x40, Box::new(OpcodeBlockhash));
        opcode_table.insert(0x41, Box::new(OpcodeCoinbase));
        opcode_table.insert(0x42, Box::new(OpcodeTimestamp));
        opcode_table.insert(0x43, Box::new(OpcodeNumber));
        opcode_table.insert(0x44, Box::new(OpcodePrevrandao));
        opcode_table.insert(0x45, Box::new(OpcodeGaslimit));
        opcode_table.insert(0x46, Box::new(OpcodeChainid));
        opcode_table.insert(0x47, Box::new(OpcodeSelfbalance));
        opcode_table.insert(0x48, Box::new(OpcodeBasefee));
        opcode_table.insert(0x49, Box::new(OpcodeBlobhash));
        opcode_table.insert(0x4a, Box::new(OpcodeBlobbasefee));*/

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
        opcode_table.insert(0xf3, Box::new(OpcodeReturn));
        opcode_table.insert(0xfd, Box::new(OpcodeRevert));

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

    fn mem_put(&mut self, offset: usize, data: &[u8]) {
        let expected_len = offset + data.len();
        if expected_len > self.memory.len() {
            self.memory.resize(expected_len, 0);
        }
        self.memory[offset..offset + data.len()].copy_from_slice(data);
    }
    fn pop_stack(&mut self) -> Result<U256, XevmError> {
        Ok(self
            .stack
            .pop()
            .ok_or(XevmError::Other("Stack empty!".into()))?)
    }
}

trait OpcodeHandler<C: Context> {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine,
        text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError>;
}

trait Context {
    fn address(&self) -> Result<U256, Box<dyn Error>>;
    fn balance(&self, address: U256) -> Result<U256, Box<dyn Error>>;
    fn sload(&self, address: U256) -> Result<U256, Box<dyn Error>>;
    fn sstore(&mut self, address: U256, value: U256) -> Result<(), Box<dyn Error>>;
}

struct CallInfo {
    pub origin: U256,
    pub caller: U256,
    pub call_value: U256,
    pub calldata: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
struct DummyContext {
    mem: HashMap<U256, U256>,
}
impl Context for DummyContext {
    fn balance(&self, address: U256) -> Result<U256, Box<dyn Error>> {
        Ok(U256::ONE)
    }
    fn address(&self) -> Result<U256, Box<dyn Error>> {
        Ok(U256::ONE)
    }
    fn sload(&self, address: U256) -> Result<U256, Box<dyn Error>> {
        Ok(self.mem.get(&address).copied().unwrap_or_default())
    }
    fn sstore(&mut self, address: U256, value: U256) -> Result<(), Box<dyn Error>> {
        self.mem.insert(address, value);
        Ok(())
    }
}

fn parse_hex(s: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut ret = Vec::new();
    for i in (0..s.len()).step_by(2) {
        ret.push(u8::from_str_radix(&s[i..i + 2], 16)?);
    }
    Ok(ret)
}

fn main() {
    let code = parse_hex("6080604052348015600e575f80fd5b5060043610603a575f3560e01c80633fb5c1cb14603e5780638381f58a14604f578063d09de08a146068575b5f80fd5b604d6049366004607d565b5f55565b005b60565f5481565b60405190815260200160405180910390f35b604d5f805490806076836093565b9190505550565b5f60208284031215608c575f80fd5b5035919050565b5f6001820160af57634e487b7160e01b5f52601160045260245ffd5b506001019056fea264697066735822122055d88f9afbd1174cf472eb6254c3e131741fcc6117353bafc4aa81bf1af88e0264736f6c634300081a0033").unwrap();
    let mut ctx = DummyContext::default();
    let call_info = CallInfo {
        origin: U256::ZERO,
        call_value: U256::ZERO,
        caller: U256::ZERO,
        calldata: vec![0xd0, 0x9d, 0xe0, 0x8a],
    };
    let res = Machine::new(code.clone())
        .run(&mut ctx, &call_info)
        .unwrap();
    println!("{:?}", res);
    let res = Machine::new(code.clone())
        .run(&mut ctx, &call_info)
        .unwrap();
    println!("{:?}", res);
    let res = Machine::new(code.clone())
        .run(&mut ctx, &call_info)
        .unwrap();
    println!("{:?}", res);

    let call_info = CallInfo {
        origin: U256::ZERO,
        call_value: U256::ZERO,
        caller: U256::ZERO,
        calldata: vec![
            0x3f, 0xb5, 0xc1, 0xcb, 0xf7, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77,
            0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77,
            0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x7f,
        ],
    };
    let res = Machine::new(code.clone())
        .run(&mut ctx, &call_info)
        .unwrap();
    println!("{:?}", res);
    println!("{:?}", ctx);
}
