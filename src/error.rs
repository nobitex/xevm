use std::error::Error;

#[derive(Debug)]
pub enum XevmError {
    UnknownOpcode(u8),
    DidntFinish,
    Other(Box<dyn Error>),
}

impl From<Box<dyn Error>> for XevmError {
    fn from(value: Box<dyn Error>) -> Self {
        Self::Other(value)
    }
}

impl std::fmt::Display for XevmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for XevmError {
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            XevmError::Other(parent) => Some(parent.as_ref()),
            _ => None,
        }
    }
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            XevmError::Other(parent) => Some(parent.as_ref()),
            _ => None,
        }
    }
}
