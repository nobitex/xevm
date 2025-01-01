use crate::u256::U256;
use crate::CallInfo;
use crate::Context;
use crate::Machine;
use crate::OpcodeHandler;
use crate::XevmError;
use std::error::Error;


#[derive(Debug)]
pub struct OpcodePush(pub u8);
impl<C: Context> OpcodeHandler<C> for OpcodePush {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<(), Box<dyn Error>> {
        let ahead = &text[machine.pc + 1..];
        if ahead.len() < self.0 as usize {
            return Err(Box::new(XevmError::Other("Not enough bytes!".into())));
        }
        let mut reversed = ahead[..self.0 as usize].to_vec();
        reversed.reverse();

        machine.stack.push(U256::from_bytes(&reversed));
        machine.pc += 1 + self.0 as usize;
        Ok(())
    }
}
