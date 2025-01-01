use anyhow::anyhow;

use crate::u256::U256;
use crate::CallInfo;
use crate::Context;
use crate::Machine;
use crate::OpcodeHandler;

#[derive(Debug)]
pub struct OpcodeJump;
impl<C: Context> OpcodeHandler<C> for OpcodeJump {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<(), anyhow::Error> {
        let target = machine.pop_stack()?.lower_usize();
        if target >= text.len() {
            return Err(anyhow!("Jump higher than code length!"));
        }
        if text[target] != 0x5b {
            return Err(anyhow!("Jump to a non-JUMPDEST target!"));
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
    ) -> Result<(), anyhow::Error> {
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
    ) -> Result<(), anyhow::Error> {
        let target = machine.pop_stack()?.lower_usize();
        let cond = machine.pop_stack()?;
        if target >= text.len() {
            return Err(anyhow!("Jump higher than code length!"));
        }
        if text[target] != 0x5b {
            return Err(anyhow!("Jump to a non-JUMPDEST target!"));
        }
        if cond != U256::ZERO {
            machine.pc = target;
        } else {
            machine.pc += 1;
        }
        Ok(())
    }
}
