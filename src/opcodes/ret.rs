use super::ExecutionResult;
use super::OpcodeHandler;
use crate::context::Context;
use crate::error::ExecError;
use crate::machine::CallInfo;
use crate::machine::Machine;

#[derive(Debug)]
pub struct OpcodeReturn;
impl<C: Context> OpcodeHandler<C> for OpcodeReturn {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let offset = machine.pop_stack()?.as_usize()?;
        let size = machine.pop_stack()?.as_usize()?;
        let return_value = machine.mem_get(offset, size);
        Ok(Some(ExecutionResult::Returned(return_value)))
    }
}
