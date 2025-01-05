use super::ExecutionResult;
use super::OpcodeHandler;
use crate::context::Context;
use crate::error::ExecError;
use crate::error::RevertError;
use crate::machine::CallInfo;
use crate::machine::Machine;
use crate::u256::U256;

#[derive(Debug)]
pub struct OpcodeJump;
impl<C: Context> OpcodeHandler<C> for OpcodeJump {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let target = machine.pop_stack()?.as_usize()?;
        if target >= machine.code.len() {
            return Err(ExecError::Revert(RevertError::InvalidJump));
        }
        if machine.code[target] != 0x5b {
            return Err(ExecError::Revert(RevertError::InvalidJump));
        }
        machine.pc = target;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeJumpDest;
impl<C: Context> OpcodeHandler<C> for OpcodeJumpDest {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeJumpi;
impl<C: Context> OpcodeHandler<C> for OpcodeJumpi {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let target = machine.pop_stack()?.as_usize()?;
        let cond = machine.pop_stack()?;
        if target >= machine.code.len() {
            return Err(ExecError::Revert(RevertError::InvalidJump));
        }
        if machine.code[target] != 0x5b {
            return Err(ExecError::Revert(RevertError::InvalidJump));
        }
        if cond != U256::ZERO {
            machine.pc = target;
        } else {
            machine.pc += 1;
        }
        Ok(None)
    }
}
