use super::ExecutionResult;
use super::OpcodeHandler;
use crate::context::Context;
use crate::error::ExecError;
use crate::error::RevertError;
use crate::machine::CallInfo;
use crate::machine::Machine;
use crate::u256::U256;

#[derive(Debug)]
pub struct OpcodePush(pub u8);
impl<C: Context> OpcodeHandler<C> for OpcodePush {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        if machine.code.len() - machine.pc < self.0 as usize {
            return Err(ExecError::Revert(RevertError::NotEnoughBytesInCode));
        }
        machine.stack.push(if self.0 == 0 {
            U256::zero()
        } else {
            U256::from_big_endian(&machine.code[machine.pc + 1..machine.pc + 1 + self.0 as usize])
        });
        machine.pc += 1 + self.0 as usize;
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::u256::U256;

    use super::super::tests::test;
    use super::*;

    #[test]
    fn test_opcode_push() {
        test(
            OpcodePush(0),
            &[
                (&[], Some(&[U256::from(0)])),
                (&[U256::from(123)], Some(&[U256::from(0), U256::from(123)])),
            ],
        );
    }
}
