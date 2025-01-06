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
        let offset = machine.pop_stack()?.to_usize()?;
        let size = machine.pop_stack()?.to_usize()?;
        let code = machine.mem_get(offset, size);
        let addr = ctx.create(CallInfo {
            origin: call_info.origin,
            caller: call_info.caller,
            call_value: value,
            calldata: code,
        })?;
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
        machine.stack.push(addr);
        machine.pc += 1;
        Ok(None)
    }
}
