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
    OpcodeAdd, OpcodeAnd, OpcodeByte, OpcodeMul, OpcodeNot, OpcodeOr, OpcodeSar, OpcodeShl,
    OpcodeShr, OpcodeSub, OpcodeXor,
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

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::u256::U256;

    use super::*;

    #[derive(Clone, Debug, Default)]
    pub struct TestContext;
    impl Context for TestContext {
        fn balance(&self, _address: U256) -> Result<U256, Box<dyn Error>> {
            unimplemented!()
        }
        fn address(&self) -> Result<U256, Box<dyn Error>> {
            unimplemented!()
        }
        fn sload(&self, _address: U256) -> Result<U256, Box<dyn Error>> {
            unimplemented!()
        }
        fn sstore(&mut self, _address: U256, _value: U256) -> Result<(), Box<dyn Error>> {
            unimplemented!()
        }
        fn log(&self, _topics: Vec<U256>, _data: Vec<u8>) -> Result<(), Box<dyn Error>> {
            unimplemented!()
        }
    }

    pub fn test<O: OpcodeHandler<TestContext>>(
        opcode_handler: O,
        testcases: &[(&[U256], Option<&[U256]>)],
    ) {
        for (inp, expected_out) in testcases {
            let mut ctx = TestContext;
            let mut machine = Machine::new(vec![]);
            let mut inp_reversed = inp.to_vec();
            inp_reversed.reverse();
            machine.stack.extend(inp_reversed);
            let res = opcode_handler.call(&mut ctx, &mut machine, &[], &CallInfo::default());
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
