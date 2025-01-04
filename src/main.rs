use axum::extract::Path;
use axum::http::{HeaderValue, StatusCode};
use axum::response::Response;
use std::error::Error;
use std::str::FromStr;
use std::{collections::HashMap, fmt::Debug};
use xevm::error::ExecError;
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
    value: U256,
    code: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
pub struct DummyContext {
    contracts: HashMap<U256, Account>,
    mem: HashMap<U256, U256>,
}
impl Context for DummyContext {
    fn create(&mut self, value: U256, code: Vec<u8>) -> Result<U256, Box<dyn Error>> {
        Ok(U256::ZERO)
    }
    fn create2(&mut self, value: U256, code: Vec<u8>, salt: U256) -> Result<U256, Box<dyn Error>> {
        Ok(U256::ZERO)
    }
    fn call(
        &mut self,
        _gas: U256,
        address: U256,
        value: U256,
        args: Vec<u8>,
    ) -> Result<ExecutionResult, ExecError> {
        let contract = self.contracts.entry(address).or_default();
        contract.value = contract.value + value;
        let machine = Machine::new(contract.code.clone());
        let exec_result = machine.run(
            self,
            &CallInfo {
                call_value: value,
                origin: address,
                caller: address,
                calldata: args,
            },
        )?;
        Ok(exec_result)
    }
    fn balance(&self, _address: U256) -> Result<U256, Box<dyn Error>> {
        Ok(U256::ONE)
    }
    fn address(&self) -> Result<U256, Box<dyn Error>> {
        Ok(U256::ONE)
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

fn decode_string(inp: &[u8]) -> (u16, String, String) {
    let status_code = U256::from_bytes_be(&inp[32..32 + 32]).as_usize().unwrap() as u16;
    let sz_type = U256::from_bytes_be(&inp[96 + 32..96 + 32 + 32])
        .as_usize()
        .unwrap();
    let content_type =
        String::from_utf8(inp[96 + 32 + 32..96 + 32 + 32 + sz_type].to_vec()).unwrap();
    let sz_content = U256::from_bytes_be(&inp[160 + 32..160 + 32 + 32])
        .as_usize()
        .unwrap();
    let content =
        String::from_utf8(inp[160 + 32 + 32..160 + 32 + 32 + sz_content].to_vec()).unwrap();
    (status_code, content_type, content)
}

#[tokio::main]
async fn main() {
    let machine = Machine::new(parse_hex("608060405234801561000f575f80fd5b5060043610610034575f3560e01c8063999c38e914610038578063f9422b5a14610056575b5f80fd5b61004061005e565b60405161004d91906101b4565b60405180910390f35b6100406100f0565b61008160405180606001604052805f815260200160608152602001606081525090565b604051806060016040528060c88152602001604051806040016040528060098152602001681d195e1d0bda1d1b5b60ba1b8152508152602001604051806040016040528060158152602001741e34189f2432b63637903bb7b93632109e17b4189f60591b815250815250905090565b61011360405180606001604052805f815260200160608152602001606081525090565b604051806060016040528060c881526020016040518060400160405280601081526020016f30b8383634b1b0ba34b7b717b539b7b760811b8152508152602001604051806040016040528060128152602001717b2273616c616d223a2022646f6e7961227d60701b815250815250905090565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b60208152815160208201525f6020830151606060408401526101d96080840182610186565b90506040840151601f198483030160608501526101f68282610186565b9594505050505056fea26469706673582212208c0ebfaf78917c4e16d1b3223a9ea21372f1aa13ba3b81880aec5203ad0773ef64736f6c634300081a0033").unwrap());
    use axum::{routing::get, Router};
    let app = Router::new()
        // `GET /` goes to `root`
        .route(
            "/{*path}",
            get(|Path(path): Path<String>| async move {
                let mut ctx = DummyContext::default();
                let func_sig = format!("GET__{}()", path.replace("/", "__"));
                let func_hash = keccak(func_sig.as_bytes())[..4].to_vec();
                let res = machine
                    .clone()
                    .run(
                        &mut ctx,
                        &CallInfo {
                            origin: Default::default(),
                            caller: Default::default(),
                            call_value: Default::default(),
                            calldata: func_hash,
                        },
                    )
                    .unwrap();
                if let ExecutionResult::Returned(a) = res {
                    let (status_code, content_type, content) = decode_string(&a);
                    let mut resp = Response::new(content);
                    *resp.status_mut() = StatusCode::from_u16(status_code).unwrap();
                    resp.headers_mut().insert(
                        "Content-Type",
                        HeaderValue::from_str(&content_type).unwrap(),
                    );
                    resp
                } else {
                    let mut resp = Response::new("Not found!".to_string());
                    *resp.status_mut() = StatusCode::NOT_FOUND;
                    resp
                }
            }),
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
