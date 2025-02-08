use alloy_primitives::primitives::Address;

use crate::{
    error::{ExecError, RevertError},
    keccak::keccak,
    machine::{CallInfo, GasTracker, Machine, Word},
    opcodes::ExecutionResult,
    u256::U256,
};
use std::{collections::HashMap, error::Error};

#[derive(Debug, Clone, Copy)]
pub enum Info {
    GasPrice,
    Coinbase,
    Timestamp,
    Number,
    PrevRandao,
    GasLimit,
    ChainId,
    BaseFee,
    BlobBaseFee,
}

pub trait Context<W: Word> {
    type Mutable: ContextMut<W>;
    fn destroy(&self, contract: W::Addr, target: W::Addr) -> Result<(), ExecError>;
    fn code(&self, address: W::Addr) -> Result<Vec<u8>, Box<dyn Error>>;
    fn blob_hash(&self, index: W) -> Result<W, Box<dyn Error>>;
    fn block_hash(&self, block_number: W) -> Result<W, Box<dyn Error>>;
    fn info(&self, inf: Info) -> Result<W, Box<dyn Error>>;
    fn balance(&self, address: W::Addr) -> Result<W, Box<dyn Error>>;
    fn sload(&self, contract: W::Addr, address: W) -> Result<W, Box<dyn Error>>;
    fn tload(&self, address: W) -> Result<W, Box<dyn Error>>;
    fn as_mut(&mut self) -> &mut Self::Mutable;
}

pub trait ContextMut<W: Word>: Context<W> {
    fn create(
        &mut self,
        stack_size: usize,
        gas_tracker: &mut GasTracker,
        call_info: CallInfo<W>,
        salt: Option<W>,
    ) -> Result<W::Addr, ExecError>;
    fn call(
        &mut self,
        stack_size: usize,
        gas_tracker: &mut GasTracker,
        address: W::Addr,
        call_info: CallInfo<W>,
    ) -> Result<ExecutionResult, ExecError>;
    fn sstore(&mut self, contract: W::Addr, address: W, value: W) -> Result<(), Box<dyn Error>>;
    fn tstore(&mut self, address: W, value: W) -> Result<(), Box<dyn Error>>;
    fn log(
        &mut self,
        address: W::Addr,
        topics: Vec<W>,
        data: Vec<u8>,
    ) -> Result<(), Box<dyn Error>>;
}

#[derive(Clone, Debug, Default)]
pub struct Account {
    pub nonce: U256,
    pub value: U256,
    pub code: Vec<u8>,
    pub storage: HashMap<U256, U256>,
}

#[derive(Clone, Default)]
pub struct MiniEthereum {
    precompiles:
        HashMap<Address, &'static dyn Fn(CallInfo<U256>) -> Result<ExecutionResult, ExecError>>,
    pub accounts: HashMap<Address, Account>,
    pub transient: HashMap<U256, U256>,
}

fn rlp_address_nonce(addr: Address, nonce: U256) -> Vec<u8> {
    let mut rlp = vec![0x94u8];
    rlp.extend(addr.as_slice());
    if nonce < U256::from(128) {
        rlp.extend(&[nonce.low_u64() as u8]);
    } else {
        let mut bytes = nonce.to_big_endian().to_vec();
        while bytes[0] == 0 {
            bytes.remove(0);
        }
        rlp.push(0x80u8 + bytes.len() as u8);
        rlp.extend(&bytes);
    }
    rlp
}

fn ecrecover(_call_info: CallInfo<U256>) -> Result<ExecutionResult, ExecError> {
    Err(ExecError::Revert(RevertError::UnknownOpcode(0x0)))
}

impl MiniEthereum {
    pub fn new() -> Self {
        let ecrecover: &'static dyn Fn(CallInfo<U256>) -> Result<ExecutionResult, ExecError> =
            &ecrecover;
        Self {
            precompiles: [(Address::ZERO, ecrecover)].into_iter().collect(),
            accounts: HashMap::new(),
            transient: HashMap::new(),
        }
    }
}

impl Context<U256> for MiniEthereum {
    type Mutable = Self;
    fn tload(&self, address: U256) -> Result<U256, Box<dyn Error>> {
        Ok(self.transient.get(&address).cloned().unwrap_or_default())
    }
    fn as_mut(&mut self) -> &mut Self::Mutable {
        self
    }
    fn destroy(&self, _contract: Address, _target: Address) -> Result<(), ExecError> {
        Err(ExecError::Revert(RevertError::UnknownOpcode(0xff)))
    }
    fn blob_hash(&self, _index: U256) -> Result<U256, Box<dyn Error>> {
        Ok(U256::ZERO)
    }
    fn code(&self, address: Address) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(self
            .accounts
            .get(&address)
            .map(|a| a.code.clone())
            .unwrap_or_default())
    }
    fn block_hash(&self, _block_number: U256) -> Result<U256, Box<dyn Error>> {
        Ok(U256::ZERO)
    }
    fn info(&self, inf: Info) -> Result<U256, Box<dyn Error>> {
        match inf {
            _ => Ok(U256::ZERO),
        }
    }

    fn balance(&self, address: Address) -> Result<U256, Box<dyn Error>> {
        Ok(self
            .accounts
            .get(&address)
            .map(|a| a.value)
            .unwrap_or_default())
    }
    fn sload(&self, contract: Address, address: U256) -> Result<U256, Box<dyn Error>> {
        Ok(if let Some(acc) = self.accounts.get(&contract) {
            acc.storage.get(&address).copied().unwrap_or_default()
        } else {
            Default::default()
        })
    }
}

impl ContextMut<U256> for MiniEthereum {
    fn tstore(&mut self, address: U256, value: U256) -> Result<(), Box<dyn Error>> {
        self.transient.insert(address, value);
        Ok(())
    }
    fn create(
        &mut self,
        stack_size: usize,
        gas_tracker: &mut GasTracker,
        call_info: CallInfo<U256>,
        salt: Option<U256>,
    ) -> Result<Address, ExecError> {
        let acc = self.accounts.entry(call_info.caller).or_default();
        if acc.value >= call_info.call_value {
            acc.value = acc.value - call_info.call_value;
            acc.nonce = acc.nonce + U256::ONE;
        } else {
            return Err(ExecError::Revert(RevertError::InsufficientBalance));
        }
        let contract_addr = if let Some(salt) = salt {
            let mut inp = vec![0xffu8];
            inp.extend(call_info.caller.as_slice());
            inp.extend(&salt.to_big_endian());
            inp.extend(&keccak(&call_info.calldata));
            Address::from_slice(&keccak(&inp)[12..32])
        } else {
            Address::from_slice(&keccak(&rlp_address_nonce(call_info.caller, acc.nonce))[12..32])
        };

        if self
            .accounts
            .get(&contract_addr)
            .map(|a| a.code.len())
            .unwrap_or_default()
            != 0
        {
            return Err(ExecError::Revert(RevertError::ContractAlreadyDeployed));
        }

        self.accounts.entry(contract_addr).or_default();
        self.accounts.get_mut(&contract_addr).unwrap().value = call_info.call_value;

        let exec_result = Machine::new(contract_addr, call_info.calldata, gas_tracker, stack_size)
            .run(
                self,
                &CallInfo {
                    call_value: call_info.call_value,
                    calldata: vec![],
                    origin: call_info.origin,
                    caller: call_info.caller,
                    is_static: false,
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
        stack_size: usize,
        gas_tracker: &mut GasTracker,
        address: Address,
        call_info: CallInfo<U256>,
    ) -> Result<ExecutionResult, ExecError> {
        if let Some(precompile) = self.precompiles.get(&address) {
            return precompile(call_info);
        }
        let caller = self.accounts.entry(call_info.caller).or_default();
        if caller.value >= call_info.call_value {
            caller.value = caller.value - call_info.call_value;
            caller.nonce = caller.nonce + U256::ONE;
        } else {
            return Err(ExecError::Revert(RevertError::InsufficientBalance));
        }
        let contract = self.accounts.entry(address).or_default();
        contract.value = contract.value + call_info.call_value;
        let machine = Machine::new(address, contract.code.clone(), gas_tracker, stack_size);
        let exec_result = machine.run(self, &call_info)?;
        Ok(exec_result)
    }
    fn sstore(
        &mut self,
        contract: Address,
        address: U256,
        value: U256,
    ) -> Result<(), Box<dyn Error>> {
        self.accounts
            .entry(contract)
            .or_default()
            .storage
            .insert(address, value);
        Ok(())
    }
    fn log(
        &mut self,
        address: Address,
        topics: Vec<U256>,
        data: Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        println!("New log from {}: {:?} {:?}", address, topics, data);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn addr(v: u8) -> Address {
        let mut arr = [0u8; 20];
        arr[19] = v;
        Address::from_slice(&arr)
    }

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
        let mut gt = GasTracker::new(10000000);
        let mut ctx = MiniEthereum::default();
        ctx.accounts.entry(addr(123)).or_insert(Account {
            nonce: U256::from_u64(0),
            value: U256::from_u64(5),
            code: vec![],
            storage: Default::default(),
        });
        let contract_addr = ctx
            .create(
                1024,
                &mut gt,
                CallInfo {
                    origin: addr(123),
                    caller: addr(123),
                    call_value: U256::from_u64(2),
                    calldata: COUNTER_CODE.to_vec(),
                    is_static: false,
                },
                None,
            )
            .unwrap();
        let number_sig = [0x83, 0x81, 0xf5, 0x8a];
        let set_number_sig = [0x3f, 0xb5, 0xc1, 0xcb];
        let increment_sig = [0xd0, 0x9d, 0xe0, 0x8a];
        let call = move |ctx: &mut MiniEthereum, inp: &[u8]| {
            let mut gt = GasTracker::new(10000000);
            ctx.call(
                1024,
                &mut gt,
                contract_addr,
                CallInfo {
                    origin: Address::ZERO,
                    caller: Address::ZERO,
                    call_value: U256::ZERO,
                    calldata: inp.to_vec(),
                    is_static: false,
                },
            )
            .unwrap()
        };
        for i in 0..2000 {
            assert_eq!(
                call(&mut ctx, &number_sig),
                ExecutionResult::Returned(U256::from(i).to_big_endian().to_vec())
            );
            assert_eq!(call(&mut ctx, &increment_sig), ExecutionResult::Halted);
        }
        let mut set_num_calldata = set_number_sig.to_vec();
        set_num_calldata.extend(U256::from(12345).to_big_endian());
        assert_eq!(call(&mut ctx, &set_num_calldata), ExecutionResult::Halted);
        assert_eq!(
            call(&mut ctx, &number_sig),
            ExecutionResult::Returned(U256::from(12345).to_big_endian().to_vec())
        );
    }

    #[test]
    fn test_context() {
        let mut gt = GasTracker::new(10000000);
        let mut ctx = MiniEthereum::default();
        ctx.create(
            1024,
            &mut gt,
            CallInfo {
                origin: addr(123),
                caller: addr(123),
                call_value: U256::from_u64(0),
                calldata: COUNTER_CODE.to_vec(),
                is_static: false,
            },
            None,
        )
        .unwrap();
    }

    #[test]
    fn test_call() {
        let mut gt = GasTracker::new(10000000);
        let mut ctx = MiniEthereum::default();
        ctx.accounts.entry(addr(123)).or_insert(Account {
            nonce: U256::from_u64(0),
            value: U256::from_u64(5),
            code: vec![],
            storage: Default::default(),
        });
        ctx.call(
            1024,
            &mut gt,
            addr(234),
            CallInfo {
                origin: Address::ZERO,
                caller: addr(123),
                call_value: U256::from_u64(2),
                calldata: vec![],
                is_static: false,
            },
        )
        .unwrap();
        assert_eq!(ctx.accounts.get(&addr(123)).unwrap().nonce, U256::from(1));
        assert_eq!(ctx.balance(addr(123)).unwrap(), U256::from(3));
        assert_eq!(ctx.accounts.get(&addr(234)).unwrap().nonce, U256::from(0));
        assert_eq!(ctx.balance(addr(234)).unwrap(), U256::from(2));
        assert_eq!(
            ctx.call(
                1024,
                &mut gt,
                addr(234),
                CallInfo {
                    origin: Address::ZERO,
                    caller: addr(123),
                    call_value: U256::from_u64(4),
                    calldata: vec![],
                    is_static: false
                },
            ),
            Err(ExecError::Revert(RevertError::InsufficientBalance))
        );
    }

    #[test]
    fn test_create() {
        let mut gt = GasTracker::new(10000000);
        let mut ctx = MiniEthereum::default();
        ctx.accounts.entry(addr(123)).or_insert(Account {
            nonce: U256::from_u64(0),
            value: U256::from_u64(5),
            code: vec![],
            storage: Default::default(),
        });
        let contract_addr_1 = ctx
            .create(
                1024,
                &mut gt,
                CallInfo {
                    origin: addr(123),
                    caller: addr(123),
                    call_value: U256::from_u64(2),
                    calldata: COUNTER_CODE.to_vec(),
                    is_static: false,
                },
                None,
            )
            .unwrap();
        assert_eq!(ctx.accounts.get(&addr(123)).unwrap().nonce, U256::from(1));
        let contract_addr_2 = ctx
            .create(
                1024,
                &mut gt,
                CallInfo {
                    origin: addr(123),
                    caller: addr(123),
                    call_value: U256::from_u64(2),
                    calldata: COUNTER_CODE.to_vec(),
                    is_static: false,
                },
                None,
            )
            .unwrap();
        assert_eq!(ctx.accounts.get(&addr(123)).unwrap().nonce, U256::from(2));
        assert_eq!(ctx.balance(addr(123)).unwrap(), U256::from(1));
        assert_eq!(ctx.balance(contract_addr_1).unwrap(), U256::from(2));
        assert_eq!(ctx.balance(contract_addr_2).unwrap(), U256::from(2));
        assert_eq!(
            contract_addr_1.to_string(),
            "0x838FeA66B9b3aAe5120D989B4Ab767396f2fcBf1".to_string()
        );
        assert_eq!(
            contract_addr_2.to_string(),
            "0xae7FaC60782BB47C1E93A68b344Aa5AFF8A644bA".to_string()
        );
    }
    #[test]
    fn test_context_prevent_redeploy() {
        let mut gt = GasTracker::new(10000000);
        let mut ctx = MiniEthereum::default();
        let res1 = ctx.create(
            1024,
            &mut gt,
            CallInfo {
                origin: addr(123),
                caller: addr(123),
                call_value: U256::from_u64(0),
                calldata: COUNTER_CODE.to_vec(),
                is_static: false,
            },
            Some(U256::from_u64(123)),
        );
        let res2 = ctx.create(
            1024,
            &mut gt,
            CallInfo {
                origin: addr(123),
                caller: addr(123),
                call_value: U256::from_u64(0),
                calldata: COUNTER_CODE.to_vec(),
                is_static: false,
            },
            Some(U256::from_u64(234)),
        );
        assert!(res1.is_ok());
        assert!(res2.is_ok());
        assert_eq!(
            res1.unwrap().to_string(),
            "0x776fb1205e347D8388F4A39C9a2CA47d5AfE0f41"
        );
        assert_eq!(
            res2.unwrap().to_string(),
            "0x554d4b57431778ac563B4f053bFd472a538edBe2"
        );
        assert_eq!(
            ctx.create(
                1024,
                &mut gt,
                CallInfo {
                    origin: addr(123),
                    caller: addr(123),
                    call_value: U256::from_u64(0),
                    calldata: COUNTER_CODE.to_vec(),
                    is_static: false
                },
                Some(U256::from_u64(123))
            ),
            Err(ExecError::Revert(RevertError::ContractAlreadyDeployed))
        );
    }
}
