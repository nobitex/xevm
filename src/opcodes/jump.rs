use std::error::Error;

use crate::u256::U256;
use crate::CallInfo;
use crate::Context;
use crate::Machine;
use crate::OpcodeHandler;
use crate::XevmError;

#[derive(Debug)]
pub struct OpcodeJump;
impl<C: Context> OpcodeHandler<C> for OpcodeJump {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<(), Box<dyn Error>> {
        let target = machine.pop_stack()?.lower_usize();
        if target >= text.len() {
            return Err(Box::new(XevmError::Other(
                "Jump higher than code length!".into(),
            )));
        }
        if text[target] != 0x5b {
            return Err(Box::new(XevmError::Other(
                "Jump to a non-JUMPDEST target!".into(),
            )));
        }
        machine.pc = target;
        Ok(())
    }
}

#[derive(Debug)]
pub struct OpcodeJumpdest;
impl<C: Context> OpcodeHandler<C> for OpcodeJumpdest {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<(), Box<dyn Error>> {
        machine.pc += 1;
        Ok(())
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
    ) -> Result<(), Box<dyn Error>> {
        let target = machine.pop_stack()?.lower_usize();
        let cond = machine.pop_stack()?;
        if target >= text.len() {
            return Err(Box::new(XevmError::Other(
                "Jump higher than code length!".into(),
            )));
        }
        if text[target] != 0x5b {
            return Err(Box::new(XevmError::Other(
                "Jump to a non-JUMPDEST target!".into(),
            )));
        }
        if cond != U256::ZERO {
            machine.pc = target;
        } else {
            machine.pc += 1;
        }
        Ok(())
    }
}
