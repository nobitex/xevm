use crate::Context;
use crate::Machine;
use crate::OpcodeHandler;

#[derive(Debug)]
pub struct OpcodeTstore;
impl<C: Context> OpcodeHandler<C> for OpcodeTstore {
    fn call(&self, _ctx: &mut C, machine: &mut Machine, text: &[u8]) -> Result<(), anyhow::Error> {
        let addr = machine.pop_stack()?;
        let val = machine.pop_stack()?;
        machine.transient.insert(addr, val);
        machine.pc += 1;
        Ok(())
    }
}
