use std::{collections::HashMap, fmt::Debug, u128};

mod u256;
use u256::U256;
mod opcodes;
use opcodes::*;

use anyhow::anyhow;

#[derive(Debug, Clone, Default)]
struct Machine {
    pc: usize,
    gas_used: usize,
    stack: Vec<U256>,
    transient: HashMap<U256, U256>,
}

impl Machine {
    fn pop_stack(&mut self) -> Result<U256, anyhow::Error> {
        self.stack.pop().ok_or(anyhow!("Stack empty!"))
    }
}

trait OpcodeHandler<C: Context> {
    fn call(&self, ctx: &mut C, machine: &mut Machine, text: &[u8]) -> Result<(), anyhow::Error>;
}

fn run<C: Context>(ctx: &mut C, machine: &mut Machine, code: &[u8]) -> Result<(), anyhow::Error> {
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
    opcode_table.insert(0x0b, Box::new(OpcodeSignextend));

    opcode_table.insert(0x10, Box::new(OpcodeLt));
    opcode_table.insert(0x11, Box::new(OpcodeGt));
    opcode_table.insert(0x12, Box::new(OpcodeSlt));
    opcode_table.insert(0x13, Box::new(OpcodeSgt));
    opcode_table.insert(0x14, Box::new(OpcodeEq));
    opcode_table.insert(0x15, Box::new(OpcodeIszero));
    opcode_table.insert(0x16, Box::new(OpcodeAnd));
    opcode_table.insert(0x17, Box::new(OpcodeOr));
    opcode_table.insert(0x18, Box::new(OpcodeXor));
    opcode_table.insert(0x19, Box::new(OpcodeNot));
    opcode_table.insert(0x1a, Box::new(OpcodeByte));
    opcode_table.insert(0x1b, Box::new(OpcodeShl));
    opcode_table.insert(0x1c, Box::new(OpcodeShr));
    opcode_table.insert(0x1d, Box::new(OpcodeSar));

    opcode_table.insert(0x30, Box::new(OpcodeAddress));
    opcode_table.insert(0x31, Box::new(OpcodeBalance));
    opcode_table.insert(0x32, Box::new(OpcodeOrigin));
    opcode_table.insert(0x33, Box::new(OpcodeCaller));
    opcode_table.insert(0x34, Box::new(OpcodeCallvalue));
    opcode_table.insert(0x35, Box::new(OpcodeCalldataload));
    opcode_table.insert(0x36, Box::new(OpcodeCalldatasize));
    opcode_table.insert(0x37, Box::new(OpcodeCalldatacopy));
    opcode_table.insert(0x38, Box::new(OpcodeCodesize));
    opcode_table.insert(0x39, Box::new(OpcodeCodecopy));
    opcode_table.insert(0x3a, Box::new(OpcodeGasprice));
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
    opcode_table.insert(0x56, Box::new(OpcodeJump));
    opcode_table.insert(0x5b, Box::new(OpcodeJumpdest));
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

    while machine.pc < code.len() {
        let opcode = code[machine.pc];
        if let Some(opcode_fn) = opcode_table.get(&opcode) {
            opcode_fn.call(ctx, machine, code)?;
        } else {
            return Err(anyhow!("Invalid opcode!"));
        }
    }
    Ok(())
}

trait Context {
    fn address(&self) -> Result<U256, anyhow::Error>;
    fn balance(&self, address: U256) -> Result<U256, anyhow::Error>;
    fn mload(&self, address: U256) -> Result<U256, anyhow::Error>;
    fn mstore(&mut self, address: U256, value: U256) -> Result<(), anyhow::Error>;
}

#[derive(Clone, Debug, Default)]
struct DummyContext {
    mem: HashMap<U256, U256>,
}
impl Context for DummyContext {
    fn balance(&self, address: U256) -> Result<U256, anyhow::Error> {
        Ok(U256::ONE)
    }
    fn address(&self) -> Result<U256, anyhow::Error> {
        Ok(U256::ONE)
    }
    fn mload(&self, address: U256) -> Result<U256, anyhow::Error> {
        Ok(self.mem.get(&address).copied().unwrap_or_default())
    }
    fn mstore(&mut self, address: U256, value: U256) -> Result<(), anyhow::Error> {
        self.mem.insert(address, value);
        Ok(())
    }
}

fn main() {
    let mut m = Machine::default();
    let mut ctx = DummyContext::default();
    m.stack.push(U256::from(123));
    m.stack.push(U256::from(234));
    m.stack.push(U256::from(345));
    m.stack.push(U256::from(456));
    //m.stack.push(u256::ONE);
    run(&mut ctx, &mut m, &[0x0]).unwrap();
    println!("{:?}", m);
    println!("{:?}", ctx);
}
