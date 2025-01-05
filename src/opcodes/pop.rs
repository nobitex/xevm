use super::ExecutionResult;
use crate::error::ExecError;
use crate::machine::CallInfo;

use super::OpcodeHandler;
use crate::context::Context;
use crate::machine::Machine;

#[derive(Debug)]
pub struct OpcodePop;
impl<C: Context> OpcodeHandler<C> for OpcodePop {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine.pop_stack()?;
        machine.pc += 1;
        Ok(None)
    }
}
