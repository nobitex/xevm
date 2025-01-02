mod cmp;
mod dup;
mod external;
mod halt;
mod jump;
mod memory;
mod ops;
mod pop;
mod push;
mod ret;
mod revert;
mod swap;

pub use cmp::{OpcodeEq, OpcodeGt, OpcodeIsZero, OpcodeLt};
pub use dup::OpcodeDup;
pub use external::{
    OpcodeAddress, OpcodeBalance, OpcodeCallValue, OpcodeCalldataCopy, OpcodeCalldataLoad,
    OpcodeCalldataSize, OpcodeCaller, OpcodeCodeCopy, OpcodeCodeSize, OpcodeOrigin,
};
pub use halt::OpcodeHalt;
pub use jump::{OpcodeJump, OpcodeJumpDest, OpcodeJumpi};
pub use memory::{
    OpcodeMload, OpcodeMstore, OpcodeMstore8, OpcodeSload, OpcodeSstore, OpcodeTload, OpcodeTstore,
};
pub use ops::{
    OpcodeAdd, OpcodeAnd, OpcodeByte, OpcodeMul, OpcodeNot, OpcodeOr, OpcodeSgt, OpcodeShl,
    OpcodeShr, OpcodeSlt, OpcodeSub, OpcodeXor,
};
pub use pop::OpcodePop;
pub use push::OpcodePush;
pub use ret::OpcodeReturn;
pub use revert::OpcodeRevert;
pub use swap::OpcodeSwap;
