use std::str::FromStr;
use parser::env_file::parse_env_content;
use env_source::{FetchResult, Fetchable, SourceError};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::s3::S3Helper;

#[derive(Debug)]
pub struct S3Fetcher {
    bucket: String,
    object: String,
    region: String
}

impl S3Fetcher {
    pub fn new(bucket: &str, object: &str, region: &str) -> S3Fetcher {
        S3Fetcher { bucket: bucket.to_owned(), region: region.to_owned(), object: object.to_owned() }
    }
}

impl Fetchable for S3Fetcher {
    fn fetch(&self) -> FetchResult {
        let s3 = S3Helper::new(DefaultCredentialsProvider::new().unwrap(), Region::from_str(&self.region).unwrap());
        let obj = try!(s3.get_object(&self.bucket, &self.object));
        let content = String::from_utf8(obj.body).unwrap();
        match parse_env_content(&content) {
            Ok(v) => Ok(v),
            Err(e) => Err(SourceError::Parse(e)),
        }
    }
}
