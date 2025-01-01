use anyhow::anyhow;

use crate::Context;
use crate::Machine;
use crate::OpcodeHandler;

#[derive(Debug)]
pub struct OpcodeHalt;
impl<C: Context> OpcodeHandler<C> for OpcodeHalt {
    fn call(&self, _ctx: &mut C, _machine: &mut Machine, _text: &[u8]) -> Result<(), anyhow::Error> {
        Err(anyhow!("Halt!"))
    }
}
