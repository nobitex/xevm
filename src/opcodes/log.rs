use crate::CallInfo;
use crate::Context;
use crate::ExecutionResult;
use crate::Machine;
use crate::OpcodeHandler;
use crate::XevmError;

#[derive(Debug)]
pub struct OpcodeLog(pub u8);
impl<C: Context> OpcodeHandler<C> for OpcodeLog {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let offset = machine.pop_stack()?.lower_usize();
        let size = machine.pop_stack()?.lower_usize();
        let mut topics = Vec::new();
        for _ in 0..self.0 {
            topics.push(machine.pop_stack()?);
        }
        let data = machine.memory[offset..offset + size].to_vec();
        ctx.log(topics, data)?;
        machine.pc += 1;
        Ok(None)
    }
}
