/* Audited 11 Feb 2025 - Keyvan Kambakhsh */

use super::ExecutionResult;
use super::OpcodeHandler;
use crate::context::Context;
use crate::context::ContextMut;
use crate::error::ExecError;
use crate::error::RevertError;
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
        call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        if call_info.is_static {
            return Err(ExecError::Revert(RevertError::CannotMutateStatic));
        }
        let offset = machine.pop_stack()?.to_usize()?;
        let size = machine.pop_stack()?.to_usize()?;
        let mut topics = Vec::new();
        for _ in 0..self.0 {
            machine.consume_gas(375)?;
            topics.push(machine.pop_stack()?);
        }
        let data = machine.mem_get(offset, size)?;
        ctx.as_mut().log(machine.address, topics, data)?;
        machine.pc += 1;
        Ok(None)
    }
}
