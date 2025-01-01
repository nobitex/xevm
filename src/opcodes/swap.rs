use anyhow::anyhow;

use crate::CallInfo;
use crate::Context;
use crate::Machine;
use crate::OpcodeHandler;

#[derive(Debug)]
pub struct OpcodeSwap(pub u8);
impl<C: Context> OpcodeHandler<C> for OpcodeSwap {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<(), anyhow::Error> {
        let stack_len = machine.stack.len();
        if self.0 as usize >= stack_len {
            return Err(anyhow!("Swap element doesn't exist!"));
        }
        let b = machine.pop_stack()?;
        let a = machine
            .stack
            .get_mut(stack_len - 2 - self.0 as usize)
            .ok_or(anyhow!("Swap element doesn't exist!"))?;
        let a_val = *a;
        *a = b;
        machine.stack.push(a_val);
        machine.pc += 1;
        Ok(())
    }
}
