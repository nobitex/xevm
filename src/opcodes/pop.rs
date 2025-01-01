use crate::CallInfo;
use crate::Context;
use crate::Machine;
use crate::OpcodeHandler;

#[derive(Debug)]
pub struct OpcodePop;
impl<C: Context> OpcodeHandler<C> for OpcodePop {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<(), anyhow::Error> {
        machine.pop_stack()?;
        machine.pc += 1;
        Ok(())
    }
}
