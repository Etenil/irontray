use std::fs::File;
use std::path::PathBuf;
use std::io::Read;
use std::env;

extern crate toml;

pub struct HttpConfig {
    root_path: PathBuf,
}

impl HttpConfig {
    pub fn new_from_file(filename: String) -> Result<HttpConfig, String> {
        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(e) => {
                return Err(format!("Failed to open conf file {:?}", e));
            }
        };

        let mut content: String = String::new();
        match file.read_to_string(&mut content) {
            Ok(file_length) => file_length,
            Err(e) => {
                return Err(format!("Failed to read conf file {:?}", e));
            }
        };

        let conf = toml::Parser::new(content.as_str()).parse().unwrap();
        let http_sec = match conf.get("http") {
            Some(http_sec) => http_sec,
            None => {
                return Err(format!("Couldn't find section 'http' in configuration file."));
            }
        };

        let root_path = match http_sec.as_table().unwrap().get("root_path") {
            Some(root_path) => root_path,
            None => {
                return Err(format!("Failed to read config."));
            }
        };

        let mut path = PathBuf::new();
        path.push(root_path.as_str().unwrap());
        return Ok(HttpConfig {
            root_path: path
        });
    }

    pub fn new_defaults() -> Option<HttpConfig> {
        return Some(HttpConfig {
            root_path: env::current_dir().unwrap()
        });
    }

    pub fn get_root_path(&self) -> Box<&str> {
        return Box::new(self.root_path.to_str().unwrap());
    }
}
