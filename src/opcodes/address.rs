use crate::Context;
use crate::Machine;
use crate::OpcodeHandler;

#[derive(Debug)]
pub struct OpcodeAddress;
impl<C: Context> OpcodeHandler<C> for OpcodeAddress {
    fn call(&self, ctx: &mut C, machine: &mut Machine, _text: &[u8]) -> Result<(), anyhow::Error> {
        machine.stack.push(ctx.address()?);
        Ok(())
    }
}
