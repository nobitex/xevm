use super::ExecutionResult;
use crate::error::ExecError;
use crate::machine::{CallInfo, Word};

use super::OpcodeHandler;
use crate::context::Context;
use crate::machine::Machine;

#[derive(Debug, PartialEq)]
pub enum OpcodeCreate {
    Create,
    Create2,
}
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
        let salt = if self == &OpcodeCreate::Create2 {
            Some(machine.pop_stack()?)
        } else {
            None
        };
        let code = machine.mem_get(offset, size);
        match ctx.create(
            machine.gas,
            CallInfo {
                origin: call_info.origin,
                caller: call_info.caller,
                call_value: value,
                calldata: code,
            },
            salt,
        ) {
            Ok(addr) => {
                machine.push_stack(W::from_addr(addr))?;
            }
            Err(e) => match e {
                ExecError::Revert(_) => {
                    machine.push_stack(W::ZERO)?;
                }
                ExecError::Context(ctx_err) => {
                    return Err(ExecError::Context(ctx_err));
                }
            },
        }
        machine.pc += 1;
        Ok(None)
    }
}
