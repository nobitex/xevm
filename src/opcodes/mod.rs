mod add;
mod cmp;
mod dup;
mod external;
mod halt;
mod jump;
mod memory;
mod mul;
mod pop;
mod push;
mod ret;
mod revert;
mod sub;
mod swap;

pub use add::OpcodeAdd;
pub use cmp::{
    OpcodeAnd, OpcodeEq, OpcodeGt, OpcodeIszero, OpcodeLt, OpcodeOr, OpcodeSgt, OpcodeShl,
    OpcodeShr, OpcodeSlt,
};
pub use dup::OpcodeDup;
pub use external::{
    OpcodeAddress, OpcodeBalance, OpcodeCaller, OpcodeCallvalue, OpcodeCodecopy, OpcodeCodesize,
};
pub use halt::OpcodeHalt;
pub use jump::{OpcodeJump, OpcodeJumpdest, OpcodeJumpi};
pub use memory::{
    OpcodeMload, OpcodeMstore, OpcodeMstore8, OpcodeSload, OpcodeSstore, OpcodeTload, OpcodeTstore,
};
pub use mul::OpcodeMul;
pub use pop::OpcodePop;
pub use push::OpcodePush;
pub use ret::OpcodeReturn;
pub use revert::OpcodeRevert;
pub use sub::OpcodeSub;
pub use swap::OpcodeSwap;
