use super::ExecutionResult;
use crate::error::ExecError;
use crate::error::RevertError;
use crate::machine::CallInfo;
use crate::u256::U256;

use super::OpcodeHandler;
use crate::context::Context;
use crate::machine::Machine;

#[derive(Debug, PartialEq)]
pub enum OpcodeCall {
    Call,
    DelegateCall,
    StaticCall,
}
impl<C: Context> OpcodeHandler<C> for OpcodeCall {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine,
        call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let mut new_call_info = call_info.clone();

        let gas = machine.pop_stack()?;
        let address = machine.pop_stack()?;
        if self == &OpcodeCall::Call {
            new_call_info.call_value = machine.pop_stack()?;
        }
        let args_offset = machine.pop_stack()?.as_usize()?;
        let args_size = machine.pop_stack()?.as_usize()?;
        let ret_offset = machine.pop_stack()?.as_usize()?;
        let ret_size = machine.pop_stack()?.as_usize()?;
        let args = machine.memory[args_offset..args_offset + args_size].to_vec();

        new_call_info.calldata = args;
        match self {
            OpcodeCall::DelegateCall => {
                new_call_info.caller = call_info.caller;
            }
            _ => {
                new_call_info.caller = machine.address;
            }
        }
        match ctx.call(gas, address, new_call_info) {
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
