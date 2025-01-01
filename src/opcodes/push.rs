use anyhow::anyhow;

use crate::u256::U256;
use crate::Context;
use crate::Machine;
use crate::OpcodeHandler;

#[derive(Debug)]
pub struct OpcodePush(pub u8);
impl<C: Context> OpcodeHandler<C> for OpcodePush {
    fn call(&self, _ctx: &mut C, machine: &mut Machine, text: &[u8]) -> Result<(), anyhow::Error> {
        let ahead = &text[machine.pc + 1..];
        if ahead.len() < self.0 as usize {
            return Err(anyhow!("Not enough bytes!"));
        }
        machine
            .stack
            .push(U256::from_bytes(&ahead[..self.0 as usize]));
        machine.pc += 1 + self.0 as usize;
        Ok(())
    }
}
