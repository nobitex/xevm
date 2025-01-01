use std::error::Error;

use crate::CallInfo;
use crate::Context;
use crate::Machine;
use crate::OpcodeHandler;
use crate::XevmError;

#[derive(Debug)]
pub struct OpcodeReturn;
impl<C: Context> OpcodeHandler<C> for OpcodeReturn {
    fn call(
        &self,
        _ctx: &mut C,
        machine: &mut Machine,
        _text: &[u8],
        _call_info: &CallInfo,
    ) -> Result<(), Box<dyn Error>> {
        let offset = machine.pop_stack()?;
        let sz = machine.pop_stack()?;
        Err(Box::new(XevmError::Other("Returned!".into())))
    }
}
