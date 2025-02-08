use alloy_primitives::primitives::Address;

use crate::{
    context::{Account, Context, ContextMut, MiniEthereum},
    machine::{CallInfo, GasTracker, Word},
    opcodes::ExecutionResult,
    u256::U256,
};

fn addr(v: u8) -> Address {
    let mut arr = [0u8; 20];
    arr[19] = v;
    Address::from_slice(&arr)
}

mod erc20;
#[test]
fn test_erc20_deploy() {
    let mut gt = GasTracker::new(10000000);
    let mut ctx = MiniEthereum::default();
    ctx.accounts.entry(addr(123)).or_insert(Account {
        nonce: U256::from(0),
        value: U256::from(5),
        code: vec![],
        storage: Default::default(),
    });
    let mut creation_code = erc20::PLAIN_ERC20_BYTECODE.to_vec();
    // ("Hello!", "HLO", 1000000 ether)
    creation_code.extend([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 211, 194,
        27, 206, 204, 237, 161, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 72, 101, 108, 108, 111, 33, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 72, 76, 79, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    let contract_addr = ctx
        .as_mut()
        .create(
            1024,
            &mut gt,
            CallInfo {
                origin: addr(123),
                caller: addr(123),
                call_value: U256::from(0),
                calldata: creation_code,
                is_static: false,
            },
            None,
        )
        .unwrap();
    let total_supply_sig = [0x18, 0x16, 0x0d, 0xdd];
    let call = move |ctx: &mut MiniEthereum, from: Address, inp: &[u8]| {
        let mut gt = GasTracker::new(10000000);
        ctx.as_mut()
            .call(
                1024,
                &mut gt,
                contract_addr,
                CallInfo {
                    origin: from,
                    caller: from,
                    call_value: U256::ZERO,
                    calldata: inp.to_vec(),
                    is_static: false,
                },
            )
            .unwrap()
    };
    assert_eq!(
        call(&mut ctx, addr(123), &total_supply_sig),
        ExecutionResult::Returned(
            U256::from_str_radix("1000000000000000000000000", 10)
                .unwrap()
                .to_big_endian()
                .to_vec()
        )
    );
    fn balance_of_calldata(addr: U256) -> Vec<u8> {
        let mut ret = vec![0x70, 0xa0, 0x82, 0x31];
        ret.extend(addr.to_big_endian());
        ret
    }
    fn transfer_calldata(to: U256, amount: U256) -> Vec<u8> {
        let mut ret = vec![0xa9, 0x05, 0x9c, 0xbb];
        ret.extend(to.to_big_endian());
        ret.extend(amount.to_big_endian());
        ret
    }
    assert_eq!(
        call(&mut ctx, addr(123), &total_supply_sig),
        ExecutionResult::Returned(
            U256::from_str_radix("1000000000000000000000000", 10)
                .unwrap()
                .to_big_endian()
                .to_vec()
        )
    );
    assert_eq!(
        call(&mut ctx, addr(123), &balance_of_calldata(U256::from(123))),
        ExecutionResult::Returned(
            U256::from_str_radix("1000000000000000000000000", 10)
                .unwrap()
                .to_big_endian()
                .to_vec()
        )
    );
    assert_eq!(
        call(&mut ctx, addr(123), &balance_of_calldata(U256::from(234))),
        ExecutionResult::Returned(U256::ZERO.to_big_endian().to_vec())
    );
    assert_eq!(
        call(
            &mut ctx,
            addr(123),
            &transfer_calldata(U256::from(234), U256::from(567))
        ),
        ExecutionResult::Returned(U256::ONE.to_big_endian().to_vec())
    );
    assert_eq!(
        call(&mut ctx, addr(123), &balance_of_calldata(U256::from(123))),
        ExecutionResult::Returned(
            U256::from_str_radix("999999999999999999999433", 10)
                .unwrap()
                .to_big_endian()
                .to_vec()
        )
    );
    assert_eq!(
        call(&mut ctx, addr(123), &balance_of_calldata(U256::from(234))),
        ExecutionResult::Returned(U256::from(567).to_big_endian().to_vec())
    );
}
