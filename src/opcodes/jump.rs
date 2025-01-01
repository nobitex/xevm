use anyhow::anyhow;

use crate::Context;
use crate::Machine;
use crate::OpcodeHandler;

#[derive(Debug)]
pub struct OpcodeJump;
impl<C: Context> OpcodeHandler<C> for OpcodeJump {
    fn call(&self, _ctx: &mut C, machine: &mut Machine, text: &[u8]) -> Result<(), anyhow::Error> {
        let target = machine.pop_stack()?.lower_usize();
        if target >= text.len() {
            return Err(anyhow!("Jump higher than code length!"));
        }
        if text[target] != 0x5b {
            return Err(anyhow!("Jump to a non-JUMPDEST target!"));
        }
        machine.pc = target;
        Ok(())
    }
}
