use super::ExecutionResult;
use crate::error::ExecError;
use crate::machine::{CallInfo, Word};

use super::OpcodeHandler;
use crate::context::{Context, Info};
use crate::machine::Machine;

#[derive(Debug)]
pub struct OpcodeInfo(pub Info);
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeInfo {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine.push_stack(ctx.info(self.0)?)?;
        machine.pc += 1;
        Ok(None)
    }
}
