use std::{error, result, fmt, convert};
use nix;
use child;
use signal;
use env_source::*;
use parser::*;

#[derive(Debug)]
enum DaemonError {
    SignalError(nix::Error)
}

impl DaemonError {
    fn exit_code(&self) -> i32 {
        match *self {
            DaemonError::SignalError(ref e) => 128
        }
    }
}

impl convert::From<nix::Error> for DaemonError {
    fn from(err: nix::Err) -> DaemonError {
        DaemonError::SignalError(err)
    }
}

impl error::Error for DaemonError {
    fn description(&self) -> &str {
        match *self {
            DaemonError::SignalError(ref e) => e.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            DaemonError::SignalError(ref e) => e.cause()
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DaemonError::SignalError(ref e) => write!(f, "DaemonError {}", e),
        }
    }
}

type Result<T> = result::Result<T, DaemonError>;

pub fn start(args: &Vec<String>, envs: Vec<parser::EnvVar>) -> Result<()> {
    try!(signals::init());

    let should_kill_group = args.flag_group;
    let arg_cmd = args.arg_cmd.as_ref();
    let mut command = child::parse_command(
        arg_cmd.unwrap(),
        &args.arg_args,
        &envs
    );

    let pid = child::spawn_command(&mut command).unwrap();

    loop {
        match signals::wait_and_forward(pid, should_kill_group) {
            Err(e) => panic!("{:?}", e),
            Ok(signals::ForwardState::ChildDead) => trace!("The child process is dead"),
            Ok(signals::ForwardState::Forwarded) => trace!("Signal forwarded to children"),
        }

        match child::reap_zombies(pid) {
            Ok(child::ReapState::Next) => {}
            Ok(child::ReapState::Exit(code)) => {
                process::exit(code);
            }
            Err(e) => panic!("Failed to reap children {:?}", e),
        }
    }
}
