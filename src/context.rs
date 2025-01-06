use crate::{
    error::{ExecError, RevertError},
    keccak::keccak,
    machine::{CallInfo, Machine},
    opcodes::ExecutionResult,
    u256::U256,
};
use std::{collections::HashMap, error::Error};

pub trait Context {
    fn create(&mut self, creator: U256, value: U256, code: Vec<u8>) -> Result<U256, ExecError>;
    fn create2(
        &mut self,
        creator: U256,
        value: U256,
        code: Vec<u8>,
        salt: U256,
    ) -> Result<U256, ExecError>;
    fn call(
        &mut self,
        _gas: U256,
        address: U256,
        call_info: CallInfo,
    ) -> Result<ExecutionResult, ExecError>;
    fn balance(&self, address: U256) -> Result<U256, Box<dyn Error>>;
    fn sload(&self, address: U256) -> Result<U256, Box<dyn Error>>;
    fn sstore(&mut self, address: U256, value: U256) -> Result<(), Box<dyn Error>>;
    fn log(&self, topics: Vec<U256>, data: Vec<u8>) -> Result<(), Box<dyn Error>>;
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
        self.accounts.entry(contract_addr).or_default();
        self.accounts.get_mut(&contract_addr).unwrap().value = value;

        let exec_result = Machine::new(contract_addr, code).run(
            self,
            &CallInfo {
                call_value: value,
                calldata: vec![],
                origin: U256::ZERO,
                caller: creator,
            },
        )?;

        match exec_result {
            ExecutionResult::Halted => {}
            ExecutionResult::Returned(code) => {
                self.accounts.get_mut(&contract_addr).unwrap().code = code;
            }
        }

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
        if self
            .accounts
            .get(&contract_addr)
            .map(|acc| !acc.code.is_empty())
            .unwrap_or_default()
        {
            return Err(ExecError::Revert(RevertError::ContractAlreadyDeployed));
        }
        self.accounts.entry(contract_addr).or_default();
        self.accounts.get_mut(&contract_addr).unwrap().value = value;

        let exec_result = Machine::new(contract_addr, code).run(
            self,
            &CallInfo {
                call_value: value,
                calldata: vec![],
                origin: U256::ZERO,
                caller: creator,
            },
        )?;
        match exec_result {
            ExecutionResult::Halted => {}
            ExecutionResult::Returned(code) => {
                self.accounts.get_mut(&contract_addr).unwrap().code = code;
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    /*
       pragma solidity ^0.8.13;

       contract Counter {
           uint256 public number;

           constructor() payable {}

           function setNumber(uint256 newNumber) public {
               number = newNumber;
           }

           function increment() public {
               number++;
           }
       }
    */
    const COUNTER_CODE: [u8; 252] = [
        96, 128, 96, 64, 82, 96, 236, 128, 97, 0, 16, 95, 57, 95, 243, 254, 96, 128, 96, 64, 82,
        52, 128, 21, 96, 14, 87, 95, 128, 253, 91, 80, 96, 4, 54, 16, 96, 58, 87, 95, 53, 96, 224,
        28, 128, 99, 63, 181, 193, 203, 20, 96, 62, 87, 128, 99, 131, 129, 245, 138, 20, 96, 79,
        87, 128, 99, 208, 157, 224, 138, 20, 96, 104, 87, 91, 95, 128, 253, 91, 96, 77, 96, 73, 54,
        96, 4, 96, 125, 86, 91, 95, 85, 86, 91, 0, 91, 96, 86, 95, 84, 129, 86, 91, 96, 64, 81,
        144, 129, 82, 96, 32, 1, 96, 64, 81, 128, 145, 3, 144, 243, 91, 96, 77, 95, 128, 84, 144,
        128, 96, 118, 131, 96, 147, 86, 91, 145, 144, 80, 85, 80, 86, 91, 95, 96, 32, 130, 132, 3,
        18, 21, 96, 140, 87, 95, 128, 253, 91, 80, 53, 145, 144, 80, 86, 91, 95, 96, 1, 130, 1, 96,
        175, 87, 99, 78, 72, 123, 113, 96, 224, 27, 95, 82, 96, 17, 96, 4, 82, 96, 36, 95, 253, 91,
        80, 96, 1, 1, 144, 86, 254, 162, 100, 105, 112, 102, 115, 88, 34, 18, 32, 139, 36, 42, 16,
        138, 0, 116, 178, 9, 210, 212, 42, 110, 151, 185, 78, 178, 48, 164, 149, 67, 3, 207, 184,
        215, 70, 118, 35, 201, 52, 39, 95, 100, 115, 111, 108, 99, 67, 0, 8, 26, 0, 51,
    ];

    #[test]
    fn test_counter_contract() {
        let mut ctx = DummyContext::default();
        ctx.accounts.entry(123.into()).or_insert(Account {
            nonce: 0.into(),
            value: 5.into(),
            code: vec![],
        });
        let contract_addr = ctx
            .create(123.into(), 2.into(), COUNTER_CODE.to_vec())
            .unwrap();
        let number_sig = [0x83, 0x81, 0xf5, 0x8a];
        let set_number_sig = [0x3f, 0xb5, 0xc1, 0xcb];
        let increment_sig = [0xd0, 0x9d, 0xe0, 0x8a];
        let call = move |ctx: &mut DummyContext, inp: &[u8]| {
            ctx.call(
                U256::ZERO,
                contract_addr,
                CallInfo {
                    origin: U256::ZERO,
                    caller: U256::ZERO,
                    call_value: U256::ZERO,
                    calldata: inp.to_vec(),
                },
            )
            .unwrap()
        };
        for i in 0..2000 {
            assert_eq!(
                call(&mut ctx, &number_sig),
                ExecutionResult::Returned(U256::from(i).to_bytes_be().to_vec())
            );
            assert_eq!(call(&mut ctx, &increment_sig), ExecutionResult::Halted);
        }
        let mut set_num_calldata = set_number_sig.to_vec();
        set_num_calldata.extend(U256::from(12345).to_bytes_be());
        assert_eq!(call(&mut ctx, &set_num_calldata), ExecutionResult::Halted);
        assert_eq!(
            call(&mut ctx, &number_sig),
            ExecutionResult::Returned(U256::from(12345).to_bytes_be().to_vec())
        );
    }

    #[test]
    fn test_context() {
        let mut ctx = DummyContext::default();
        ctx.create(123.into(), 0.into(), COUNTER_CODE.to_vec())
            .unwrap();
    }

    #[test]
    fn test_call() {
        let mut ctx = DummyContext::default();
        ctx.accounts.entry(123.into()).or_insert(Account {
            nonce: 0.into(),
            value: 5.into(),
            code: vec![],
        });
        ctx.call(
            U256::ZERO,
            234.into(),
            CallInfo {
                origin: U256::ZERO,
                caller: 123.into(),
                call_value: 2.into(),
                calldata: vec![],
            },
        )
        .unwrap();
        assert_eq!(ctx.accounts.get(&123.into()).unwrap().nonce, U256::from(1));
        assert_eq!(ctx.balance(123.into()).unwrap(), U256::from(3));
        assert_eq!(ctx.accounts.get(&234.into()).unwrap().nonce, U256::from(0));
        assert_eq!(ctx.balance(234.into()).unwrap(), U256::from(2));
        assert_eq!(
            ctx.call(
                U256::ZERO,
                234.into(),
                CallInfo {
                    origin: U256::ZERO,
                    caller: 123.into(),
                    call_value: 4.into(),
                    calldata: vec![],
                },
            ),
            Err(ExecError::Revert(RevertError::InsufficientBalance))
        );
    }

    #[test]
    fn test_create() {
        let mut ctx = DummyContext::default();
        ctx.accounts.entry(123.into()).or_insert(Account {
            nonce: 0.into(),
            value: 5.into(),
            code: vec![],
        });
        let contract_addr_1 = ctx
            .create(123.into(), 2.into(), COUNTER_CODE.to_vec())
            .unwrap();
        assert_eq!(ctx.accounts.get(&123.into()).unwrap().nonce, U256::from(1));
        let contract_addr_2 = ctx
            .create(123.into(), 2.into(), COUNTER_CODE.to_vec())
            .unwrap();
        assert_eq!(ctx.accounts.get(&123.into()).unwrap().nonce, U256::from(2));
        assert_eq!(ctx.balance(123.into()).unwrap(), U256::from(1));
        assert_eq!(ctx.balance(contract_addr_1).unwrap(), U256::from(2));
        assert_eq!(ctx.balance(contract_addr_2).unwrap(), U256::from(2));
        assert_eq!(
            contract_addr_1.to_string(),
            "0x000000000000000000000000838fea66b9b3aae5120d989b4ab767396f2fcbf1".to_string()
        );
        assert_eq!(
            contract_addr_2.to_string(),
            "0x000000000000000000000000ae7fac60782bb47c1e93a68b344aa5aff8a644ba".to_string()
        );
    }
    #[test]
    fn test_context_prevent_redeploy() {
        let mut ctx = DummyContext::default();
        let res1 = ctx.create2(123.into(), 0.into(), COUNTER_CODE.to_vec(), 123.into());
        let res2 = ctx.create2(123.into(), 0.into(), COUNTER_CODE.to_vec(), 234.into());
        assert!(res1.is_ok());
        assert!(res2.is_ok());
        assert_eq!(
            res1.unwrap().to_string(),
            "0x000000000000000000000000776fb1205e347d8388f4a39c9a2ca47d5afe0f41"
        );
        assert_eq!(
            res2.unwrap().to_string(),
            "0x000000000000000000000000554d4b57431778ac563b4f053bfd472a538edbe2"
        );
        assert_eq!(
            ctx.create2(123.into(), 0.into(), COUNTER_CODE.to_vec(), 123.into()),
            Err(ExecError::Revert(RevertError::ContractAlreadyDeployed))
        );
    }
}
