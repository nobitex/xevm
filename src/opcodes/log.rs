use super::ExecutionResult;
use super::OpcodeHandler;
use crate::error::XevmError;
use crate::machine::CallInfo;
use crate::machine::Context;
use crate::machine::Machine;

#[derive(Debug)]
pub struct OpcodeLog(pub u8);
impl<C: Context> OpcodeHandler<C> for OpcodeLog {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let offset = machine.pop_stack()?.as_usize()?;
        let size = machine.pop_stack()?.as_usize()?;
        let mut topics = Vec::new();
        for _ in 0..self.0 {
            topics.push(machine.pop_stack()?);
        }
        let data = machine.mem_get(offset, size);
        ctx.log(topics, data)?;
        machine.pc += 1;
        Ok(None)
    }
}
