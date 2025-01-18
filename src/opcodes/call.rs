use super::ExecutionResult;
use crate::context::ContextMut;
use crate::error::ExecError;
use crate::error::RevertError;
use crate::machine::CallInfo;
use crate::machine::GasTracker;
use crate::machine::Word;

use super::OpcodeHandler;
use crate::context::Context;
use crate::machine::Machine;

#[derive(Debug, PartialEq)]
pub enum OpcodeCall {
    Call,
    DelegateCall,
    StaticCall,
}
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeCall {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine<W>,
        call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let is_static = call_info.is_static || self == &OpcodeCall::StaticCall;
        if is_static && call_info.call_value != W::ZERO {
            return Err(ExecError::Revert(RevertError::CannotMutateStatic));
        }

        let mut new_call_info = call_info.clone();

        let gas = machine.pop_stack()?.to_usize()?;
        let address = machine.pop_stack()?.to_addr();
        if self == &OpcodeCall::Call {
            new_call_info.call_value = machine.pop_stack()?;
        }
        let args_offset = machine.pop_stack()?.to_usize()?;
        let args_size = machine.pop_stack()?.to_usize()?;
        let ret_offset = machine.pop_stack()?.to_usize()?;
        let ret_size = machine.pop_stack()?.to_usize()?;
        let args = machine.mem_get(args_offset, args_size);

        new_call_info.calldata = args;
        new_call_info.caller = match self {
            OpcodeCall::DelegateCall => call_info.caller,
            _ => machine.address,
        };
        new_call_info.is_static = is_static;

        let mut gas_tracker = GasTracker::new(gas);
        match ctx.as_mut().call(&mut gas_tracker, address, new_call_info) {
            Ok(exec_result) => match exec_result {
                ExecutionResult::Halted => {
                    machine.last_return = Some(vec![]);
                    machine.push_stack(W::ONE)?;
                }
                ExecutionResult::Returned(ret) => {
                    machine.mem_put(ret_offset, &ret, 0, ret_size)?;
                    machine.last_return = Some(ret);
                    machine.push_stack(W::ONE)?;
                }
            },
            Err(e) => match e {
                ExecError::Context(e) => {
                    return Err(ExecError::Context(e));
                }
                ExecError::Revert(e) => {
                    if let RevertError::Revert(ret) = e {
                        machine.mem_put(ret_offset, &ret, 0, ret_size)?;
                        machine.last_return = Some(ret);
                    } else {
                        machine.last_return = Some(vec![]);
                    }
                    machine.push_stack(W::ZERO)?;
                }
            },
        }
        machine.consume_gas(gas_tracker.gas_used)?;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeReturnDataSize;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeReturnDataSize {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        if let Some(dat) = &machine.last_return {
            machine.push_stack(W::from_u64(dat.len() as u64))?;
        } else {
            return Err(ExecError::Revert(RevertError::ReturnDataUnavailable));
        }
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeReturnDataCopy;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeReturnDataCopy {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        if let Some(dat) = machine.last_return.clone() {
            let dest_addr = machine.pop_stack()?.to_usize()?;
            let addr = machine.pop_stack()?.to_usize()?;
            let size = machine.pop_stack()?.to_usize()?;
            machine.mem_put(dest_addr, &dat, addr, size)?;
        } else {
            return Err(ExecError::Revert(RevertError::ReturnDataUnavailable));
        }
        machine.pc += 1;
        Ok(None)
    }
}
