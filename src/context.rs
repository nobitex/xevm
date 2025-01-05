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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context() {
        let mut ctx = DummyContext::default();
        ctx.create(123.into(), 0.into(), vec![1, 2, 3]).unwrap();
    }

    #[test]
    fn test_context_spend() {
        let mut ctx = DummyContext::default();
        ctx.accounts.entry(123.into()).or_insert(Account {
            nonce: 0.into(),
            value: 5.into(),
            code: vec![],
        });
        let contract_addr = ctx.create(123.into(), 2.into(), vec![1, 2, 3]).unwrap();
        assert_eq!(ctx.balance(123.into()).unwrap(), U256::from(3));
        assert_eq!(ctx.balance(contract_addr).unwrap(), U256::from(2));
    }
}
