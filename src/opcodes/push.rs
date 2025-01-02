use super::ExecutionResult;
use super::OpcodeHandler;
use crate::error::XevmError;
use crate::machine::CallInfo;
use crate::machine::Context;
use crate::machine::Machine;
use crate::u256::U256;

#[derive(Debug)]
pub struct OpcodePush(pub u8);
impl<C: Context> OpcodeHandler<C> for OpcodePush {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let ahead = &text[machine.pc + 1..];
        if ahead.len() < self.0 as usize {
            return Err(XevmError::Other("Not enough bytes!".into()));
        }

        machine
            .stack
            .push(U256::from_bytes_be(&ahead[..self.0 as usize]));
        machine.pc += 1 + self.0 as usize;
        Ok(None)
    }
}
