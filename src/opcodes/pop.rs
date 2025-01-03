use super::ExecutionResult;
use crate::error::XevmError;
use crate::machine::CallInfo;

use super::OpcodeHandler;
use crate::machine::Context;
use crate::machine::Machine;

#[derive(Debug)]
pub struct OpcodePop;
impl<C: Context> OpcodeHandler<C> for OpcodePop {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        machine.pop_stack()?;
        machine.pc += 1;
        Ok(None)
    }
}
