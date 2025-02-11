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
pub struct OpcodeDup(pub u8);
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeDup {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,

        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        if self.0 as usize >= machine.stack.len() {
            return Err(ExecError::Revert(RevertError::NotEnoughValuesOnStack));
        }
        let elem = machine
            .stack
            .get(machine.stack.len() - 1 - self.0 as usize)
            .copied()
            .ok_or(ExecError::Revert(RevertError::NotEnoughValuesOnStack))?;
        machine.push_stack(elem)?;
        machine.pc += 1;
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::u256::U256;

    use super::super::tests::test;
    use super::*;

    #[test]
    fn test_opcode_dup() {
        test(
            OpcodeDup(0),
            &[
                (&[], None),
                (
                    &[U256::from(123)],
                    Some(&[U256::from(123), U256::from(123)]),
                ),
                (
                    &[U256::from(234), U256::from(123)],
                    Some(&[U256::from(234), U256::from(234), U256::from(123)]),
                ),
            ],
        );
        test(
            OpcodeDup(1),
            &[
                (&[], None),
                (&[U256::from(123)], None),
                (
                    &[U256::from(234), U256::from(123)],
                    Some(&[U256::from(123), U256::from(234), U256::from(123)]),
                ),
            ],
        );
    }
}
