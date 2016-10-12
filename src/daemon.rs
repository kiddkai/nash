use std::{error, result, fmt, convert};
use nix;
use child;
use signals;
use parser::*;

#[derive(Debug)]
pub enum DaemonError {
    SignalError(nix::Error),
    ChildError(nix::Error)
}

impl DaemonError {
    pub fn exit_code(&self) -> i32 {
        match *self {
            DaemonError::SignalError(_) => 128,
            DaemonError::ChildError(_) => 1
        }
    }
}

impl convert::From<nix::Error> for DaemonError {
    fn from(err: nix::Error) -> DaemonError {
        DaemonError::SignalError(err)
    }
}

impl error::Error for DaemonError {
    fn description(&self) -> &str {
        match *self {
            DaemonError::SignalError(ref e) => e.description(),
            DaemonError::ChildError(ref e) => e.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            DaemonError::SignalError(ref e) => e.cause(),
            DaemonError::ChildError(ref e) => e.cause()
        }
    }
}

impl fmt::Display for DaemonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DaemonError::SignalError(ref e) => write!(f, "SignalError {}", e),
            DaemonError::ChildError(ref e) => write!(f, "ChildError {}", e)
        }
    }
}

pub type Result<T> = result::Result<T, DaemonError>;

pub fn start(args: &Vec<String>, envs: &Vec<EnvVar>, should_kill_group: bool) -> Result<i32> {
    try!(signals::init());

    let mut command = child::parse_command(
        &args[0],
        &args.into_iter().skip(1).map(|s| s.to_owned()).collect(),
        &envs
    );

    let pid = child::spawn_command(&mut command).unwrap();

    loop {
        match try!(signals::wait_and_forward(pid, should_kill_group)){
            signals::ForwardState::ChildDead => trace!("The child process is dead"),
            signals::ForwardState::Forwarded => trace!("Signal forwarded to children"),
        }

        match try!(child::reap_zombies(pid)) {
            child::ReapState::Exit(code) => return Ok(code),
            _ => {}
        }
    }
}
