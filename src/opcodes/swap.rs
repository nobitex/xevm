use super::ExecutionResult;
use super::OpcodeHandler;
use crate::context::Context;
use crate::error::ExecError;
use crate::error::RevertError;
use crate::machine::CallInfo;
use crate::machine::Machine;

#[derive(Debug)]
pub struct OpcodeSwap(pub u8);
impl<C: Context> OpcodeHandler<C> for OpcodeSwap {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let a = machine.pop_stack()?;
        let stack_len = machine.stack.len();
        if self.0 as usize >= stack_len {
            return Err(ExecError::Revert(RevertError::NotEnoughValuesOnStack));
        }
        let b = machine
            .stack
            .get_mut(stack_len - 1 - self.0 as usize)
            .ok_or(ExecError::Revert(RevertError::NotEnoughValuesOnStack))?;
        let b_val = *b;
        *b = a;
        machine.stack.push(b_val);
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
    fn test_opcode_swap() {
        test(
            OpcodeSwap(0),
            &[
                (&[], None),
                (&[U256::from(123)], None),
                (
                    &[U256::from(234), U256::from(123)],
                    Some(&[U256::from(123), U256::from(234)]),
                ),
            ],
        );
        test(
            OpcodeSwap(1),
            &[
                (&[], None),
                (&[U256::from(123)], None),
                (&[U256::from(234), U256::from(123)], None),
                (
                    &[U256::from(345), U256::from(234), U256::from(123)],
                    Some(&[U256::from(123), U256::from(234), U256::from(345)]),
                ),
            ],
        );
    }
}
