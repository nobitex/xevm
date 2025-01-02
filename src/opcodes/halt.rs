use super::ExecutionResult;
use crate::error::XevmError;
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
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        Ok(Some(ExecutionResult::Halted))
    }
}
