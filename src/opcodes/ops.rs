use super::ExecutionResult;
use super::OpcodeHandler;
use crate::context::Context;
use crate::error::ExecError;
use crate::error::RevertError;
use crate::machine::CallInfo;
use crate::machine::Machine;
use crate::machine::Word;

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

impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeModularOp {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        let n = machine.pop_stack()?;
        machine.stack.push(match self {
            Self::AddMod => a.addmod(b, n),
            Self::MulMod => a.mulmod(b, n),
        });
        machine.pc += 1;
        Ok(None)
    }
}

impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeUnaryOp {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let a = machine.pop_stack()?;
        machine.stack.push(match self {
            Self::IsZero => W::from_u64((a == W::ZERO) as u64),
            Self::Not => a.not(),
        });
        machine.pc += 1;
        Ok(None)
    }
}

impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeBinaryOp {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(match self {
            Self::Add => a.add(b),
            Self::Mul => a.mul(b),
            Self::Sub => a.sub(b),
            Self::Div => {
                if b == W::ZERO {
                    W::ZERO
                } else {
                    a.div(b)
                }
            }
            Self::Sdiv => match (a.is_neg(), b.is_neg()) {
                (false, false) => a.div(b),
                (true, true) => a.neg().div(b.neg()),
                (false, true) => if a.rem(b.neg()) == W::ZERO {
                    a.div(b.neg())
                } else {
                    a.div(b.neg()).add(W::ONE)
                }
                .neg(),
                (true, false) => if a.neg().rem(b) == W::ZERO {
                    a.neg().div(b)
                } else {
                    a.neg().div(b).add(W::ONE)
                }
                .neg(),
            },
            Self::Mod => a.rem(b),
            Self::Smod => match (a.is_neg(), b.is_neg()) {
                (false, false) => a.rem(b),
                (true, true) => a.neg().rem(b.neg()).neg(),
                (false, true) => {
                    if a.rem(b.neg()) == W::ZERO {
                        W::ZERO
                    } else {
                        b.neg().sub(a.rem(b.neg())).neg()
                    }
                }
                (true, false) => {
                    if a.neg().rem(b) == W::ZERO {
                        W::ZERO
                    } else {
                        b.sub(a.neg().rem(b))
                    }
                }
            },
            Self::Exp => a.pow(b),
            Self::Shl => b.shl(a),
            Self::Shr => b.shr(a),
            Self::And => a.and(b),
            Self::Or => a.or(b),
            Self::Xor => a.xor(b),
            Self::Lt => W::from_u64((a < b) as u64),
            Self::Gt => W::from_u64((a > b) as u64),
            Self::Slt => W::from_u64(match (a.is_neg(), b.is_neg()) {
                (false, false) => a.lt(b),
                (false, true) => false,
                (true, false) => true,
                (true, true) => a.neg().gt(b.neg()),
            } as u64),
            Self::Sgt => W::from_u64(match (a.is_neg(), b.is_neg()) {
                (false, false) => a.gt(b),
                (false, true) => true,
                (true, false) => false,
                (true, true) => a.neg().lt(b.neg()),
            } as u64),
            Self::Eq => W::from_u64((a == b) as u64),
            Self::Byte => {
                let i = a.to_usize()?;
                let x = b.to_big_endian();
                W::from_u64(if i < 32 { x[i] as u64 } else { 0 })
            }
            Self::Sar => {
                let mut result = b.shr(a);
                if b.is_neg() {
                    let addition = W::MAX.shl(W::from_u64(256).sub(a));
                    result = result.add(addition);
                }
                result
            }
            Self::SignExtend => {
                let bytes_1 = a.to_usize()?;
                if bytes_1 > 31 {
                    return Err(ExecError::Revert(RevertError::OutOfBounds));
                }
                let bytes = bytes_1 + 1;
                let is_neg = b.bit(bytes_1 * 8 + 7);
                let x = b
                    .shl(W::from_u64((256 - bytes * 8) as u64))
                    .shr(W::from_u64((256 - bytes * 8) as u64));
                if is_neg {
                    x.add(W::MAX.shl(W::from_u64(((bytes_1 + 1) * 8) as u64)))
                } else {
                    x
                }
            }
        });
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
                    &[U256::from(128), U256::MAX >> U256::ONE],
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
                (&[U256::from(123), U256::from(120)], Some(&[U256::ZERO])),
                (&[U256::from(123), U256::from(123)], Some(&[U256::ZERO])),
                (&[U256::from(123), U256::from(234)], Some(&[U256::ONE])),
                (
                    &[U256::MAX, U256::MAX - U256::from(123)],
                    Some(&[U256::ZERO]),
                ),
                (&[U256::MAX, U256::MAX], Some(&[U256::ZERO])),
                (
                    &[U256::MAX - U256::from(123), U256::MAX],
                    Some(&[U256::ONE]),
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
                (&[U256::from(123), U256::from(120)], Some(&[U256::ONE])),
                (&[U256::from(123), U256::from(123)], Some(&[U256::ZERO])),
                (&[U256::from(123), U256::from(234)], Some(&[U256::ZERO])),
                (
                    &[U256::MAX, U256::MAX - U256::from(123)],
                    Some(&[U256::ONE]),
                ),
                (&[U256::MAX, U256::MAX], Some(&[U256::ZERO])),
                (
                    &[U256::MAX - U256::from(123), U256::MAX],
                    Some(&[U256::ZERO]),
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
                (&[U256::from(123), U256::from(120)], Some(&[U256::ZERO])),
                (&[U256::from(123), U256::from(123)], Some(&[U256::ZERO])),
                (&[U256::from(123), U256::from(234)], Some(&[U256::ONE])),
                (&[-U256::from(123), U256::from(123)], Some(&[U256::ONE])),
                (&[U256::from(123), -U256::from(123)], Some(&[U256::ZERO])),
                (&[-U256::from(123), -U256::from(123)], Some(&[U256::ZERO])),
                (&[-U256::from(123), -U256::from(234)], Some(&[U256::ZERO])),
                (&[-U256::from(234), -U256::from(123)], Some(&[U256::ONE])),
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
                (&[U256::from(123), U256::from(120)], Some(&[U256::ONE])),
                (&[U256::from(123), U256::from(123)], Some(&[U256::ZERO])),
                (&[U256::from(123), U256::from(234)], Some(&[U256::ZERO])),
                (&[-U256::from(123), U256::from(123)], Some(&[U256::ZERO])),
                (&[U256::from(123), -U256::from(123)], Some(&[U256::ONE])),
                (&[-U256::from(123), -U256::from(123)], Some(&[U256::ZERO])),
                (&[-U256::from(123), -U256::from(234)], Some(&[U256::ONE])),
                (&[-U256::from(234), -U256::from(123)], Some(&[U256::ZERO])),
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
                (&[U256::from(0), U256::from(0)], Some(&[U256::ONE])),
                (&[U256::from(123), U256::from(123)], Some(&[U256::ONE])),
                (&[U256::from(123), U256::from(122)], Some(&[U256::ZERO])),
                (&[U256::MAX, U256::MAX], Some(&[U256::ONE])),
                (&[U256::MAX, U256::MAX - U256::ONE], Some(&[U256::ZERO])),
            ],
        );
    }

    #[test]
    fn test_opcode_is_zero() {
        test(
            OpcodeUnaryOp::IsZero,
            &[
                (&[], None),
                (&[U256::from(0)], Some(&[U256::ONE])),
                (&[U256::from(123)], Some(&[U256::ZERO])),
                (&[U256::MAX], Some(&[U256::ZERO])),
                (&[U256::MAX - U256::ONE], Some(&[U256::ZERO])),
            ],
        );
    }

    #[test]
    fn test_opcode_sdiv() {
        test(
            OpcodeBinaryOp::Sdiv,
            &[
                (&[U256::from(11), U256::from(2)], Some(&[U256::from(5)])),
                (&[-U256::from(11), -U256::from(2)], Some(&[U256::from(5)])),
                (&[-U256::from(11), U256::from(2)], Some(&[-U256::from(6)])),
                (&[U256::from(11), -U256::from(2)], Some(&[-U256::from(6)])),
                (&[U256::from(10), U256::from(2)], Some(&[U256::from(5)])),
                (&[-U256::from(10), -U256::from(2)], Some(&[U256::from(5)])),
                (&[U256::from(10), -U256::from(2)], Some(&[-U256::from(5)])),
                (&[-U256::from(10), U256::from(2)], Some(&[-U256::from(5)])),
            ],
        );
    }
    #[test]
    fn test_opcode_smod() {
        test(
            OpcodeBinaryOp::Smod,
            &[
                (&[U256::from(11), U256::from(3)], Some(&[U256::from(2)])),
                (&[-U256::from(11), -U256::from(3)], Some(&[-U256::from(2)])),
                (&[-U256::from(11), U256::from(3)], Some(&[U256::from(1)])),
                (&[U256::from(11), -U256::from(3)], Some(&[-U256::from(1)])),
                (&[U256::from(10), U256::from(3)], Some(&[U256::from(1)])),
                (&[-U256::from(10), -U256::from(3)], Some(&[-U256::from(1)])),
                (&[-U256::from(10), U256::from(3)], Some(&[U256::from(2)])),
                (&[U256::from(10), -U256::from(3)], Some(&[-U256::from(2)])),
                (&[U256::from(123), U256::from(100)], Some(&[U256::from(23)])),
                (
                    &[-U256::from(123), -U256::from(100)],
                    Some(&[-U256::from(23)]),
                ),
                (
                    &[-U256::from(123), U256::from(100)],
                    Some(&[U256::from(77)]),
                ),
                (
                    &[U256::from(123), -U256::from(100)],
                    Some(&[-U256::from(77)]),
                ),
            ],
        );
    }
    #[test]
    fn test_sign_extend() {
        test(
            OpcodeBinaryOp::SignExtend,
            &[
                (&[U256::from(0), U256::from(0xff)], Some(&[U256::MAX])),
                (
                    &[U256::from(0), U256::from(0x7f)],
                    Some(&[U256::from(0x7f)]),
                ),
                (&[U256::from(0), -U256::from(1)], Some(&[-U256::from(1)])),
                (
                    &[U256::from(1), U256::from(0x1234)],
                    Some(&[U256::from(0x1234)]),
                ),
                (
                    &[U256::from(1), U256::from(0x8234)],
                    Some(&[-U256::from(32204)]),
                ),
                (
                    &[U256::from(2), U256::from(0x8234)],
                    Some(&[U256::from(0x8234)]),
                ),
                (
                    &[U256::from(31), U256::from(0x8234)],
                    Some(&[U256::from(0x8234)]),
                ),
                (&[U256::from(32), U256::from(0x8234)], None),
                (&[U256::MAX, U256::from(0x8234)], None),
                (&[U256::from(31), U256::MAX], Some(&[U256::MAX])),
            ],
        );
    }
}
