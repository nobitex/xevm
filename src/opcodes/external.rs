use std::error::Error;

use crate::u256::U256;
use crate::CallInfo;
use crate::Context;
use crate::Machine;
use crate::OpcodeHandler;

#[derive(Debug)]
pub struct OpcodeAddress;
impl<C: Context> OpcodeHandler<C> for OpcodeAddress {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<(), Box<dyn Error>> {
        machine.stack.push(ctx.address()?);
        machine.pc += 1;
        Ok(())
    }
}

#[derive(Debug)]
pub struct OpcodeBalance;
impl<C: Context> OpcodeHandler<C> for OpcodeBalance {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<(), Box<dyn Error>> {
        let addr = machine.pop_stack()?;
        machine.stack.push(ctx.balance(addr)?);
        machine.pc += 1;
        Ok(())
    }
}

#[derive(Debug)]
pub struct OpcodeCallvalue;
impl<C: Context> OpcodeHandler<C> for OpcodeCallvalue {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        call_info: &CallInfo,
    ) -> Result<(), Box<dyn Error>> {
        machine.stack.push(call_info.call_value);
        machine.pc += 1;
        Ok(())
    }
}

#[derive(Debug)]
pub struct OpcodeCaller;
impl<C: Context> OpcodeHandler<C> for OpcodeCaller {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        call_info: &CallInfo,
    ) -> Result<(), Box<dyn Error>> {
        machine.stack.push(call_info.caller);
        machine.pc += 1;
        Ok(())
    }
}

#[derive(Debug)]
pub struct OpcodeCodesize;
impl<C: Context> OpcodeHandler<C> for OpcodeCodesize {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<(), Box<dyn Error>> {
        machine.stack.push(U256::from(text.len() as u64));
        machine.pc += 1;
        Ok(())
    }
}

#[derive(Debug)]
pub struct OpcodeCodecopy;
impl<C: Context> OpcodeHandler<C> for OpcodeCodecopy {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<(), Box<dyn Error>> {
        unimplemented!();
        machine.pc += 1;
        Ok(())
    }
}

