use super::ExecutionResult;
use super::OpcodeHandler;
use crate::context::Context;
use crate::error::ExecError;
use crate::machine::CallInfo;
use crate::machine::Machine;
use crate::machine::Word;

#[derive(Debug)]
pub struct OpcodeLog(pub u8);
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeLog {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine<W>,

        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let offset = machine.pop_stack()?.to_usize()?;
        let size = machine.pop_stack()?.to_usize()?;
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
