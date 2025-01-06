use super::ExecutionResult;
use crate::error::ExecError;
use crate::keccak::keccak;
use crate::machine::CallInfo;
use crate::u256::U256;

use super::OpcodeHandler;
use crate::context::Context;
use crate::machine::Machine;

#[derive(Debug)]
pub struct OpcodeKeccak;
impl<C: Context> OpcodeHandler<C> for OpcodeKeccak {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let offset = machine.pop_stack()?.to_usize()?;
        let size = machine.pop_stack()?.to_usize()?;
        let data = machine.mem_get(offset, size);
        let res = keccak(&data);
        machine.stack.push(U256::from_big_endian(&res));
        Ok(Some(ExecutionResult::Halted))
    }
}
