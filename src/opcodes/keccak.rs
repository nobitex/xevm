use super::ExecutionResult;
use crate::error::XevmError;
use crate::keccak::keccak;
use crate::machine::CallInfo;
use crate::u256::U256;

use super::OpcodeHandler;
use crate::machine::Context;
use crate::machine::Machine;

#[derive(Debug)]
pub struct OpcodeKeccak;
impl<C: Context> OpcodeHandler<C> for OpcodeKeccak {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let offset = machine.pop_stack()?.as_usize()?;
        let size = machine.pop_stack()?.as_usize()?;
        let data = machine.mem_get(offset, size);
        let res = keccak(&data);
        machine.stack.push(U256::from_bytes_be(&res));
        Ok(Some(ExecutionResult::Halted))
    }
}
