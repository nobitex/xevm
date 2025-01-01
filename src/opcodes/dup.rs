use anyhow::anyhow;

use crate::Context;
use crate::Machine;
use crate::OpcodeHandler;

#[derive(Debug)]
pub struct OpcodeDup(pub u8);
impl<C: Context> OpcodeHandler<C> for OpcodeDup {
    fn call(&self, ctx: &mut C, machine: &mut Machine, text: &[u8]) -> Result<(), anyhow::Error> {
        if self.0 as usize >= machine.stack.len() {
            return Err(anyhow!("Dup element doesn't exist!"));
        }
        let elem = machine
            .stack
            .get(machine.stack.len() - 1 - self.0 as usize)
            .copied()
            .ok_or(anyhow!("Dup element doesn't exist!"))?;
        machine.stack.push(elem);
        machine.pc += 1;
        Ok(())
    }
}
