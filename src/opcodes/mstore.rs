use crate::CallInfo;
use crate::Context;
use crate::Machine;
use crate::OpcodeHandler;

#[derive(Debug)]
pub struct OpcodeMstore;
impl<C: Context> OpcodeHandler<C> for OpcodeMstore {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<(), anyhow::Error> {
        let addr = machine.pop_stack()?;
        let val = machine.pop_stack()?;
        ctx.mstore(addr, val)?;
        machine.pc += 1;
        Ok(())
    }
}
