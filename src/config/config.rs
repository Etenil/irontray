use std::fs::File;
use std::path::PathBuf;
use std::path::Path;
use std::io::Read;
use std::env;

extern crate toml;

pub struct HttpConfig {
    root_path: PathBuf,
}

impl HttpConfig {
    pub fn new_from_file(filename: String) -> Option<HttpConfig> {
        match File::open(filename) {
            Ok(mut file) => {
                let mut content: String = String::new();
                match file.read_to_string(&mut content) {
                    Ok(file_length) => {
                        let conf = toml::Parser::new(content.as_str()).parse().unwrap();
                        match conf.get("http") {
                            Some(v) => {
                                match v.as_table().unwrap().get("root_path") {
                                    Some(v) => {
                                        let mut path = PathBuf::new();
                                        path.push(v.as_str().unwrap());
                                        return Some(HttpConfig {
                                            root_path: path
                                        });
                                    },
                                    None => {
                                        println!("Failed to read config.");
                                    }
                                }
                            },
                            None => {
                                println!("Failed to read config.");
                            }
                        }
                    },
                    Err(e) => {
                        // TODO: Log error.
                    }
                }
            },
            Err(e) => {
                // TODO: Log error
            }
        }
        
        return Some(HttpConfig {
            root_path: env::current_dir().unwrap()
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
