use env_source::s3::S3Fetcher;
use super::{CliResult, CliError};

const USAGE: &'static str = "
Usage:
    nash [options] s3 [<s3_args>...] -- <command> [<command_args>...]
Options:
    -h,         --help             Display this message
    -v,         --version          Print version info and exit
    -g,         --group            Forward signals to process group rather than the single process
Arguments:
    -b BUCKET   --bucket BUCKET    The bucket name to fetch the file from
    -o OBJECT   --object OBJECT    The object key to fetch the file form
    -r REGION   --region REGION    The aws region name
Example:
    nash s3 -b prod_secrets -o service_name.env -- printenv
";

pub fn execute(ref args: &Vec<String>) -> CliResult<S3Fetcher> {
    let mut bucket_name: Option<String> = None;
    let mut object_key: Option<String> = None;
    let mut region_name: Option<String> = None;

    for (arg, value) in args.into_iter().zip(args.into_iter().skip(1)) {
        match arg.as_ref() {
            "-b" | "--bucket" => bucket_name = Some(value.to_owned()),
            "-o" | "--object" => object_key = Some(value.to_owned()),
            "-r" | "--region" => region_name = Some(value.to_owned()),
            _ => { continue; }
        }
    }
    
    match bucket_name {
        None => return Err(CliError::BadArgument("--bucket is not specified".to_string(), USAGE.to_string())),
        _ => {}
    }

    match object_key {
        None => return Err(CliError::BadArgument("--object is not specified".to_string(), USAGE.to_string())),
        _ => {}
    }

    match region_name {
        None => return Err(CliError::BadArgument("--object is not specified".to_string(), USAGE.to_string())),
        _ => {}
    }

    Ok(S3Fetcher::new(&bucket_name.unwrap(), &object_key.unwrap(), &region_name.unwrap()))
}
