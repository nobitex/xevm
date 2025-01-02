use super::ExecutionResult;
use super::OpcodeHandler;
use crate::error::XevmError;
use crate::machine::CallInfo;
use crate::machine::Context;
use crate::machine::Machine;

#[derive(Debug)]
pub struct OpcodeDup(pub u8);
impl<C: Context> OpcodeHandler<C> for OpcodeDup {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        if self.0 as usize >= machine.stack.len() {
            return Err(XevmError::Other("Dup element doesn't exist!".into()));
        }
        let elem = machine
            .stack
            .get(machine.stack.len() - 1 - self.0 as usize)
            .copied()
            .ok_or(XevmError::Other("Dup element doesn't exist!".into()))?;
        machine.stack.push(elem);
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
