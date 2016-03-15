use std::fs::File;
use std::path::PathBuf;
use std::path::Path;
use std::env;

extern crate toml;

pub struct HttpConfig {
    root_path: PathBuf,
}

impl HttpConfig {
    pub fn new_from_file(filename: String) -> Option<HttpConfig> {
/*        let mut file = try!(File::open(filename));
        let mut conf_string = String::new();
        try!(file.read_to_string(&mut conf_string));

        let conf = toml::Parser::new(conf_string).parse().unwrap();
        //conf::wakawaka(); */
        return Some(HttpConfig {
            root_path: env::current_dir().unwrap()
        });
    }

    pub fn new_defaults() -> Option<HttpConfig> {
        return Some(HttpConfig {
            root_path: env::current_dir().unwrap()
        });
    }

    pub fn get_root_path(&self) -> PathBuf {
        return self.root_path;
    }
}
