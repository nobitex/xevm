use std::error::Error;

use crate::CallInfo;
use crate::Context;
use crate::ExecutionResult;
use crate::Machine;
use crate::OpcodeHandler;
use crate::XevmError;

#[derive(Debug)]
pub struct OpcodeDup(pub u8);
impl<C: Context> OpcodeHandler<C> for OpcodeDup {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, Box<dyn Error>> {
        if self.0 as usize >= machine.stack.len() {
            return Err(Box::new(XevmError::Other(
                "Dup element doesn't exist!".into(),
            )));
        }
        let elem = machine
            .stack
            .get(machine.stack.len() - 1 - self.0 as usize)
            .copied()
            .ok_or(Box::new(XevmError::Other(
                "Dup element doesn't exist!".into(),
            )))?;
        machine.stack.push(elem);
        machine.pc += 1;
        Ok(None)
    }
}
