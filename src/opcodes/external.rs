use super::ExecutionResult;
use super::OpcodeHandler;
use crate::error::ExecError;
use crate::machine::CallInfo;
use crate::machine::Context;
use crate::machine::Machine;
use crate::u256::U256;

#[derive(Debug)]
pub struct OpcodeAddress;
impl<C: Context> OpcodeHandler<C> for OpcodeAddress {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine.stack.push(ctx.address()?);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeBalance;
impl<C: Context> OpcodeHandler<C> for OpcodeBalance {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine,

        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let addr = machine.pop_stack()?;
        machine.stack.push(ctx.balance(addr)?);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCallValue;
impl<C: Context> OpcodeHandler<C> for OpcodeCallValue {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine.stack.push(call_info.call_value);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCaller;
impl<C: Context> OpcodeHandler<C> for OpcodeCaller {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine.stack.push(call_info.caller);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeOrigin;
impl<C: Context> OpcodeHandler<C> for OpcodeOrigin {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine.stack.push(call_info.origin);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCodeSize;
impl<C: Context> OpcodeHandler<C> for OpcodeCodeSize {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine.stack.push(U256::from(machine.code.len() as u64));
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCodeCopy;
impl<C: Context> OpcodeHandler<C> for OpcodeCodeCopy {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let dest_addr = machine.pop_stack()?.as_usize()?;
        let addr = machine.pop_stack()?.as_usize()?;
        let size = machine.pop_stack()?.as_usize()?;
        let code = machine.code[addr..addr + size].to_vec();
        machine.mem_put(dest_addr, &code);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCalldataSize;
impl<C: Context> OpcodeHandler<C> for OpcodeCalldataSize {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine
            .stack
            .push(U256::from(call_info.calldata.len() as u64));
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCalldataCopy;
impl<C: Context> OpcodeHandler<C> for OpcodeCalldataCopy {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let dest_addr = machine.pop_stack()?.as_usize()?;
        let addr = machine.pop_stack()?.as_usize()?;
        let size = machine.pop_stack()?.as_usize()?;
        machine.mem_put(dest_addr, &call_info.calldata[addr..addr + size]);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCalldataLoad;
impl<C: Context> OpcodeHandler<C> for OpcodeCalldataLoad {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,

        call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let offset = machine.pop_stack()?.as_usize()?;
        let mut ret = [0u8; 32];
        for i in 0..32 {
            ret[i] = call_info
                .calldata
                .get(offset + i)
                .copied()
                .unwrap_or_default();
        }
        machine.stack.push(U256::from_bytes_be(&ret));
        machine.pc += 1;
        Ok(None)
    }
}
