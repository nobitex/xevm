use anyhow::anyhow;

use crate::Context;
use crate::Machine;
use crate::OpcodeHandler;

#[derive(Debug)]
pub struct OpcodeReturn;
impl<C: Context> OpcodeHandler<C> for OpcodeReturn {
    fn call(&self, _ctx: &mut C, machine: &mut Machine, _text: &[u8]) -> Result<(), anyhow::Error> {
        let offset = machine.pop_stack()?;
        let sz = machine.pop_stack()?;
        Err(anyhow!("Returned!"))
    }
}
