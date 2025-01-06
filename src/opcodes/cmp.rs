use super::ExecutionResult;
use super::OpcodeHandler;
use crate::context::Context;
use crate::error::ExecError;
use crate::machine::CallInfo;
use crate::machine::Machine;
use crate::u256::U256;

#[derive(Debug)]
pub struct OpcodeLt;
impl<C: Context> OpcodeHandler<C> for OpcodeLt {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(U256::from((a < b) as u64));
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeGt;
impl<C: Context> OpcodeHandler<C> for OpcodeGt {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(U256::from((a > b) as u64));
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeEq;
impl<C: Context> OpcodeHandler<C> for OpcodeEq {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(U256::from((a == b) as u64));
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeIsZero;
impl<C: Context> OpcodeHandler<C> for OpcodeIsZero {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let a = machine.pop_stack()?;
        machine.stack.push(U256::from((a == U256::zero()) as u64));
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeSlt;
impl<C: Context> OpcodeHandler<C> for OpcodeSlt {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        let res = match (a.is_neg(), b.is_neg()) {
            (false, false) => a < b,
            (false, true) => false,
            (true, false) => true,
            (true, true) => -a > -b,
        };
        machine.stack.push(U256::from(res as u64));
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeSgt;
impl<C: Context> OpcodeHandler<C> for OpcodeSgt {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        let res = match (a.is_neg(), b.is_neg()) {
            (false, false) => a > b,
            (false, true) => true,
            (true, false) => false,
            (true, true) => -a < -b,
        };
        machine.stack.push(U256::from(res as u64));
        machine.pc += 1;
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::test;
    use super::*;

    #[test]
    fn test_opcode_lt() {
        test(
            OpcodeLt,
            &[
                (&[], None),
                (&[U256::from(123)], None),
                (&[U256::from(123), U256::from(120)], Some(&[U256::zero()])),
                (&[U256::from(123), U256::from(123)], Some(&[U256::zero()])),
                (&[U256::from(123), U256::from(234)], Some(&[U256::one()])),
                (
                    &[U256::MAX, U256::MAX - U256::from(123)],
                    Some(&[U256::zero()]),
                ),
                (&[U256::MAX, U256::MAX], Some(&[U256::zero()])),
                (
                    &[U256::MAX - U256::from(123), U256::MAX],
                    Some(&[U256::one()]),
                ),
            ],
        );
    }

    #[test]
    fn test_opcode_gt() {
        test(
            OpcodeGt,
            &[
                (&[], None),
                (&[U256::from(123)], None),
                (&[U256::from(123), U256::from(120)], Some(&[U256::one()])),
                (&[U256::from(123), U256::from(123)], Some(&[U256::zero()])),
                (&[U256::from(123), U256::from(234)], Some(&[U256::zero()])),
                (
                    &[U256::MAX, U256::MAX - U256::from(123)],
                    Some(&[U256::one()]),
                ),
                (&[U256::MAX, U256::MAX], Some(&[U256::zero()])),
                (
                    &[U256::MAX - U256::from(123), U256::MAX],
                    Some(&[U256::zero()]),
                ),
            ],
        );
    }

    #[test]
    fn test_opcode_slt() {
        test(
            OpcodeSlt,
            &[
                (&[], None),
                (&[U256::from(123)], None),
                (&[U256::from(123), U256::from(120)], Some(&[U256::zero()])),
                (&[U256::from(123), U256::from(123)], Some(&[U256::zero()])),
                (&[U256::from(123), U256::from(234)], Some(&[U256::one()])),
                (&[-U256::from(123), U256::from(123)], Some(&[U256::one()])),
                (&[U256::from(123), -U256::from(123)], Some(&[U256::zero()])),
                (&[-U256::from(123), -U256::from(123)], Some(&[U256::zero()])),
                (&[-U256::from(123), -U256::from(234)], Some(&[U256::zero()])),
                (&[-U256::from(234), -U256::from(123)], Some(&[U256::one()])),
            ],
        );
    }

    #[test]
    fn test_opcode_sgt() {
        test(
            OpcodeSgt,
            &[
                (&[], None),
                (&[U256::from(123)], None),
                (&[U256::from(123), U256::from(120)], Some(&[U256::one()])),
                (&[U256::from(123), U256::from(123)], Some(&[U256::zero()])),
                (&[U256::from(123), U256::from(234)], Some(&[U256::zero()])),
                (&[-U256::from(123), U256::from(123)], Some(&[U256::zero()])),
                (&[U256::from(123), -U256::from(123)], Some(&[U256::one()])),
                (&[-U256::from(123), -U256::from(123)], Some(&[U256::zero()])),
                (&[-U256::from(123), -U256::from(234)], Some(&[U256::one()])),
                (&[-U256::from(234), -U256::from(123)], Some(&[U256::zero()])),
            ],
        );
    }

    #[test]
    fn test_opcode_eq() {
        test(
            OpcodeEq,
            &[
                (&[], None),
                (&[U256::from(123)], None),
                (&[U256::from(0), U256::from(0)], Some(&[U256::one()])),
                (&[U256::from(123), U256::from(123)], Some(&[U256::one()])),
                (&[U256::from(123), U256::from(122)], Some(&[U256::zero()])),
                (&[U256::MAX, U256::MAX], Some(&[U256::one()])),
                (&[U256::MAX, U256::MAX - U256::one()], Some(&[U256::zero()])),
            ],
        );
    }

    #[test]
    fn test_opcode_is_zero() {
        test(
            OpcodeIsZero,
            &[
                (&[], None),
                (&[U256::from(0)], Some(&[U256::one()])),
                (&[U256::from(123)], Some(&[U256::zero()])),
                (&[U256::MAX], Some(&[U256::zero()])),
                (&[U256::MAX - U256::one()], Some(&[U256::zero()])),
            ],
        );
    }
}
