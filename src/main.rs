#[macro_use]
extern crate log;
extern crate nix;
extern crate libc;
extern crate hyper;
extern crate tempdir;

pub mod signals;
pub mod child;
pub mod env_source;
pub mod parser;
pub mod cli;
pub mod daemon;

use std::process;

fn main() {
    match cli::start() {
        Ok(code) => process::exit(code),
        Err(e) => panic!("{}", e)
    }
}
