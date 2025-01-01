use crate::Context;
use crate::Machine;
use crate::OpcodeHandler;

#[derive(Debug)]
pub struct OpcodeBalance;
impl<C: Context> OpcodeHandler<C> for OpcodeBalance {
    fn call(&self, ctx: &mut C, machine: &mut Machine, _text: &[u8]) -> Result<(), anyhow::Error> {
        let addr = machine.pop_stack()?;
        machine.stack.push(ctx.balance(addr)?);
        Ok(())
    }
}
