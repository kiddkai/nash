use env_source::file::FileFetcher;
use super::{CliResult, CliError};

const USAGE: &'static str = "
Usage:
    nash [options] file [<file_args>...] -- <command> [<command_args>...]
Options:
    -h,       --help       Display this message
    -v,       --version    Print version info and exit
    -g,       --group      Forward signals to process group rather than the single process
Arguments:
    -f PATH,  --file PATH  The file reads environment variables from 
Example:
    nash file -f file://$HOME/v.env -- echo \"$FOO\"
";

pub fn execute(ref args: &Vec<String>) -> CliResult<FileFetcher> {
    let mut filePath: Option<String> = None;

    for (idx, s) in args.into_iter().enumerate() {
        match s.as_ref() {
            "-f" | "--file" => filePath = Some(args[idx + 1].clone()),
            "--" => { break; },
            _ => { continue; }
        }
    }
    
    match filePath {
        None => Err(CliError::BadArgument),
        Some(p) => Ok(FileFetcher::new(p.as_ref()))
    }
}
