use xevm::context::MiniEthereum;
use xevm::machine::{CallInfo, Machine, Word};
use xevm::opcodes::ExecutionResult;
use xevm::u256::U256;

fn main() {
    let code = vec![1, 2, 3];
    let mut ctx = MiniEthereum::new();
    let exec_result = Machine::new(U256::ZERO, code.clone())
        .run(
            &mut ctx,
            &CallInfo {
                origin: U256::ZERO,
                call_value: U256::ZERO,
                caller: U256::ZERO,
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
