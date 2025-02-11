use super::ExecutionResult;
use crate::error::{ExecError, RevertError};
use crate::machine::{CallInfo, GasTracker, Word};

use super::OpcodeHandler;
use crate::context::{Context, ContextMut};
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
        if call_info.is_static {
            return Err(ExecError::Revert(RevertError::CannotMutateStatic));
        }
        let value = machine.pop_stack()?;
        let offset = machine.pop_stack()?.to_usize()?;
        let size = machine.pop_stack()?.to_usize()?;
        let salt = if self == &OpcodeCreate::Create2 {
            Some(machine.pop_stack()?)
        } else {
            None
        };
        let code = machine.mem_get(offset, size)?;
        let mut gas_tracker = GasTracker::new(machine.gas_tracker.remaining_gas());
        let stack_size = machine.stack_size - machine.stack.len();
        match ctx.as_mut().create(
            stack_size,
            &mut gas_tracker,
            CallInfo {
                origin: call_info.origin,
                caller: machine.address,
                value,
                data: code,
                is_static: call_info.is_static,
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
        machine.consume_gas(gas_tracker.gas_used)?;
        machine.pc += 1;
        Ok(None)
    }
}
