use crate::CallInfo;
use crate::ExecutionResult;
use crate::XevmError;
use std::error::Error;

use crate::Context;
use crate::Machine;
use crate::OpcodeHandler;

#[derive(Debug)]
pub struct OpcodeHalt;
impl<C: Context> OpcodeHandler<C> for OpcodeHalt {
    fn call(
        &self,
        _ctx: &mut C,
        _machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, Box<dyn Error>> {
        Ok(Some(ExecutionResult::Halted))
    }
}
