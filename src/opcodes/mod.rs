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
pub use call::OpcodeCall;
pub use create::{OpcodeCreate, OpcodeCreate2};
pub use dup::OpcodeDup;
pub use external::{
    OpcodeAddress, OpcodeBalance, OpcodeCallValue, OpcodeCalldataCopy, OpcodeCalldataLoad,
    OpcodeCalldataSize, OpcodeCaller, OpcodeCodeCopy, OpcodeCodeSize, OpcodeOrigin,
};
pub use halt::OpcodeHalt;
pub use info::OpcodeInfo;
pub use jump::{OpcodeJump, OpcodeJumpDest, OpcodeJumpi};
pub use keccak::OpcodeKeccak;
pub use log::OpcodeLog;
pub use memory::{
    OpcodeMcopy, OpcodeMload, OpcodeMstore, OpcodeMstore8, OpcodeSload, OpcodeSstore, OpcodeTload,
    OpcodeTstore,
};
pub use ops::{OpcodeBinaryOp, OpcodeNot, OpcodeUnaryOp};
pub use pop::OpcodePop;
pub use push::OpcodePush;
pub use ret::OpcodeReturn;
pub use revert::OpcodeRevert;
pub use swap::OpcodeSwap;

use crate::{
    context::Context,
    error::ExecError,
    machine::{CallInfo, Machine},
};

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionResult {
    Returned(Vec<u8>),
    Halted,
}

pub trait OpcodeHandler<C: Context> {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine,
        _call_info: &CallInfo,
    ) -> Result<Option<ExecutionResult>, ExecError>;
}

#[derive(Debug)]
pub struct OpcodeUnsupported(pub u8);
impl<C: Context> OpcodeHandler<C> for OpcodeUnsupported {
    fn call(
        &self,
        _ctx: &mut C,
        _machine: &mut Machine,
        _call_info: &CallInfo,
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
    use std::error::Error;

    use crate::{context::Info, u256::U256};

    use super::*;

    #[derive(Clone, Debug, Default)]
    pub struct TestContext;
    impl Context for TestContext {
        fn info(&self, _inf: Info) -> Result<U256, Box<dyn Error>> {
            unimplemented!()
        }
        fn create(
            &mut self,
            _creator: U256,
            _value: U256,
            _code: Vec<u8>,
        ) -> Result<U256, ExecError> {
            unimplemented!()
        }
        fn create2(
            &mut self,
            _creator: U256,
            _value: U256,
            _code: Vec<u8>,
            _salt: U256,
        ) -> Result<U256, ExecError> {
            unimplemented!()
        }
        fn call(
            &mut self,
            _gas: U256,
            _address: U256,
            _call_info: CallInfo,
        ) -> Result<ExecutionResult, ExecError> {
            unimplemented!()
        }
        fn balance(&self, _address: U256) -> Result<U256, Box<dyn Error>> {
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
            let mut machine = Machine::new(U256::zero(), vec![]);
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
