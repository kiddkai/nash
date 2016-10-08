use env_source::{FetchResult, Fetchable};
use std::path::Path;
use std::fs::File;
use std::io::Read;

pub struct FileFetcher {
    path: String
}

pub fn parse_env_content(content: String) -> FetchResult {
    return Ok(vec![("FOO".to_string(), "BAR".to_string())])
}

impl Fetchable for FileFetcher {
    fn fetch(&self) -> FetchResult {
        let path = Path::new(&*self.path);
        let mut file = try!(File::open(path));
        let mut content = String::new();
        try!(file.read_to_string(&mut content));
        parse_env_content(content)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse_simple_pair() {
        assert!(0 == 0)
    }
}
