#[macro_use]
extern crate log;
extern crate nix;
extern crate libc;
extern crate hyper;
extern crate docopt;
extern crate tempdir;
extern crate rustc_serialize;

use std::process;
use docopt::Docopt;

pub mod signals;
pub mod child;
pub mod env_source;
pub mod parser;
pub mod cli;

const USAGE: &'static str = "
Usage:
    nash [--from=<URL>] <cmd> [<args>...]
Options:
    -h,     --help       Display this message
    -V,     --version    Print version info and exit
    -v,     --verbose    Use verbose output
    -g,     --group      Forward signals to process group rather than the single process
    -f URL, --from=URL   URI to retrive the environments from
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_args: Vec<String>,
    arg_cmd: Option<String>,
    flag_group: bool,
    flag_verbose: bool,
    flag_from: Option<String>,
}


fn fetch_env(args: &Args) -> env_source::FetchResult {
    let from_url = args.flag_from.as_ref().unwrap();
    env_source::fetch(from_url)
}

fn main() {
    match cli::start() {
        Ok(_) => println!("OK"),
        Err(e) => panic!("{:?}", e)
    }
    //let args: Args = Docopt::new(USAGE)
    //    .and_then(|d| d.options_first(true).decode())
    //    .unwrap_or_else(|e| e.exit());

    //let envs = match fetch_env(&args) {
    //    Ok(envs) => envs,
    //    Err(_) => vec![]
    //};
    //bootstrap(&args, envs)
}
