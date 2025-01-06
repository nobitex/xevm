use std::error::Error;
use xevm::context::DummyContext;
use xevm::machine::{CallInfo, Machine};
use xevm::u256::U256;

fn parse_hex(s: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut ret = Vec::new();
    for i in (0..s.len()).step_by(2) {
        ret.push(u8::from_str_radix(&s[i..i + 2], 16)?);
    }
    Ok(ret)
}

fn main() {
    let code = parse_hex("6080604052348015600e575f80fd5b5060043610603a575f3560e01c80633fb5c1cb14603e5780638381f58a14604f578063d09de08a146068575b5f80fd5b604d6049366004607d565b5f55565b005b60565f5481565b60405190815260200160405180910390f35b604d5f805490806076836093565b9190505550565b5f60208284031215608c575f80fd5b5035919050565b5f6001820160af57634e487b7160e01b5f52601160045260245ffd5b506001019056fea264697066735822122055d88f9afbd1174cf472eb6254c3e131741fcc6117353bafc4aa81bf1af88e0264736f6c634300081a0033").unwrap();
    let mut ctx = DummyContext::default();
    let call_info = CallInfo {
        origin: U256::zero(),
        call_value: U256::zero(),
        caller: U256::zero(),
        calldata: vec![0xd0, 0x9d, 0xe0, 0x8a],
    };
    let res = Machine::new(U256::zero(), code.clone())
        .run(&mut ctx, &call_info)
        .unwrap();
    println!("{:?}", res);
    let res = Machine::new(U256::zero(), code.clone())
        .run(&mut ctx, &call_info)
        .unwrap();
    println!("{:?}", res);
    let res = Machine::new(U256::zero(), code.clone())
        .run(&mut ctx, &call_info)
        .unwrap();
    println!("{:?}", res);

    let call_info = CallInfo {
        origin: U256::zero(),
        call_value: U256::zero(),
        caller: U256::zero(),
        calldata: vec![
            0x3f, 0xb5, 0xc1, 0xcb, 0xf7, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77,
            0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77,
            0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x7f,
        ],
    };
    let res = Machine::new(U256::zero(), code.clone())
        .run(&mut ctx, &call_info)
        .unwrap();
    println!("{:?}", res);
    println!("{:?}", ctx);
}
