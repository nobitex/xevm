/* Audited 11 Feb 2025 - Keyvan Kambakhsh */

use super::ExecutionResult;
use crate::error::ExecError;
use crate::machine::{CallInfo, Word};

use super::OpcodeHandler;
use crate::context::Context;
use crate::machine::Machine;

#[derive(Debug)]
pub struct OpcodePop;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodePop {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine.pop_stack()?;
        machine.pc += 1;
        Ok(None)
    }
}
