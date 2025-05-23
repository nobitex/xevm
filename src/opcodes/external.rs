/* Audited 11 Feb 2025 - Keyvan Kambakhsh */

use super::ExecutionResult;
use super::OpcodeHandler;
use crate::context::Context;
use crate::error::ExecError;
use crate::keccak::keccak;
use crate::machine::CallInfo;
use crate::machine::Machine;
use crate::machine::Word;

#[derive(Debug)]
pub struct OpcodeAddress;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeAddress {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine.push_stack(W::from_addr(machine.address))?;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeBalance;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeBalance {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let addr = machine.pop_stack()?;
        machine.push_stack(ctx.balance(addr.to_addr()?)?)?;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCallValue;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeCallValue {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine.push_stack(call_info.value)?;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCaller;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeCaller {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine.push_stack(W::from_addr(call_info.caller))?;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeBlockHash;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeBlockHash {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let block_number = machine.pop_stack()?;
        machine.push_stack(ctx.block_hash(block_number)?)?;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeSelfBalance;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeSelfBalance {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine.push_stack(ctx.balance(machine.address)?)?;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeOrigin;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeOrigin {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine.push_stack(W::from_addr(call_info.origin))?;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCodeSize;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeCodeSize {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine.push_stack(W::from_u64(machine.code.len() as u64))?;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCodeCopy;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeCodeCopy {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let dest_addr = machine.pop_stack()?.to_usize()?;
        let addr = machine.pop_stack()?.to_usize()?;
        let size = machine.pop_stack()?.to_usize()?;
        let code = machine.code.clone();
        machine.mem_put(dest_addr, &code, addr, size)?;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCalldataSize;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeCalldataSize {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine.push_stack(W::from_u64(call_info.data.len() as u64))?;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCalldataCopy;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeCalldataCopy {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let dest_addr = machine.pop_stack()?.to_usize()?;
        let addr = machine.pop_stack()?.to_usize()?;
        let size = machine.pop_stack()?.to_usize()?;
        machine.mem_put(dest_addr, &call_info.data, addr, size)?;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeCalldataLoad;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeCalldataLoad {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let offset = machine.pop_stack()?.to_usize()?;
        let mut ret = [0u8; 32];
        for i in 0..32 {
            ret[i] = call_info.data.get(offset + i).copied().unwrap_or_default();
        }
        machine.push_stack(W::from_big_endian(&ret))?;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeExtCodeSize;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeExtCodeSize {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let addr = machine.pop_stack()?;
        let code = ctx.code(addr.to_addr()?)?;
        machine.push_stack(W::from_u64(code.len() as u64))?;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeExtCodeCopy;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeExtCodeCopy {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let addr = machine.pop_stack()?;
        let dest_offset = machine.pop_stack()?.to_usize()?;
        let offset = machine.pop_stack()?.to_usize()?;
        let size = machine.pop_stack()?.to_usize()?;
        let code = ctx.code(addr.to_addr()?)?;
        machine.mem_put(dest_offset, &code, offset, size)?;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeExtCodeHash;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeExtCodeHash {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let addr = machine.pop_stack()?;
        let code_hash = keccak(&ctx.code(addr.to_addr()?)?);
        machine.push_stack(W::from_big_endian(&code_hash))?;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeBlobHash;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeBlobHash {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let index = machine.pop_stack()?;
        machine.push_stack(ctx.blob_hash(index)?)?;
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeSelfDestruct;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeSelfDestruct {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        let target = machine.pop_stack()?;
        ctx.destroy(machine.address, target.to_addr()?)?;
        Ok(Some(ExecutionResult::Halted))
    }
}

#[derive(Debug)]
pub struct OpcodePc;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodePc {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine.stack.push(W::from_u64(machine.pc as u64));
        machine.pc += 1;
        Ok(None)
    }
}

#[derive(Debug)]
pub struct OpcodeGas;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeGas {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine
            .stack
            .push(W::from_u64(machine.gas_tracker.remaining_gas() as u64));
        machine.pc += 1;
        Ok(None)
    }
}
