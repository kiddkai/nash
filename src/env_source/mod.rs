use std::io;
use std::error;
use std::fmt;
use std::convert::From;

pub type EnvVar = (String, String);
pub type EnvVars = Vec<EnvVar>;

#[derive(Debug)]
pub enum SourceError {
    Io(io::Error),
    Nothing
}

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SourceError::Io(ref err) => write!(f, "IO Error {}", err),
            SourceError::Nothing => write!(f, "Something else")
        }
    }
}

impl error::Error for SourceError {
    fn description(&self) -> &str {
        match *self {
            SourceError::Io(ref err) => err.description(),
            SourceError::Nothing => "something else unknown"
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SourceError::Io(ref err) => err.cause(),
            SourceError::Nothing => None
        }
    }
}

impl From<io::Error> for SourceError {
    fn from(err: io::Error) -> SourceError {
        SourceError::Io(err)
    }
}

pub type FetchResult = Result<EnvVars, SourceError>;

pub trait Fetchable {
    fn fetch(&self) -> FetchResult;
}

pub mod file;
