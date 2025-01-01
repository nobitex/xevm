use crate::Context;
use crate::Machine;
use crate::OpcodeHandler;

#[derive(Debug)]
pub struct OpcodeJumpdest;
impl<C: Context> OpcodeHandler<C> for OpcodeJumpdest {
    fn call(&self, _ctx: &mut C, machine: &mut Machine, _text: &[u8]) -> Result<(), anyhow::Error> {
        machine.pc += 1;
        Ok(())
    }
}
