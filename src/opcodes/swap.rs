use super::ExecutionResult;
use super::OpcodeHandler;
use crate::error::XevmError;
use crate::machine::CallInfo;
use crate::machine::Context;
use crate::machine::Machine;

#[derive(Debug)]
pub struct OpcodeSwap(pub u8);
impl<C: Context> OpcodeHandler<C> for OpcodeSwap {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let stack_len = machine.stack.len();
        if self.0 as usize >= stack_len {
            return Err(XevmError::Other("Swap element doesn't exist!".into()));
        }
        let b = machine.pop_stack()?;
        let a = machine
            .stack
            .get_mut(stack_len - 2 - self.0 as usize)
            .ok_or(XevmError::Other("Swap element doesn't exist!".into()))?;
        let a_val = *a;
        *a = b;
        machine.stack.push(a_val);
        machine.pc += 1;
        Ok(None)
    }
}
