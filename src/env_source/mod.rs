use std::io;
use std::fmt;
use std::error;
use parser::{self, EnvVar};
use std::convert::From;

#[derive(Debug)]
pub enum SourceError {
    Io(io::Error),
    Parse(parser::ParseError),
    Nothing,
}

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SourceError::Parse(ref err) => write!(f, "Parse Error {}", err),
            SourceError::Io(ref err) => write!(f, "IO Error {}", err),
            SourceError::Nothing => write!(f, "Something else"),
        }
    }
}

impl error::Error for SourceError {
    fn description(&self) -> &str {
        match *self {
            SourceError::Parse(ref err) => err.description(),
            SourceError::Io(ref err) => err.description(),
            SourceError::Nothing => "something else unknown",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SourceError::Parse(ref err) => err.cause(),
            SourceError::Io(ref err) => err.cause(),
            SourceError::Nothing => None,
        }
    }
}

impl From<io::Error> for SourceError {
    fn from(err: io::Error) -> SourceError {
        SourceError::Io(err)
    }
}

impl From<parser::ParseError> for SourceError {
    fn from(err: parser::ParseError) -> SourceError {
        SourceError::Parse(err)
    }
}

pub type FetchResult = Result<Vec<EnvVar>, SourceError>;

pub trait Fetchable {
    fn fetch(&self) -> FetchResult;
}

pub mod file;
