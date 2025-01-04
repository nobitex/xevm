use super::ExecutionResult;
use crate::error::ExecError;
use crate::error::RevertError;
use crate::machine::CallInfo;
use crate::u256::U256;

use super::OpcodeHandler;
use crate::machine::Context;
use crate::machine::Machine;

#[derive(Debug)]
pub struct OpcodeCall;
impl<C: Context> OpcodeHandler<C> for OpcodeCall {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine,
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let gas = machine.pop_stack()?;
        let address = machine.pop_stack()?;
        let value: U256 = machine.pop_stack()?;
        let args_offset = machine.pop_stack()?.as_usize()?;
        let args_size = machine.pop_stack()?.as_usize()?;
        let ret_offset = machine.pop_stack()?.as_usize()?;
        let ret_size = machine.pop_stack()?.as_usize()?;
        let args = machine.memory[args_offset..args_offset + args_size].to_vec();
        match ctx.call(gas, address, value, args) {
            Ok(exec_result) => match exec_result {
                ExecutionResult::Halted => {
                    machine.stack.push(U256::ONE);
                }
                ExecutionResult::Returned(ret) => {
                    machine.mem_put(ret_offset, &ret[..ret_size]);
                    machine.stack.push(U256::ONE);
                }
            },
            Err(e) => match e {
                ExecError::Context(e) => {
                    return Err(ExecError::Context(e));
                }
                ExecError::Revert(e) => {
                    if let RevertError::Revert(ret) = e {
                        machine.mem_put(ret_offset, &ret[..ret_size]);
                    }
                    machine.stack.push(U256::ZERO);
                }
            },
        }
        machine.pc += 1;
        Ok(None)
    }
}
