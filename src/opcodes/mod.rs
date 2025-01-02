mod cmp;
mod dup;
mod external;
mod halt;
mod jump;
mod log;
mod memory;
mod ops;
mod pop;
mod push;
mod ret;
mod revert;
mod swap;

pub use cmp::{OpcodeEq, OpcodeGt, OpcodeIsZero, OpcodeLt, OpcodeSgt, OpcodeSlt};
pub use dup::OpcodeDup;
pub use external::{
    OpcodeAddress, OpcodeBalance, OpcodeCallValue, OpcodeCalldataCopy, OpcodeCalldataLoad,
    OpcodeCalldataSize, OpcodeCaller, OpcodeCodeCopy, OpcodeCodeSize, OpcodeOrigin,
};
pub use halt::OpcodeHalt;
pub use jump::{OpcodeJump, OpcodeJumpDest, OpcodeJumpi};
pub use log::OpcodeLog;
pub use memory::{
    OpcodeMload, OpcodeMstore, OpcodeMstore8, OpcodeSload, OpcodeSstore, OpcodeTload, OpcodeTstore,
};
pub use ops::{
    OpcodeAdd, OpcodeAnd, OpcodeByte, OpcodeMul, OpcodeNot, OpcodeOr, OpcodeShl, OpcodeShr,
    OpcodeSub, OpcodeXor,
};
pub use pop::OpcodePop;
pub use push::OpcodePush;
pub use ret::OpcodeReturn;
pub use revert::OpcodeRevert;
pub use swap::OpcodeSwap;

use crate::{
    error::XevmError,
    machine::{CallInfo, Context, Machine},
};

#[derive(Debug, Clone)]
pub enum ExecutionResult {
    Reverted(Vec<u8>),
    Returned(Vec<u8>),
    Halted,
}

pub trait OpcodeHandler<C: Context> {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine,
        text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, XevmError>;
}
