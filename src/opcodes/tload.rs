use crate::Context;
use crate::Machine;
use crate::OpcodeHandler;

#[derive(Debug)]
pub struct OpcodeTload;
impl<C: Context> OpcodeHandler<C> for OpcodeTload {
    fn call(&self, _ctx: &mut C, machine: &mut Machine, text: &[u8]) -> Result<(), anyhow::Error> {
        let addr = machine.pop_stack()?;
        machine
            .stack
            .push(machine.transient.get(&addr).copied().unwrap_or_default());
        machine.pc += 1;
        Ok(())
    }
}
