use crate::u256::U256;
use crate::CallInfo;
use crate::Context;
use crate::ExecutionResult;
use crate::Machine;
use crate::OpcodeHandler;
use crate::XevmError;

#[derive(Debug)]
pub struct OpcodeLt;
impl<C: Context> OpcodeHandler<C> for OpcodeLt {
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
pub struct OpcodeGt;
impl<C: Context> OpcodeHandler<C> for OpcodeGt {
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
pub struct OpcodeEq;
impl<C: Context> OpcodeHandler<C> for OpcodeEq {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(U256::from((a == b) as u64));
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeIszero;
impl<C: Context> OpcodeHandler<C> for OpcodeIszero {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let a = machine.pop_stack()?;
        machine.stack.push(U256::from((a == U256::ZERO) as u64));
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
