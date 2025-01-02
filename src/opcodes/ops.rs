use super::ExecutionResult;
use super::OpcodeHandler;
use crate::error::XevmError;
use crate::machine::CallInfo;
use crate::machine::Context;
use crate::machine::Machine;
use crate::u256::U256;

#[derive(Debug)]
pub struct OpcodeAdd;
impl<C: Context> OpcodeHandler<C> for OpcodeAdd {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(a + b);
        machine.gas_used += 3;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeSub;
impl<C: Context> OpcodeHandler<C> for OpcodeSub {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(a - b);
        machine.gas_used += 3;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeMul;
impl<C: Context> OpcodeHandler<C> for OpcodeMul {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(a * b);
        machine.gas_used += 5;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeShl;
impl<C: Context> OpcodeHandler<C> for OpcodeShl {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let shift = machine.pop_stack()?;
        let val = machine.pop_stack()?;
        machine.stack.push(val << shift);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeShr;
impl<C: Context> OpcodeHandler<C> for OpcodeShr {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let shift = machine.pop_stack()?;
        let val = machine.pop_stack()?;
        machine.stack.push(val >> shift);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeSar;
impl<C: Context> OpcodeHandler<C> for OpcodeSar {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let shift = machine.pop_stack()?;
        let val = machine.pop_stack()?;
        let mut result = val >> shift;
        if val.is_neg() {
            let addition = U256::MAX << (U256::from(256) - shift);
            result = result + addition;
        }
        machine.stack.push(result);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeAnd;
impl<C: Context> OpcodeHandler<C> for OpcodeAnd {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(a & b);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeOr;
impl<C: Context> OpcodeHandler<C> for OpcodeOr {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(a | b);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeXor;
impl<C: Context> OpcodeHandler<C> for OpcodeXor {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let a = machine.pop_stack()?;
        let b = machine.pop_stack()?;
        machine.stack.push(a ^ b);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeNot;
impl<C: Context> OpcodeHandler<C> for OpcodeNot {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let a = machine.pop_stack()?;
        machine.stack.push(!a);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeByte;
impl<C: Context> OpcodeHandler<C> for OpcodeByte {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let i = machine.pop_stack()?.as_usize()?;
        let x = machine.pop_stack()?.to_bytes_be();
        machine
            .stack
            .push(U256::from(if i < 32 { x[i] as u64 } else { 0 }));
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
            OpcodeSar,
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
}
