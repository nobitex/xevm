use std::error::Error;

use crate::CallInfo;
use crate::Context;
use crate::ExecutionResult;
use crate::Machine;
use crate::OpcodeHandler;
use crate::XevmError;

#[derive(Debug)]
pub struct OpcodeReturn;
impl<C: Context> OpcodeHandler<C> for OpcodeReturn {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, Box<dyn Error>> {
        let offset = machine.pop_stack()?.lower_usize();
        let sz = machine.pop_stack()?.lower_usize();
        let return_value = machine.memory[offset..offset + sz].to_vec();
        Ok(Some(ExecutionResult::Returned(return_value)))
    }
}
