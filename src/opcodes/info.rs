use super::ExecutionResult;
use crate::error::ExecError;
use crate::machine::CallInfo;

use super::OpcodeHandler;
use crate::context::{Context, Info};
use crate::machine::Machine;

#[derive(Debug)]
pub struct OpcodeInfo(pub Info);
impl<C: Context> OpcodeHandler<C> for OpcodeInfo {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine,
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine.stack.push(ctx.info(self.0)?);
        Ok(None)
    }
}
