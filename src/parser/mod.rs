use std::error;
use std::fmt;

#[derive(Debug)]
pub struct ErrorDetail {
    line_content: String,
    line_number: i32,
    description: String,
}

#[derive(Debug)]
pub struct ParseError {
    content: String,
    errors: Vec<ErrorDetail>,
}

#[derive(PartialEq, Debug)]
pub struct EnvVar(String, String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse Error")
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        "parse error"
    }
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

pub type ParseResult = Result<Vec<EnvVar>, ParseError>;

pub mod env_file;
