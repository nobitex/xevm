use crate::Context;
use crate::Machine;
use crate::OpcodeHandler;

#[derive(Debug)]
pub struct OpcodeMload;
impl<C: Context> OpcodeHandler<C> for OpcodeMload {
    fn call(&self, ctx: &mut C, machine: &mut Machine, _text: &[u8]) -> Result<(), anyhow::Error> {
        let addr = machine.pop_stack()?;
        machine.stack.push(ctx.mload(addr)?);
        machine.pc += 1;
        Ok(())
    }
}
