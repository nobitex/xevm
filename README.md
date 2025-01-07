# xevm

![Tests badge](https://github.com/nobitex/xevm/actions/workflows/xevm.yml/badge.svg)

`xevm` is a tiny implementation of Ethereum Virtual Machine, written in pure Rust!

Sample usage:

```rust
use xevm::context::MiniEthereum;
use xevm::machine::{CallInfo, Machine};
use xevm::opcodes::ExecutionResult;
use xevm::u256::U256;

fn main() {
    let code = vec![1, 2, 3];
    let mut ctx = MiniEthereum::new();
    let exec_result = Machine::new(U256::zero(), code.clone())
        .run(
            &mut ctx,
            &CallInfo {
                origin: U256::zero(),
                call_value: U256::zero(),
                caller: U256::zero(),
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
```