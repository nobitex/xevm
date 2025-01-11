mod call;
mod create;
mod dup;
mod external;
mod halt;
mod info;
mod jump;
mod keccak;
mod log;
mod memory;
mod ops;
mod pop;
mod push;
mod ret;
mod revert;
mod swap;

pub use call::{OpcodeCall, OpcodeReturnDataCopy, OpcodeReturnDataSize};
pub use create::OpcodeCreate;
pub use dup::OpcodeDup;
pub use external::{
    OpcodeAddress, OpcodeBalance, OpcodeBlobHash, OpcodeBlockHash, OpcodeCallValue,
    OpcodeCalldataCopy, OpcodeCalldataLoad, OpcodeCalldataSize, OpcodeCaller, OpcodeCodeCopy,
    OpcodeCodeSize, OpcodeExtCodeCopy, OpcodeExtCodeHash, OpcodeExtCodeSize, OpcodeGas,
    OpcodeOrigin, OpcodePc, OpcodeSelfBalance, OpcodeSelfDestruct,
};
pub use halt::OpcodeHalt;
pub use info::OpcodeInfo;
pub use jump::{OpcodeJump, OpcodeJumpDest, OpcodeJumpi};
pub use keccak::OpcodeKeccak;
pub use log::OpcodeLog;
pub use memory::{
    OpcodeMcopy, OpcodeMload, OpcodeMsize, OpcodeMstore, OpcodeMstore8, OpcodeSload, OpcodeSstore,
    OpcodeTload, OpcodeTstore,
};
pub use ops::{OpcodeBinaryOp, OpcodeModularOp, OpcodeUnaryOp};
pub use pop::OpcodePop;
pub use push::OpcodePush;
pub use ret::OpcodeReturn;
pub use revert::OpcodeRevert;
pub use swap::OpcodeSwap;

use crate::{
    context::Context,
    error::ExecError,
    machine::{CallInfo, Machine, Word},
};

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionResult {
    Returned(Vec<u8>),
    Halted,
}

pub trait OpcodeHandler<W: Word, C: Context<W>> {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError>;
}

#[derive(Debug)]
pub struct OpcodeUnsupported(pub u8);
impl<W: Word, C: Context<W>> OpcodeHandler<W, C> for OpcodeUnsupported {
    fn call(
        &self,
        _ctx: &mut C,
        _machine: &mut Machine<W>,
        _call_info: &CallInfo<W>,
    ) -> Result<Option<ExecutionResult>, ExecError> {
        Err(ExecError::Context(
            format!(
                "Opcode 0x{:02x} is not supported! Feel free to open a PR! :)",
                self.0
            )
            .into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::primitives::Address;

    use crate::{context::MiniEthereum, u256::U256};

    use super::*;

    pub fn test<O: OpcodeHandler<U256, MiniEthereum>>(
        opcode_handler: O,
        testcases: &[(&[U256], Option<&[U256]>)],
    ) {
        for (inp, expected_out) in testcases {
            let mut ctx = MiniEthereum::new();
            let mut machine = Machine::new(Address::ZERO, vec![], 10000000);
            let mut inp_reversed = inp.to_vec();
            inp_reversed.reverse();
            machine.stack.extend(inp_reversed);
            let res = opcode_handler.call(&mut ctx, &mut machine, &CallInfo::default());
            if let Some(expected) = expected_out {
                assert!(res.is_ok());
                let mut out_reversed = expected.to_vec();
                out_reversed.reverse();
                assert_eq!(&machine.stack, &out_reversed);
            } else {
                assert!(res.is_err());
            }
        }
    }
}
