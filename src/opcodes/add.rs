use crate::CallInfo;
use crate::Context;
use crate::ExecutionResult;
use crate::Machine;
use crate::OpcodeHandler;
use crate::XevmError;

#[derive(Debug)]
pub struct OpcodeAdd;
impl<C: Context> OpcodeHandler<C> for OpcodeAdd {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(a + b);
        machine.gas_used += 3;
        machine.pc += 1;
        Ok(None)
    }
}
