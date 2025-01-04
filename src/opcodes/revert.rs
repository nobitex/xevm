use super::ExecutionResult;
use super::OpcodeHandler;
use crate::error::ExecError;
use crate::error::RevertError;
use crate::machine::CallInfo;
use crate::machine::Context;
use crate::machine::Machine;

#[derive(Debug)]
pub struct OpcodeRevert;
impl<C: Context> OpcodeHandler<C> for OpcodeRevert {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let offset = machine.pop_stack()?.as_usize()?;
        let size = machine.pop_stack()?.as_usize()?;
        let revert_value = machine.mem_get(offset, size);
        Err(ExecError::Revert(RevertError::Revert(revert_value)))
    }
}
