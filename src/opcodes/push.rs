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
pub struct OpcodePush(pub u8);
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodePush {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine.push_stack(if self.0 == 0 {
            W::ZERO
        } else {
            if machine.code.len() < machine.pc + 1 + self.0 as usize {
                return Err(ExecError::Revert(RevertError::NotEnoughBytesInCode));
            }
            W::from_big_endian(&machine.code[machine.pc + 1..machine.pc + 1 + self.0 as usize])
        })?;
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
            vec![],
            OpcodePush(0),
            &[
                (&[], Some(&[U256::from(0)])),
                (&[U256::from(123)], Some(&[U256::from(0), U256::from(123)])),
            ],
        );
        test(
            vec![0, 0x12],
            OpcodePush(1),
            &[
                (&[], Some(&[U256::from(0x12)])),
                (
                    &[U256::from(123)],
                    Some(&[U256::from(0x12), U256::from(123)]),
                ),
            ],
        );
        test(
            vec![0, 0x12, 0x34],
            OpcodePush(2),
            &[
                (&[], Some(&[U256::from(0x1234)])),
                (
                    &[U256::from(123)],
                    Some(&[U256::from(0x1234), U256::from(123)]),
                ),
            ],
        );
        test(
            vec![0, 0x12, 0x34, 0x0],
            OpcodePush(2),
            &[
                (&[], Some(&[U256::from(0x1234)])),
                (
                    &[U256::from(123)],
                    Some(&[U256::from(0x1234), U256::from(123)]),
                ),
            ],
        );
        test(
            vec![0, 0x12, 0x34, 0x0],
            OpcodePush(3),
            &[
                (&[], Some(&[U256::from(0x123400)])),
                (
                    &[U256::from(123)],
                    Some(&[U256::from(0x123400), U256::from(123)]),
                ),
            ],
        );
    }
}
