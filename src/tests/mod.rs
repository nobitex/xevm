use crate::{
    context::{Account, Context, MiniEthereum},
    machine::CallInfo,
    opcodes::ExecutionResult,
    u256::U256,
};

mod erc20;
#[test]
fn test_erc20_deploy() {
    let mut ctx = MiniEthereum::default();
    ctx.accounts.entry(123.into()).or_insert(Account {
        nonce: 0.into(),
        value: 5.into(),
        code: vec![],
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
        .create(CallInfo {
            origin: 123.into(),
            caller: 123.into(),
            call_value: 0.into(),
            calldata: creation_code,
        })
        .unwrap();
    let total_supply_sig = [0x18, 0x16, 0x0d, 0xdd];
    let balance_of_sig = [0x70, 0xa0, 0x82, 0x31];
    let transfer_sig = [0xa9, 0x05, 0x9c, 0xbb];
    let call = move |ctx: &mut MiniEthereum, inp: &[u8]| {
        ctx.call(
            U256::zero(),
            contract_addr,
            CallInfo {
                origin: U256::zero(),
                caller: U256::zero(),
                call_value: U256::zero(),
                calldata: inp.to_vec(),
            },
        )
        .unwrap()
    };
}
