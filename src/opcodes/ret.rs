use super::ExecutionResult;
use super::OpcodeHandler;
use crate::context::Context;
use crate::error::ExecError;
use crate::machine::CallInfo;
use crate::machine::Machine;
use crate::machine::Word;

#[derive(Debug)]
pub struct OpcodeReturn;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeReturn {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,

        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let offset = machine.pop_stack()?.to_usize()?;
        let size = machine.pop_stack()?.to_usize()?;
        let return_value = machine.mem_get(offset, size);
        Ok(Some(ExecutionResult::Returned(return_value)))
    }
}
