use crate::Context;
use crate::Machine;
use crate::OpcodeHandler;

#[derive(Debug)]
pub struct OpcodeAdd;
impl<C: Context> OpcodeHandler<C> for OpcodeAdd {
    fn call(&self, ctx: &mut C, machine: &mut Machine, text: &[u8]) -> Result<(), anyhow::Error> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(a + b);
        machine.gas_used += 3;
        machine.pc += 1;
        Ok(())
    }
}