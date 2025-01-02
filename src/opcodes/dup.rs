use super::ExecutionResult;
use super::OpcodeHandler;
use crate::error::XevmError;
use crate::machine::CallInfo;
use crate::machine::Context;
use crate::machine::Machine;

#[derive(Debug)]
pub struct OpcodeDup(pub u8);
impl<C: Context> OpcodeHandler<C> for OpcodeDup {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        if self.0 as usize >= machine.stack.len() {
            return Err(XevmError::Other("Dup element doesn't exist!".into()));
        }
        let elem = machine
            .stack
            .get(machine.stack.len() - 1 - self.0 as usize)
            .copied()
            .ok_or(XevmError::Other("Dup element doesn't exist!".into()))?;
        machine.stack.push(elem);
        machine.pc += 1;
        Ok(None)
    }
}
