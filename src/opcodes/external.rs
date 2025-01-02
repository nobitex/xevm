use crate::u256::U256;
use crate::CallInfo;
use crate::Context;
use crate::ExecutionResult;
use crate::Machine;
use crate::OpcodeHandler;
use crate::XevmError;

#[derive(Debug)]
pub struct OpcodeAddress;
impl<C: Context> OpcodeHandler<C> for OpcodeAddress {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
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
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let addr = machine.pop_stack()?;
        machine.stack.push(ctx.balance(addr)?);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCallvalue;
impl<C: Context> OpcodeHandler<C> for OpcodeCallvalue {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
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
        _text: &[u8],
        call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        machine.stack.push(call_info.caller);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCodesize;
impl<C: Context> OpcodeHandler<C> for OpcodeCodesize {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        machine.stack.push(U256::from(text.len() as u64));
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCodecopy;
impl<C: Context> OpcodeHandler<C> for OpcodeCodecopy {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let dest_addr = machine.pop_stack()?.lower_usize();
        let addr = machine.pop_stack()?.lower_usize();
        let size = machine.pop_stack()?.lower_usize();
        machine.mem_put(dest_addr, &text[addr..addr + size]);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCalldatasize;
impl<C: Context> OpcodeHandler<C> for OpcodeCalldatasize {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        machine
            .stack
            .push(U256::from(call_info.calldata.len() as u64));
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCalldatacopy;
impl<C: Context> OpcodeHandler<C> for OpcodeCalldatacopy {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let dest_addr = machine.pop_stack()?.lower_usize();
        let addr = machine.pop_stack()?.lower_usize();
        let size = machine.pop_stack()?.lower_usize();
        machine.mem_put(dest_addr, &call_info.calldata[addr..addr + size]);
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCalldataload;
impl<C: Context> OpcodeHandler<C> for OpcodeCalldataload {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError> {
        let offset = machine.pop_stack()?.lower_usize();
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
