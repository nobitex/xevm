use alloy_primitives::primitives::{Address, U256};
use xevm::context::MiniEthereum;
use xevm::machine::{CallInfo, GasTracker, Machine};
use xevm::opcodes::ExecutionResult;

fn main() {
    let code = vec![];
    let mut gas_tracker = GasTracker::new(10000000);
    let mut ctx = MiniEthereum::new();
    let exec_result = Machine::new(Address::ZERO, code.clone(), &mut gas_tracker, 1024)
        .run(
            &mut ctx,
            &CallInfo {
                origin: Address::ZERO,
                caller: Address::ZERO,
                call_value: U256::ZERO,
                calldata: vec![0xd0, 0x9d, 0xe0, 0x8a],
                is_static: false,
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
