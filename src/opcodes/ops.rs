use crate::u256::U256;
use crate::CallInfo;
use crate::Context;
use crate::ExecutionResult;
use crate::Machine;
use crate::OpcodeHandler;
use crate::XevmError;

#[derive(Debug)]
pub struct OpcodeAdd;
impl<C: Context> OpcodeHandler<C> for OpcodeAdd {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(a + b);
        machine.gas_used += 3;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeSub;
impl<C: Context> OpcodeHandler<C> for OpcodeSub {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(a - b);
        machine.gas_used += 3;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeMul;
impl<C: Context> OpcodeHandler<C> for OpcodeMul {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(a * b);
        machine.gas_used += 5;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeShl;
impl<C: Context> OpcodeHandler<C> for OpcodeShl {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let shift = machine.pop_stack()?;
        let val = machine.pop_stack()?;
        machine.stack.push(val << shift);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeShr;
impl<C: Context> OpcodeHandler<C> for OpcodeShr {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let shift = machine.pop_stack()?;
        let val = machine.pop_stack()?;
        machine.stack.push(val >> shift);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeAnd;
impl<C: Context> OpcodeHandler<C> for OpcodeAnd {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(a & b);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeOr;
impl<C: Context> OpcodeHandler<C> for OpcodeOr {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(a | b);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeXor;
impl<C: Context> OpcodeHandler<C> for OpcodeXor {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(a ^ b);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeNot;
impl<C: Context> OpcodeHandler<C> for OpcodeNot {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let a = machine.pop_stack()?;
        machine.stack.push(!a);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeSlt;
impl<C: Context> OpcodeHandler<C> for OpcodeSlt {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(U256::from((a < b) as u64));
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeSgt;
impl<C: Context> OpcodeHandler<C> for OpcodeSgt {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(U256::from((a > b) as u64));
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeByte;
impl<C: Context> OpcodeHandler<C> for OpcodeByte {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let i = machine.pop_stack()?.lower_usize();
        let x = machine.pop_stack()?.to_bytes_be();
        machine
            .stack
            .push(U256::from(if i < 32 { x[i] as u64 } else { 0 }));
        machine.pc += 1;
        Ok(None)
    }
}
