use std::{error, env, fmt};
use std::convert::From;
use env_source::{SourceError, Fetchable};
use daemon;

pub mod file;
pub mod s3;

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
pub enum CliError {
    Fetch(SourceError),
    Daemon(daemon::DaemonError),
    BadArgument(String, String),
    UnknownSource(String, String),
    EmptySource,
    EmptyCommand
}

impl CliError {
    pub fn exit_code(&self) -> i32 {
        match *self {
            CliError::Daemon(ref e) => e.exit_code(),
            _ => 1
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Fetch(ref e) => write!(f, "Failed to fetch {}", e),
            CliError::Daemon(ref e) => write!(f, "Daemon error {}", e),
            CliError::BadArgument(ref arg, ref usage) => write!(f, "Bad argument {}.\n{}", arg, usage),
            CliError::UnknownSource(ref cmd, ref usage) => write!(f, "Unknown <source> {}.\n{}", cmd, usage),
            CliError::EmptySource => write!(f, "<source> is not specified.\n{}", USAGE),
            CliError::EmptyCommand => write!(f, "<command> is empty.\n{}", USAGE)
        }
    }
}

impl From<SourceError> for CliError { fn from(err: SourceError) -> CliError { CliError::Fetch(err) } }
impl From<daemon::DaemonError> for CliError { fn from(err: daemon::DaemonError) -> CliError { CliError::Daemon(err) } }

impl error::Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::Fetch(ref e) => e.description(),
            CliError::Daemon(ref e) => e.description(),
            CliError::BadArgument(_, _) => "BadArgument",
            CliError::UnknownSource(_, _) => "UnknownComand",
            CliError::EmptySource => "EmptySource",
            CliError::EmptyCommand => "EmptyCommand"
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CliError::Fetch(ref e) => e.cause(),
            CliError::Daemon(ref e) => e.cause(),
            CliError::BadArgument(_, _) => None,
            CliError::UnknownSource(_, _) => None,
            CliError::EmptySource => None,
            CliError::EmptyCommand => None
        }
    }
}

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug)]
struct CliOption {
    flag_group: bool,
    flag_help: bool,
    flag_version: bool,
    source: Option<String>,
    source_args: Vec<String>,
    command_args: Vec<String>
}

#[derive(Debug)]
struct CliOptionBuilder {
    is_after_cmd: bool,
    is_command_start: bool,
    flag_group: bool,
    flag_help: bool,
    flag_version: bool,
    source: Option<String>,
    source_args: Vec<String>,
    command_args: Vec<String>
}

impl CliOptionBuilder {
    fn new() -> CliOptionBuilder {
        CliOptionBuilder {
            is_after_cmd: false,
            is_command_start: false,
            flag_version: false,
            flag_group: false,
            flag_help: false,
            source: None,
            source_args: vec![],
            command_args: vec![]
        }
    }

    fn to_cli_option(&self) -> CliOption {
        let source = self.source.as_ref().map(|s| s.clone());

        CliOption {
            flag_version: self.flag_version,
            flag_group: self.flag_group,
            flag_help: self.flag_help,
            source: source,
            source_args: self.source_args.clone(),
            command_args: self.command_args.clone()
        }
    }

    fn add(&mut self, s: &str) -> &mut CliOptionBuilder {
        if self.is_after_cmd {
            if s == "--" {
                self.is_command_start = true;
                return self;
            }

            if self.is_command_start {
                self.command_args.push(s.to_owned());
            } else {
                self.source_args.push(s.to_owned());
            }
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


pub fn start() -> CliResult<i32> {
    let mut builder = CliOptionBuilder::new();

    for arg in env::args().skip(1) {
        builder.add(&arg);
    }
    let options = builder.to_cli_option();

    if options.flag_help {
        println!("{}", USAGE);
        return Ok(0);
    }

    if options.source == None {
        return Err(CliError::EmptySource);
    }

    let source = options.source.unwrap();
    let envs = match source.as_ref() {
        "file" => {
            let f = try!(file::execute(&options.source_args));
            try!(f.fetch())
        },
        "s3" => {
            let f = try!(s3::execute(&options.source_args));
            try!(f.fetch())
        }
        _ => return Err(CliError::UnknownSource(source.clone(), USAGE.to_owned()))
    };

    if options.command_args.len() < 1 {
        return Err(CliError::EmptyCommand);
    }

    let exit_code = try!(daemon::start(&options.command_args, &envs, options.flag_group));

    Ok(exit_code)
}

