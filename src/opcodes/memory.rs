use super::ExecutionResult;
use super::OpcodeHandler;
use crate::error::ExecError;
use crate::machine::CallInfo;
use crate::machine::Context;
use crate::machine::Machine;
use crate::u256::U256;

#[derive(Debug)]
pub struct OpcodeSstore;
impl<C: Context> OpcodeHandler<C> for OpcodeSstore {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let addr = machine.pop_stack()?;
        let val = machine.pop_stack()?;
        ctx.sstore(addr, val)?;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeSload;
impl<C: Context> OpcodeHandler<C> for OpcodeSload {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let addr = machine.pop_stack()?;
        machine.stack.push(ctx.sload(addr)?);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeTstore;
impl<C: Context> OpcodeHandler<C> for OpcodeTstore {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let addr = machine.pop_stack()?;
        let val = machine.pop_stack()?;
        machine.transient.insert(addr, val);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeTload;
impl<C: Context> OpcodeHandler<C> for OpcodeTload {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let addr = machine.pop_stack()?;
        machine
            .stack
            .push(machine.transient.get(&addr).copied().unwrap_or_default());
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeMstore;
impl<C: Context> OpcodeHandler<C> for OpcodeMstore {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let addr = machine.pop_stack()?.as_usize()?;
        let val = machine.pop_stack()?.to_bytes_be();
        machine.mem_put(addr, &val);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeMload;
impl<C: Context> OpcodeHandler<C> for OpcodeMload {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let addr = machine.pop_stack()?.as_usize()?;
        let ret = machine.mem_get(addr, 32);
        machine.stack.push(U256::from_bytes_be(&ret));
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeMstore8;
impl<C: Context> OpcodeHandler<C> for OpcodeMstore8 {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let addr = machine.pop_stack()?.as_usize()?;
        let val = machine.pop_stack()?.as_usize()?;
        machine.mem_put(addr, &[val as u8]);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeMcopy;
impl<C: Context> OpcodeHandler<C> for OpcodeMcopy {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let dest_offset = machine.pop_stack()?.as_usize()?;
        let offset = machine.pop_stack()?.as_usize()?;
        let size = machine.pop_stack()?.as_usize()?;
        let data = machine.mem_get(offset, size);
        machine.mem_put(dest_offset, &data);
        machine.pc += 1;
        Ok(None)
    }
}
