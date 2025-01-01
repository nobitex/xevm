use std::error::Error;
use std::{collections::HashMap, fmt::Debug};
mod u256;
use u256::U256;
mod opcodes;
use opcodes::*;

#[derive(Debug, Clone)]
enum XevmError {
    Other(String),
}

impl std::fmt::Display for XevmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl Error for XevmError {
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            XevmError::Other(_) => None,
        }
    }
    fn description(&self) -> &str {
        match self {
            XevmError::Other(other) => &other,
        }
    }
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            XevmError::Other(_) => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Machine {
    pc: usize,
    gas_used: usize,
    stack: Vec<U256>,
    transient: HashMap<U256, U256>,
}

impl Machine {
    fn pop_stack(&mut self) -> Result<U256, Box<dyn Error>> {
        Ok(self
            .stack
            .pop()
            .ok_or(XevmError::Other("Stack empty!".into()))?)
    }
}

trait OpcodeHandler<C: Context> {
    fn call(
        &self,
        ctx: &mut C,
        machine: &mut Machine,
        text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<(), Box<dyn Error>>;
}

fn run<C: Context>(
    ctx: &mut C,
    machine: &mut Machine,
    code: &[u8],
    call_info: CallInfo,
) -> Result<(), Box<dyn Error>> {
    let mut opcode_table: HashMap<u8, Box<dyn OpcodeHandler<C>>> = HashMap::new();
    opcode_table.insert(0x00, Box::new(OpcodeHalt));
    opcode_table.insert(0x01, Box::new(OpcodeAdd));
    opcode_table.insert(0x02, Box::new(OpcodeMul));

    opcode_table.insert(0x03, Box::new(OpcodeSub));
    /*opcode_table.insert(0x04, Box::new(OpcodeDiv));
    opcode_table.insert(0x05, Box::new(OpcodeSdiv));
    opcode_table.insert(0x06, Box::new(OpcodeMod));
    opcode_table.insert(0x07, Box::new(OpcodeSmod));
    opcode_table.insert(0x08, Box::new(OpcodeAddmod));
    opcode_table.insert(0x09, Box::new(OpcodeMulmod));
    opcode_table.insert(0x0a, Box::new(OpcodeExp));
    opcode_table.insert(0x0b, Box::new(OpcodeSignextend));*/

    opcode_table.insert(0x10, Box::new(OpcodeLt));
    opcode_table.insert(0x11, Box::new(OpcodeGt));
    //opcode_table.insert(0x12, Box::new(OpcodeSlt));
    //opcode_table.insert(0x13, Box::new(OpcodeSgt));
    opcode_table.insert(0x14, Box::new(OpcodeEq));
    opcode_table.insert(0x15, Box::new(OpcodeIszero));
    opcode_table.insert(0x16, Box::new(OpcodeAnd));
    opcode_table.insert(0x17, Box::new(OpcodeOr));
    /*opcode_table.insert(0x18, Box::new(OpcodeXor));
    opcode_table.insert(0x19, Box::new(OpcodeNot));
    opcode_table.insert(0x1a, Box::new(OpcodeByte));*/
    opcode_table.insert(0x1b, Box::new(OpcodeShl));
    opcode_table.insert(0x1c, Box::new(OpcodeShr));
    /*opcode_table.insert(0x1d, Box::new(OpcodeSar));*/

    opcode_table.insert(0x30, Box::new(OpcodeAddress));
    opcode_table.insert(0x31, Box::new(OpcodeBalance));
    /*opcode_table.insert(0x32, Box::new(OpcodeOrigin));*/
    opcode_table.insert(0x33, Box::new(OpcodeCaller));
    opcode_table.insert(0x34, Box::new(OpcodeCallvalue));
    /*opcode_table.insert(0x35, Box::new(OpcodeCalldataload));
    opcode_table.insert(0x36, Box::new(OpcodeCalldatasize));
    opcode_table.insert(0x37, Box::new(OpcodeCalldatacopy));*/
    opcode_table.insert(0x38, Box::new(OpcodeCodesize));
    opcode_table.insert(0x39, Box::new(OpcodeCodecopy));
    /*opcode_table.insert(0x3a, Box::new(OpcodeGasprice));
    opcode_table.insert(0x3b, Box::new(OpcodeExtcodesize));
    opcode_table.insert(0x3c, Box::new(OpcodeExtcodecopy));
    opcode_table.insert(0x3d, Box::new(OpcodeReturndatasize));
    opcode_table.insert(0x3e, Box::new(OpcodeReturndatacopy));
    opcode_table.insert(0x3f, Box::new(OpcodeExtcodehash));
    opcode_table.insert(0x40, Box::new(OpcodeBlockhash));
    opcode_table.insert(0x41, Box::new(OpcodeCoinbase));
    opcode_table.insert(0x42, Box::new(OpcodeTimestamp));
    opcode_table.insert(0x43, Box::new(OpcodeNumber));
    opcode_table.insert(0x44, Box::new(OpcodePrevrandao));
    opcode_table.insert(0x45, Box::new(OpcodeGaslimit));
    opcode_table.insert(0x46, Box::new(OpcodeChainid));
    opcode_table.insert(0x47, Box::new(OpcodeSelfbalance));
    opcode_table.insert(0x48, Box::new(OpcodeBasefee));
    opcode_table.insert(0x49, Box::new(OpcodeBlobhash));
    opcode_table.insert(0x4a, Box::new(OpcodeBlobbasefee));*/

    opcode_table.insert(0x50, Box::new(OpcodePop));
    opcode_table.insert(0x51, Box::new(OpcodeMload));
    opcode_table.insert(0x52, Box::new(OpcodeMstore));
    opcode_table.insert(0x56, Box::new(OpcodeJump));
    opcode_table.insert(0x57, Box::new(OpcodeJumpi));
    opcode_table.insert(0x5b, Box::new(OpcodeJumpdest));
    opcode_table.insert(0x5c, Box::new(OpcodeTload));
    opcode_table.insert(0x5d, Box::new(OpcodeTstore));
    for sz in 0..=32 {
        opcode_table.insert(0x5f + sz, Box::new(OpcodePush(sz)));
    }
    for sz in 0..16 {
        opcode_table.insert(0x80 + sz, Box::new(OpcodeDup(sz)));
    }
    for sz in 0..16 {
        opcode_table.insert(0x90 + sz, Box::new(OpcodeSwap(sz)));
    }
    opcode_table.insert(0xf3, Box::new(OpcodeReturn));

    while machine.pc < code.len() {
        let opcode = code[machine.pc];
        println!("{} 0x{:x}", machine.pc, opcode);
        println!("{:?}", machine.stack);
        if let Some(opcode_fn) = opcode_table.get(&opcode) {
            opcode_fn.call(ctx, machine, code, &call_info)?;
        } else {
            return Err(Box::new(XevmError::Other(format!(
                "Unknown opcode 0x{:02x}!",
                opcode
            ))));
        }
    }
    Ok(())
}

trait Context {
    fn address(&self) -> Result<U256, Box<dyn Error>>;
    fn balance(&self, address: U256) -> Result<U256, Box<dyn Error>>;
    fn mload(&self, address: U256) -> Result<U256, Box<dyn Error>>;
    fn mstore(&mut self, address: U256, value: U256) -> Result<(), Box<dyn Error>>;
}

struct CallInfo {
    pub caller: U256,
    pub call_value: U256,
}

#[derive(Clone, Debug, Default)]
struct DummyContext {
    mem: HashMap<U256, U256>,
}
impl Context for DummyContext {
    fn balance(&self, address: U256) -> Result<U256, Box<dyn Error>> {
        Ok(U256::ONE)
    }
    fn address(&self) -> Result<U256, Box<dyn Error>> {
        Ok(U256::ONE)
    }
    fn mload(&self, address: U256) -> Result<U256, Box<dyn Error>> {
        Ok(self.mem.get(&address).copied().unwrap_or_default())
    }
    fn mstore(&mut self, address: U256, value: U256) -> Result<(), Box<dyn Error>> {
        self.mem.insert(address, value);
        Ok(())
    }
}

fn parse_hex(s: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut ret = Vec::new();
    for i in (0..s.len()).step_by(2) {
        ret.push(u8::from_str_radix(&s[i..i + 2], 16)?);
    }
    Ok(ret)
}

fn main() {
    let code = parse_hex("608060405234801561001057600080fd5b50604051610cf6380380610cf683398101604081905261002f916102d8565b8383600361003d83826103fe565b50600461004a82826103fe565b50505061005d33826100ac60201b60201c565b600561006983826103fe565b507f9f9dee0161e88ffd7beb95907e0efbce0f492d3ed8fa931cfb215aaca528d53a308360405161009b9291906104bc565b60405180910390a150505050610525565b6001600160a01b0382166100db5760405163ec442f0560e01b8152600060048201526024015b60405180910390fd5b6100e7600083836100eb565b5050565b6001600160a01b03831661011657806002600082825461010b91906104fe565b909155506101889050565b6001600160a01b038316600090815260208190526040902054818110156101695760405163391434e360e21b81526001600160a01b038516600482015260248101829052604481018390526064016100d2565b6001600160a01b03841660009081526020819052604090209082900390555b6001600160a01b0382166101a4576002805482900390556101c3565b6001600160a01b03821660009081526020819052604090208054820190555b816001600160a01b0316836001600160a01b03167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef8360405161020891815260200190565b60405180910390a3505050565b634e487b7160e01b600052604160045260246000fd5b60005b8381101561024657818101518382015260200161022e565b50506000910152565b600082601f83011261026057600080fd5b81516001600160401b0381111561027957610279610215565b604051601f8201601f19908116603f011681016001600160401b03811182821017156102a7576102a7610215565b6040528181528382016020018510156102bf57600080fd5b6102d082602083016020870161022b565b949350505050565b600080600080608085870312156102ee57600080fd5b84516001600160401b0381111561030457600080fd5b6103108782880161024f565b602087015190955090506001600160401b0381111561032e57600080fd5b61033a8782880161024f565b604087015190945090506001600160401b0381111561035857600080fd5b6103648782880161024f565b606096909601519497939650505050565b600181811c9082168061038957607f821691505b6020821081036103a957634e487b7160e01b600052602260045260246000fd5b50919050565b601f8211156103f957806000526020600020601f840160051c810160208510156103d65750805b601f840160051c820191505b818110156103f657600081556001016103e2565b50505b505050565b81516001600160401b0381111561041757610417610215565b61042b816104258454610375565b846103af565b6020601f82116001811461045f57600083156104475750848201515b600019600385901b1c1916600184901b1784556103f6565b600084815260208120601f198516915b8281101561048f578785015182556020948501946001909201910161046f565b50848210156104ad5786840151600019600387901b60f8161c191681555b50505050600190811b01905550565b60018060a01b038316815260406020820152600082518060408401526104e981606085016020870161022b565b601f01601f1916919091016060019392505050565b8082018082111561051f57634e487b7160e01b600052601160045260246000fd5b92915050565b6107c2806105346000396000f3fe608060405234801561001057600080fd5b506004361061009e5760003560e01c8063313ce56711610066578063313ce5671461011157806370a082311461012057806395d89b4114610149578063a9059cbb14610151578063dd62ed3e1461016457600080fd5b806306fdde03146100a3578063095ea7b3146100c157806318160ddd146100e45780631f1bd692146100f657806323b872dd146100fe575b600080fd5b6100ab61019d565b6040516100b8919061060b565b60405180910390f35b6100d46100cf366004610675565b61022f565b60405190151581526020016100b8565b6002545b6040519081526020016100b8565b6100ab610249565b6100d461010c36600461069f565b6102d7565b604051601281526020016100b8565b6100e861012e3660046106dc565b6001600160a01b031660009081526020819052604090205490565b6100ab6102fb565b6100d461015f366004610675565b61030a565b6100e86101723660046106fe565b6001600160a01b03918216600090815260016020908152604080832093909416825291909152205490565b6060600380546101ac90610731565b80601f01602080910402602001604051908101604052809291908181526020018280546101d890610731565b80156102255780601f106101fa57610100808354040283529160200191610225565b820191906000526020600020905b81548152906001019060200180831161020857829003601f168201915b5050505050905090565b60003361023d818585610318565b60019150505b92915050565b6005805461025690610731565b80601f016020809104026020016040519081016040528092919081815260200182805461028290610731565b80156102cf5780601f106102a4576101008083540402835291602001916102cf565b820191906000526020600020905b8154815290600101906020018083116102b257829003601f168201915b505050505081565b6000336102e585828561032a565b6102f08585856103ad565b506001949350505050565b6060600480546101ac90610731565b60003361023d8185856103ad565b610325838383600161040c565b505050565b6001600160a01b0383811660009081526001602090815260408083209386168352929052205460001981146103a7578181101561039857604051637dc7a0d960e11b81526001600160a01b038416600482015260248101829052604481018390526064015b60405180910390fd5b6103a78484848403600061040c565b50505050565b6001600160a01b0383166103d757604051634b637e8f60e11b81526000600482015260240161038f565b6001600160a01b0382166104015760405163ec442f0560e01b81526000600482015260240161038f565b6103258383836104e1565b6001600160a01b0384166104365760405163e602df0560e01b81526000600482015260240161038f565b6001600160a01b03831661046057604051634a1406b160e11b81526000600482015260240161038f565b6001600160a01b03808516600090815260016020908152604080832093871683529290522082905580156103a757826001600160a01b0316846001600160a01b03167f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925846040516104d391815260200190565b60405180910390a350505050565b6001600160a01b03831661050c578060026000828254610501919061076b565b9091555061057e9050565b6001600160a01b0383166000908152602081905260409020548181101561055f5760405163391434e360e21b81526001600160a01b0385166004820152602481018290526044810183905260640161038f565b6001600160a01b03841660009081526020819052604090209082900390555b6001600160a01b03821661059a576002805482900390556105b9565b6001600160a01b03821660009081526020819052604090208054820190555b816001600160a01b0316836001600160a01b03167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef836040516105fe91815260200190565b60405180910390a3505050565b602081526000825180602084015260005b81811015610639576020818601810151604086840101520161061c565b506000604082850101526040601f19601f83011684010191505092915050565b80356001600160a01b038116811461067057600080fd5b919050565b6000806040838503121561068857600080fd5b61069183610659565b946020939093013593505050565b6000806000606084860312156106b457600080fd5b6106bd84610659565b92506106cb60208501610659565b929592945050506040919091013590565b6000602082840312156106ee57600080fd5b6106f782610659565b9392505050565b6000806040838503121561071157600080fd5b61071a83610659565b915061072860208401610659565b90509250929050565b600181811c9082168061074557607f821691505b60208210810361076557634e487b7160e01b600052602260045260246000fd5b50919050565b8082018082111561024357634e487b7160e01b600052601160045260246000fdfea264697066735822122091dd1118262332eb13f78f73601566e7a5756634f22aaa27a3594fbfd7f3509464736f6c634300081a0033").unwrap();
    let mut m = Machine::default();
    let mut ctx = DummyContext::default();

    let call_info = CallInfo {
        call_value: U256::ZERO,
        caller: U256::ZERO,
    };
    //m.stack.push(u256::ONE);
    run(&mut ctx, &mut m, &code, call_info).unwrap();
    println!("{:?}", m);
    println!("{:?}", ctx);
}
