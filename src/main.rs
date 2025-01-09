use alloy_primitives::primitives::{Address, U256};
use xevm::context::MiniEthereum;
use xevm::machine::{CallInfo, Machine};
use xevm::opcodes::ExecutionResult;

fn main() {
    let code = vec![];
    let mut ctx = MiniEthereum::new();
    let exec_result = Machine::new(Address::ZERO, code.clone(), 10000000)
        .run(
            &mut ctx,
            &CallInfo {
                origin: Address::ZERO,
                caller: Address::ZERO,
                call_value: U256::ZERO,
                calldata: vec![0xd0, 0x9d, 0xe0, 0x8a],
            },
        )
        .unwrap();
    match exec_result {
        ExecutionResult::Returned(ret) => {
            println!("Returned {:?}!", ret);
        }
        ExecutionResult::Halted => {
            println!("Halted!");
        }
    }
}
