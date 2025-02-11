/* Audited 11 Feb 2025 - Keyvan Kambakhsh */

use super::ExecutionResult;
use super::OpcodeHandler;
use crate::context::Context;
use crate::error::ExecError;
use crate::error::RevertError;
use crate::machine::CallInfo;
use crate::machine::Machine;
use crate::machine::Word;

#[derive(Debug)]
pub struct OpcodeRevert;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeRevert {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let offset = machine.pop_stack()?.to_usize()?;
        let size = machine.pop_stack()?.to_usize()?;
        let revert_value = machine.mem_get(offset, size);
        Err(ExecError::Revert(RevertError::Revert(revert_value)))
    }
}
