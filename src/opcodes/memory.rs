use super::ExecutionResult;
use super::OpcodeHandler;
use crate::context::Context;
use crate::error::ExecError;
use crate::machine::CallInfo;
use crate::machine::Machine;
use crate::machine::Word;
use crate::u256::U256;

#[derive(Debug)]
pub struct OpcodeSstore;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeSstore {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine<W>,

        _call_info: &CallInfo<W>,
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
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeSload {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine<W>,

        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let addr = machine.pop_stack()?;
        machine.stack.push(ctx.sload(addr)?);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeTstore;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeTstore {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,

        _call_info: &CallInfo<W>,
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
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeTload {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,

        _call_info: &CallInfo<W>,
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
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeMstore {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,

        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let addr = machine.pop_stack()?.to_usize()?;
        let val = machine.pop_stack()?.to_big_endian();
        machine.mem_put(addr, &val);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeMload;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeMload {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,

        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let addr = machine.pop_stack()?.to_usize()?;
        let ret = machine.mem_get(addr, 32);
        machine.stack.push(W::from_big_endian(&ret));
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeMstore8;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeMstore8 {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,

        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let addr = machine.pop_stack()?.to_usize()?;
        let val = machine.pop_stack()?.to_usize()?;
        machine.mem_put(addr, &[val as u8]);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeMcopy;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeMcopy {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,

        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let dest_offset = machine.pop_stack()?.to_usize()?;
        let offset = machine.pop_stack()?.to_usize()?;
        let size = machine.pop_stack()?.to_usize()?;
        let data = machine.mem_get(offset, size);
        machine.mem_put(dest_offset, &data);
        machine.pc += 1;
        Ok(None)
    }
}
