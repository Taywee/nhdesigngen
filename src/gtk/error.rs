use std::convert::From;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    // Object not found for this label
    ObjectNotFound { label: String },
    Io(io::Error),
    NumberConversionError(Box<dyn error::Error>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ObjectNotFound { label, .. } => {
                write!(f, "Need object by id '{}' but couldn't find it", label)
            }
            Self::Io(e) => write!(f, "IO error encountered: {}", e),
            Self::NumberConversionError(e) => {
                write!(f, "Number conversion error encountered: {}", e)
            }
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

