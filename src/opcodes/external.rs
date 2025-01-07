use super::ExecutionResult;
use super::OpcodeHandler;
use crate::context::Context;
use crate::error::ExecError;
use crate::keccak::keccak;
use crate::machine::CallInfo;
use crate::machine::Machine;
use crate::machine::Word;
use crate::u256::U256;

#[derive(Debug)]
pub struct OpcodeAddress;
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeAddress {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        machine.stack.push(machine.address);
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
        machine.stack.push(ctx.balance(addr)?);
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
        machine.stack.push(call_info.call_value);
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
        machine.stack.push(call_info.caller);
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
        machine.stack.push(ctx.block_hash(block_number)?);
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
        machine.stack.push(ctx.balance(machine.address)?);
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
        machine.stack.push(call_info.origin);
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
        machine.stack.push(W::from(machine.code.len() as u64));
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
        let code = machine.code[addr..addr + size].to_vec();
        machine.mem_put(dest_addr, &code);
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
        machine
            .stack
            .push(W::from(call_info.calldata.len() as u64));
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
        machine.mem_put(dest_addr, &call_info.calldata[addr..addr + size]);
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
            ret[i] = call_info
                .calldata
                .get(offset + i)
                .copied()
                .unwrap_or_default();
        }
        machine.stack.push(W::from_big_endian(&ret));
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
        let code = ctx.code(addr)?;
        machine.stack.push(W::from(code.len() as u64));
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
        let code = ctx.code(addr)?;
        machine.mem_put(dest_offset, &code[offset..offset + size]);
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
        let code_hash = keccak(&ctx.code(addr)?);
        machine.stack.push(W::from_big_endian(&code_hash));
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
        machine.stack.push(ctx.blob_hash(index)?);
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
        ctx.destroy(machine.address, target)?;
        Ok(Some(ExecutionResult::Halted))
    }
}
