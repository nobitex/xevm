/* Audited 11 Feb 2025 - Keyvan Kambakhsh */

use super::ExecutionResult;
use super::OpcodeHandler;
use crate::context::Context;
use crate::error::ExecError;
use crate::error::RevertError;
use crate::machine::CallInfo;
use crate::machine::Machine;
use crate::machine::Word;

#[derive(Debug)]
pub struct OpcodeJump;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeJump {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let target = machine.pop_stack()?.to_usize()?;
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
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeJumpDest {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeJumpi;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeJumpi {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let target = machine.pop_stack()?.to_usize()?;
        let cond = machine.pop_stack()?;
        if target >= machine.code.len() {
            return Err(ExecError::Revert(RevertError::InvalidJump));
        }
        if machine.code[target] != 0x5b {
            return Err(ExecError::Revert(RevertError::InvalidJump));
        }
        if cond != W::ZERO {
            machine.pc = target;
        } else {
            machine.pc += 1;
        }
        Ok(None)
    }
}
