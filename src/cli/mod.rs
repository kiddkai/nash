use std::{error, result, env, fmt};
use std::convert::From;
use env_source::{SourceError, Fetchable};

const USAGE: &'static str = "
Usage:
    nash [options] <source> [<source_args>...] -- <command> [<command_args>...]
Options:
    -h,     --help       Display this message
    -v,     --version    Print version info and exit
    -g,     --group      Forward signals to process group rather than the single process
Sources:
    file                 The local file source
    s3                   File source from s3
";

#[derive(Debug)]
struct CliOption {
    flag_group: bool,
    flag_help: bool,
    flag_version: bool,
    source: Option<String>,
    source_args: Vec<String>,
}

#[derive(Debug)]
struct CliOptionBuilder {
    is_after_cmd: bool,
    flag_group: bool,
    flag_help: bool,
    flag_version: bool,
    source: Option<String>,
    source_args: Vec<String>,
}

#[derive(Debug)]
pub enum CliError {
    Fetch(SourceError),
    BadArgument,
    UnknownCommand
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Fetch(ref e) => write!(f, "Failed to fetch {}", e),
            CliError::BadArgument => write!(f, "bad argument"),
            CliError::UnknownCommand => write!(f, "unknown command")
        }
    }
}

impl From<SourceError> for CliError {
    fn from(err: SourceError) -> CliError {
        CliError::Fetch(err)
    }
}

impl error::Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::Fetch(ref e) => e.description(),
            CliError::BadArgument => "bad argument",
            CliError::UnknownCommand => "unknown command"
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CliError::Fetch(ref e) => e.cause(),
            CliError::BadArgument => None,
            CliError::UnknownCommand => None
        }
    }
}

pub type CliResult<T> = Result<T, CliError>;

impl CliOptionBuilder {
    fn new() -> CliOptionBuilder {
        CliOptionBuilder {
            is_after_cmd: false,
            flag_version: false,
            flag_group: false,
            flag_help: false,
            source: None,
            source_args: vec![]
        }
    }

    fn to_cli_option(&self) -> CliOption {
        let source = self.source.as_ref().map(|s| s.clone());

        CliOption {
            flag_version: self.flag_version,
            flag_group: self.flag_group,
            flag_help: self.flag_help,
            source: source,
            source_args: self.source_args.clone()
        }
    }

    fn add(&mut self, s: &str) -> &mut CliOptionBuilder {
        if self.is_after_cmd {
            self.source_args.push(s.to_owned());
            return self;
        }

        match s {
            "-g" | "--group" => {
                self.flag_group = true;
                self
            }
            "-h" | "--help" => {
                self.flag_help = true;
                self
            }
            "-v" | "--version" => {
                self.flag_version = true;
                self
            }
            _ => {
                if s.chars().nth(0) == Some('-') {
                    return self;
                }
                self.source = Some(s.to_owned());
                self.is_after_cmd = true;
                self
            }
        }
    }
}

pub mod file;

pub fn start() -> CliResult<()> {
    let mut builder = CliOptionBuilder::new();

    for arg in env::args().skip(1) {
        builder.add(&arg);
    }

    let options = builder.to_cli_option();

    if options.flag_help {
        println!("{}", USAGE);
        return Ok(());
    }

    if options.source == None {
        println!("{}", USAGE);
        return Ok(());
    }

    let source = options.source.unwrap();

    let fetcher = match source.as_ref() {
        "file" => try!(file::execute(&options.source_args)),
        _ => return Err(CliError::UnknownCommand)
    };

    let envs = try!(fetcher.fetch());

    Ok(())
}

