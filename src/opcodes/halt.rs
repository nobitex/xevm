use super::ExecutionResult;
use crate::error::ExecError;
use crate::machine::CallInfo;

use super::OpcodeHandler;
use crate::machine::Context;
use crate::machine::Machine;

#[derive(Debug)]
pub struct OpcodeHalt;
impl<C: Context> OpcodeHandler<C> for OpcodeHalt {
    fn call(
        &self,
        _ctx: &mut C,
        _machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        Ok(Some(ExecutionResult::Halted))
    }
}
