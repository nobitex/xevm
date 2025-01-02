use super::ExecutionResult;
use super::OpcodeHandler;
use crate::error::XevmError;
use crate::machine::CallInfo;
use crate::machine::Context;
use crate::machine::Machine;
use crate::u256::U256;

#[derive(Debug)]
pub struct OpcodeJump;
impl<C: Context> OpcodeHandler<C> for OpcodeJump {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let target = machine.pop_stack()?.lower_usize();
        if target >= text.len() {
            return Err(XevmError::Other("Jump higher than code length!".into()));
        }
        if text[target] != 0x5b {
            return Err(XevmError::Other("Jump to a non-JUMPDEST target!".into()));
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
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
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
        text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let target = machine.pop_stack()?.lower_usize();
        let cond = machine.pop_stack()?;
        if target >= text.len() {
            return Err(XevmError::Other("Jump higher than code length!".into()));
        }
        if text[target] != 0x5b {
            return Err(XevmError::Other("Jump to a non-JUMPDEST target!".into()));
        }
        if cond != U256::ZERO {
            machine.pc = target;
        } else {
            machine.pc += 1;
        }
        Ok(None)
    }
}
