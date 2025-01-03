use super::ExecutionResult;
use super::OpcodeHandler;
use crate::error::XevmError;
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
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let offset = machine.pop_stack()?.as_usize()?;
        let sz = machine.pop_stack()?.as_usize()?;
        let revert_value = machine.memory[offset..offset + sz].to_vec();
        Ok(Some(ExecutionResult::Reverted(revert_value)))
    }
}
