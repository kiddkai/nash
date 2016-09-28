extern crate nix;
extern crate libc;
extern crate docopt;
extern crate rustc_serialize;

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

    let pid = child::spawn(cmd).unwrap();
    
    loop {
        match signals::wait_and_forward(pid) {
            Ok(signals::ForwardState::Forwarded) => {
                println!("Forwarded")
            },
            Ok(signals::ForwardState::ChildDead) => {
                println!("Child dead")
            },
            Err(e) => panic!("{:?}", e)
        }
    }
}
