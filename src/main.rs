#[macro_use]
extern crate log;
extern crate nix;
extern crate libc;
extern crate docopt;
extern crate rustc_serialize;

use std::process;
use docopt::Docopt;

mod signals;
mod child;

const USAGE: &'static str = "
Usage:
    nash [options] <cmd> [<args>...]
Options:
    -h, --help       Display this message
    -V, --version    Print version info and exit
    -v, --verbose    Use verbose output
    -g, --group      Forward signals to groups
    --from           URL to retrive the environments from
    --from-env       Envrionment variable contains the URL to retrive the environments from
Examples:
    nash ls -al
    nash --from s3://bucket/secrets/foo.json ls -al
    nash --from-env NASH_FROM ls -al
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_args: Vec<String>,
    arg_cmd: Option<String>,
    flag_group: bool,
    flag_verbose: bool,
    flag_from: Option<String>
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.options_first(true).decode())
                            .unwrap_or_else(|e| e.exit());

    match signals::init() {
        Ok(_) => {},
        Err(e) => panic!("failed to register initial signal handlers due to: {}", e)
    }

    let cmd = args.arg_cmd.unwrap();
    let should_kill_group = args.flag_group;
    let args = args.arg_args;
    let mut command = child::parse_command(cmd, &args);
    let pid = child::spawn_command(&mut command).unwrap();
    
    loop {
        match signals::wait_and_forward(pid, should_kill_group) {
            Err(e) => panic!("{:?}", e),
            Ok(signals::ForwardState::ChildDead) => { trace!("The child process is dead") },
            Ok(signals::ForwardState::Forwarded) => { trace!("Signal forwarded to children") }
        }

        match child::reap_zombies(pid) {
            Ok(child::ReapState::Next) => {},
            Ok(child::ReapState::Exit(code)) => {
                process::exit(code);
            },
            Err(e) => panic!("Failed to reap children {:?}", e)
        }
    }
}
