use std::error::Error;

#[derive(Debug)]
pub enum ExecError {
    Revert(RevertError),
    Context(Box<dyn Error>),
}

impl From<anyhow::Error> for ExecError {
    fn from(value: anyhow::Error) -> Self {
        ExecError::Context(value.into())
    }
}

unsafe impl Sync for ExecError {}
unsafe impl Send for ExecError {}

impl PartialEq for ExecError {
    fn eq(&self, other: &Self) -> bool {
        if let ExecError::Revert(a) = self {
            if let ExecError::Revert(b) = other {
                return a == b;
            }
        }
        false
    }
}

#[derive(Debug, PartialEq)]
pub enum RevertError {
    UnknownOpcode(u8),
    NotEnoughValuesOnStack,
    NotEnoughBytesInCode,
    OffsetSizeTooLarge,
    InvalidJump,
    Revert(Vec<u8>),
    InsufficientBalance,
    ContractAlreadyDeployed,
    BlockHashUnavailable,
    ReturnDataUnavailable,
    OutOfBounds,
    InsufficientGas,
    StackFull,
    CannotMutateStatic,
}

impl From<Box<dyn Error>> for ExecError {
    fn from(value: Box<dyn Error>) -> Self {
        Self::Context(value)
    }
}

impl From<RevertError> for ExecError {
    fn from(value: RevertError) -> Self {
        Self::Revert(value)
    }
}

impl std::fmt::Display for ExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for RevertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RevertError {}

impl Error for ExecError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ExecError::Context(parent) => Some(parent.as_ref()),
            ExecError::Revert(rev) => Some(rev),
        }
    }
}
