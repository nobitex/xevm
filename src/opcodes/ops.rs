use super::ExecutionResult;
use super::OpcodeHandler;
use crate::context::Context;
use crate::error::ExecError;
use crate::error::RevertError;
use crate::machine::CallInfo;
use crate::machine::Machine;
use crate::u256::U256;

pub enum OpcodeBinaryOp {
    Add,
    Mul,
    Sub,
    Div,
    Sdiv,
    Mod,
    Smod,
    Exp,
    Shl,
    Shr,
    Sar,
    And,
    Or,
    Xor,
    Byte,
    Lt,
    Gt,
    Slt,
    Sgt,
    Eq,
    SignExtend,
}

pub enum OpcodeUnaryOp {
    IsZero,
    Not,
}

pub enum OpcodeModularOp {
    AddMod,
    MulMod,
}

impl<C: Context> OpcodeHandler<C> for OpcodeModularOp {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let a = machine.pop_stack()?.as_u512();
        let b = machine.pop_stack()?.as_u512();
        let n = machine.pop_stack()?.as_u512();
        machine.stack.push(
            match self {
                Self::AddMod => (a + b) % n,
                Self::MulMod => (a * b) % n,
            }
            .low_u256(),
        );
        machine.pc += 1;
        Ok(None)
    }
}

impl<C: Context> OpcodeHandler<C> for OpcodeUnaryOp {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let a = machine.pop_stack()?;
        machine.stack.push(match self {
            Self::IsZero => U256::from((a == U256::zero()) as u64),
            Self::Not => !a,
        });
        machine.pc += 1;
        Ok(None)
    }
}

impl<C: Context> OpcodeHandler<C> for OpcodeBinaryOp {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(match self {
            Self::Add => a.overflowing_add(b).0,
            Self::Mul => a.overflowing_mul(b).0,
            Self::Sub => a.overflowing_sub(b).0,
            Self::Div => a / b,
            Self::Sdiv => a / b,
            Self::Mod => a % b,
            Self::Smod => a % b,
            Self::Exp => a.pow(b),
            Self::Shl => b << a,
            Self::Shr => b >> a,
            Self::And => a & b,
            Self::Or => a | b,
            Self::Xor => a ^ b,
            Self::Lt => U256::from((a < b) as u64),
            Self::Gt => U256::from((a > b) as u64),
            Self::Slt => U256::from(match (a.is_neg(), b.is_neg()) {
                (false, false) => a < b,
                (false, true) => false,
                (true, false) => true,
                (true, true) => -a > -b,
            } as u64),
            Self::Sgt => U256::from(match (a.is_neg(), b.is_neg()) {
                (false, false) => a > b,
                (false, true) => true,
                (true, false) => false,
                (true, true) => -a < -b,
            } as u64),
            Self::Eq => U256::from((a == b) as u64),
            Self::Byte => {
                let i = a.to_usize()?;
                let x = b.to_big_endian();
                U256::from(if i < 32 { x[i] as u64 } else { 0 })
            }
            Self::Sar => {
                let mut result = b >> a;
                if b.is_neg() {
                    let addition = U256::MAX << (U256::from(256) - a);
                    result += addition;
                }
                result
            }
            Self::SignExtend => return Err(ExecError::Revert(RevertError::UnknownOpcode(0x0b))),
        });
        machine.pc += 1;
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::test;
    use super::*;

    #[test]
    fn test_opcode_sar() {
        test(
            OpcodeBinaryOp::Sar,
            &[
                (&[], None),
                (&[U256::from(123)], None),
                (&[U256::from(0), U256::from(123)], Some(&[U256::from(123)])),
                (&[U256::from(1), U256::from(123)], Some(&[U256::from(61)])),
                (&[U256::from(2), U256::from(123)], Some(&[U256::from(30)])),
                (&[U256::from(3), U256::from(123)], Some(&[U256::from(15)])),
                (&[U256::from(100), U256::from(123)], Some(&[U256::from(0)])),
                (&[U256::from(1), -U256::from(123)], Some(&[-U256::from(62)])),
                (&[U256::from(2), -U256::from(123)], Some(&[-U256::from(31)])),
                (&[U256::from(3), -U256::from(123)], Some(&[-U256::from(16)])),
                (
                    &[U256::from(100), -U256::from(123)],
                    Some(&[-U256::from(1)]),
                ),
                (
                    &[U256::from(128), U256::MAX >> U256::one()],
                    Some(&[U256::MAX >> U256::from(129)]),
                ),
                (&[U256::from(128), U256::MAX], Some(&[U256::MAX])),
            ],
        );
    }

    #[test]
    fn test_opcode_lt() {
        test(
            OpcodeBinaryOp::Lt,
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
            OpcodeBinaryOp::Gt,
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
            OpcodeBinaryOp::Slt,
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
            OpcodeBinaryOp::Sgt,
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
            OpcodeBinaryOp::Eq,
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
            OpcodeUnaryOp::IsZero,
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
