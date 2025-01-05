use super::ExecutionResult;
use crate::error::ExecError;
use crate::machine::CallInfo;

use super::OpcodeHandler;
use crate::context::Context;
use crate::machine::Machine;

#[derive(Debug)]
pub struct OpcodeCreate;
impl<C: Context> OpcodeHandler<C> for OpcodeCreate {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine,
        call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let value = machine.pop_stack()?;
        let offset = machine.pop_stack()?.as_usize()?;
        let size = machine.pop_stack()?.as_usize()?;
        let code = machine.mem_get(offset, size);
        let addr = ctx.create(call_info.caller, value, code)?;
        machine.stack.push(addr);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCreate2;
impl<C: Context> OpcodeHandler<C> for OpcodeCreate2 {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine,
        call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let value = machine.pop_stack()?;
        let offset = machine.pop_stack()?.as_usize()?;
        let size = machine.pop_stack()?.as_usize()?;
        let salt = machine.pop_stack()?;
        let code = machine.mem_get(offset, size);
        let addr = ctx.create2(call_info.caller, value, code, salt)?;
        machine.stack.push(addr);
        machine.pc += 1;
        Ok(None)
    }
}
