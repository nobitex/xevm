use super::ExecutionResult;
use crate::error::ExecError;
use crate::keccak::keccak;
use crate::machine::{CallInfo, Word};

use super::OpcodeHandler;
use crate::context::Context;
use crate::machine::Machine;

#[derive(Debug)]
pub struct OpcodeKeccak;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeKeccak {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let offset = machine.pop_stack()?.to_usize()?;
        let size = machine.pop_stack()?.to_usize()?;
        let data = machine.mem_get(offset, size);
        let res = keccak(&data);
        machine.push_stack(W::from_big_endian(&res))?;
        machine.pc += 1;
        Ok(None)
    }
}
