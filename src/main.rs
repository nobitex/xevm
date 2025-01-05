use std::error::Error;
use std::{collections::HashMap, fmt::Debug};
use xevm::error::{ExecError, RevertError};
use xevm::keccak::keccak;
use xevm::machine::{CallInfo, Context, Machine};
use xevm::opcodes::ExecutionResult;
use xevm::u256::U256;

fn parse_hex(s: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut ret = Vec::new();
    for i in (0..s.len()).step_by(2) {
        ret.push(u8::from_str_radix(&s[i..i + 2], 16)?);
    }
    Ok(ret)
}

#[derive(Clone, Debug, Default)]
pub struct Account {
    nonce: U256,
    value: U256,
    code: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
pub struct DummyContext {
    accounts: HashMap<U256, Account>,
    mem: HashMap<U256, U256>,
}

fn rlp_address_nonce(addr: U256, nonce: U256) -> Vec<u8> {
    let mut rlp = vec![0x94u8];
    rlp.extend(&addr.to_bytes_be()[12..32]);
    if nonce < U256::from(128) {
        rlp.extend(&[nonce.as_usize().unwrap() as u8]);
    } else {
        let mut bytes = nonce.to_bytes_be().to_vec();
        while bytes[0] == 0 {
            bytes.remove(0);
        }
        rlp.push(0x80u8 + bytes.len() as u8);
        rlp.extend(&bytes);
    }
    rlp
}

impl Context for DummyContext {
    fn create(&mut self, creator: U256, value: U256, code: Vec<u8>) -> Result<U256, ExecError> {
        let acc = self.accounts.entry(creator).or_default();
        if acc.value >= value {
            acc.value = acc.value - value;
            acc.nonce = acc.nonce + U256::ONE;
        } else {
            return Err(ExecError::Revert(RevertError::InsufficientBalance));
        }
        let contract_addr =
            U256::from_bytes_be(&keccak(&rlp_address_nonce(creator, acc.nonce))[12..32]);
        acc.nonce = acc.nonce + U256::ONE;
        let cont = self.accounts.entry(contract_addr).or_default();
        cont.code = code;
        cont.value = value;
        Ok(contract_addr)
    }
    fn create2(
        &mut self,
        creator: U256,
        value: U256,
        code: Vec<u8>,
        salt: U256,
    ) -> Result<U256, ExecError> {
        let acc = self.accounts.entry(creator).or_default();
        if acc.value >= value {
            acc.value = acc.value - value;
            acc.nonce = acc.nonce + U256::ONE;
        } else {
            return Err(ExecError::Revert(RevertError::InsufficientBalance));
        }
        let mut inp = vec![0xffu8];
        inp.extend(&creator.to_bytes_be()[12..32]);
        inp.extend(&salt.to_bytes_be());
        inp.extend(&keccak(&code));
        let contract_addr = U256::from_bytes_be(&keccak(&inp)[12..32]);
        let cont = self.accounts.entry(contract_addr).or_default();
        cont.value = value;
        cont.code = code;
        Ok(contract_addr)
    }
    fn call(
        &mut self,
        _gas: U256,
        address: U256,
        call_info: CallInfo,
    ) -> Result<ExecutionResult, ExecError> {
        let caller = self.accounts.entry(call_info.caller).or_default();
        if caller.value >= call_info.call_value {
            caller.value = caller.value - call_info.call_value;
            caller.nonce = caller.nonce + U256::ONE;
        } else {
            return Err(ExecError::Revert(RevertError::InsufficientBalance));
        }
        let contract = self.accounts.entry(address).or_default();
        contract.value = contract.value + call_info.call_value;
        let machine = Machine::new(address, contract.code.clone());
        let exec_result = machine.run(self, &call_info)?;
        Ok(exec_result)
    }
    fn balance(&self, address: U256) -> Result<U256, Box<dyn Error>> {
        Ok(self
            .accounts
            .get(&address)
            .map(|a| a.value)
            .unwrap_or_default())
    }
    fn sload(&self, address: U256) -> Result<U256, Box<dyn Error>> {
        Ok(self.mem.get(&address).copied().unwrap_or_default())
    }
    fn sstore(&mut self, address: U256, value: U256) -> Result<(), Box<dyn Error>> {
        self.mem.insert(address, value);
        Ok(())
    }
    fn log(&self, topics: Vec<U256>, data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        println!("New log! {:?} {:?}", topics, data);
        Ok(())
    }
}

fn main() {
    let code = parse_hex("6080604052348015600e575f80fd5b5060043610603a575f3560e01c80633fb5c1cb14603e5780638381f58a14604f578063d09de08a146068575b5f80fd5b604d6049366004607d565b5f55565b005b60565f5481565b60405190815260200160405180910390f35b604d5f805490806076836093565b9190505550565b5f60208284031215608c575f80fd5b5035919050565b5f6001820160af57634e487b7160e01b5f52601160045260245ffd5b506001019056fea264697066735822122055d88f9afbd1174cf472eb6254c3e131741fcc6117353bafc4aa81bf1af88e0264736f6c634300081a0033").unwrap();
    let mut ctx = DummyContext::default();
    let call_info = CallInfo {
        origin: U256::ZERO,
        call_value: U256::ZERO,
        caller: U256::ZERO,
        calldata: vec![0xd0, 0x9d, 0xe0, 0x8a],
    };
    let res = Machine::new(U256::ZERO, code.clone())
        .run(&mut ctx, &call_info)
        .unwrap();
    println!("{:?}", res);
    let res = Machine::new(U256::ZERO, code.clone())
        .run(&mut ctx, &call_info)
        .unwrap();
    println!("{:?}", res);
    let res = Machine::new(U256::ZERO, code.clone())
        .run(&mut ctx, &call_info)
        .unwrap();
    println!("{:?}", res);

    let call_info = CallInfo {
        origin: U256::ZERO,
        call_value: U256::ZERO,
        caller: U256::ZERO,
        calldata: vec![
            0x3f, 0xb5, 0xc1, 0xcb, 0xf7, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77,
            0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77,
            0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x7f,
        ],
    };
    let res = Machine::new(U256::ZERO, code.clone())
        .run(&mut ctx, &call_info)
        .unwrap();
    println!("{:?}", res);
    println!("{:?}", ctx);
}
