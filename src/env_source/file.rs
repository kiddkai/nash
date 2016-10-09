use std::fs::File;
use std::io::Read;
use std::path::Path;
use parser::env_file::parse_env_content;
use env_source::{FetchResult, Fetchable, SourceError};

#[derive(Debug)]
pub struct FileFetcher {
    path: String,
}

impl FileFetcher {
    pub fn new(path: &str) -> FileFetcher {
        FileFetcher { path: path.to_owned() }
    }
}

impl Fetchable for FileFetcher {
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

#[cfg(test)]
mod test {
    use env_source::*;
    use super::*;
    use std::fs::File;
    use std::io::*;
    use tempdir::TempDir;

    #[test]
    fn parse_simple_pair() {
        let dir = TempDir::new("single_line").expect("failed to create dir");
        let path = dir.path().join("single_line.env");
        let mut f = File::create(&path).expect("can not create file");
        writeln!(f, "FOO=BAR").expect("Failed to write content to file");
        let fetcher = FileFetcher::new(path.as_path().to_str().unwrap());
        let env = fetcher.fetch().unwrap();
        assert_eq!(env.len(), 1)
    }
}
