use super::ExecutionResult;
use crate::error::ExecError;
use crate::machine::{CallInfo, Word};

use super::OpcodeHandler;
use crate::context::Context;
use crate::machine::Machine;

#[derive(Debug)]
pub struct OpcodeCreate;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeCreate {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine<W>,
        call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let value = machine.pop_stack()?;
        let offset = machine.pop_stack()?.to_usize()?;
        let size = machine.pop_stack()?.to_usize()?;
        let code = machine.mem_get(offset, size);
        let addr = ctx.create(CallInfo {
            origin: call_info.origin,
            caller: call_info.caller,
            call_value: value,
            calldata: code,
        })?;
        machine.stack.push(W::from_addr(addr));
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCreate2;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeCreate2 {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine<W>,
        call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let value = machine.pop_stack()?;
        let offset = machine.pop_stack()?.to_usize()?;
        let size = machine.pop_stack()?.to_usize()?;
        let salt = machine.pop_stack()?;
        let code = machine.mem_get(offset, size);
        let addr = ctx.create2(
            CallInfo {
                origin: call_info.origin,
                caller: call_info.caller,
                call_value: value,
                calldata: code,
            },
            salt,
        )?;
        machine.stack.push(W::from_addr(addr));
        machine.pc += 1;
        Ok(None)
    }
}
