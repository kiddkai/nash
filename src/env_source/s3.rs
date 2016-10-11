use std::fs::File;
use std::io::Read;
use std::path::Path;
use parser::env_file::parse_env_content;
use env_source::{FetchResult, Fetchable, SourceError};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::s3::{S3Helper, S3Client, ListObjectsRequest, HeadObjectRequest};

#[derive(Debug)]
pub struct S3Fetcher {
    url: String,
}

impl S3Fetcher {
    pub fn new(url: &str) -> FileFetcher {
        S3Fetcher { url: url.to_owned() }
    }
}

impl Fetchable for S3Fetcher {
    fn fetch(&self) -> FetchResult {
        let path = Path::new(&*self.path);
        let mut file = try!(File::open(path));
        let mut content = String::new();
        try!(file.read_to_string(&mut content));
        match parse_env_content(&content) {
            Ok(v) => Ok(v),
            Err(e) => Err(SourceError::Parse(e)),
        }
    }
}
