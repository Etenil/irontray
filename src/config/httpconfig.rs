use std::fs::File;
use std::path::PathBuf;
use std::io::Read;
use std::env;

extern crate toml;

pub struct VhostConfig {
    hostname: String
}

pub struct HttpConfig {
    root_path: PathBuf,
    index: String,
    port: String,
    vhosts: Vec<VhostConfig>
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
                return Err(format!("Root path must be set in config file."));
            }
        };

        let index = match http_sec.as_table().unwrap().get("index") {
            Some(index) => index.as_str().unwrap(),
            None => "index.html"
        };

        let port = match http_sec.as_table().unwrap().get("port") {
            Some(port) => match(port.as_str()) {
                Some(portString) => portString,
                None => "8000"
            },
            None => "8000"
        };

        let vhosts_sec = match conf.get("vhosts") {
            Some(vhosts) => vhosts,
            None => {
                return Err(format!("Couldn't find a 'vhosts' section in config file"));
            }
        };

        let mut vhosts: Vec<VhostConfig>;
        for (vhost_name, vhost_config in vhosts_sec.iter()) {
            let vhostname = match vhost_config.get("hostname") {
                Some(hostname) => hostname,
                None => {
                    return Err(format!("Couldn't find a hostname in vhost definition"))
                }
            };
            vhosts.push(VhostConfig {
                hostname: vhostname
            });
        }

        let mut path = PathBuf::new();
        path.push(root_path.as_str().unwrap());
        return Ok(HttpConfig {
            root_path: path,
            index: String::from(index),
            port: String::from(port),
            vhosts: vhosts
        });
    }

    pub fn new_defaults() -> Option<HttpConfig> {
        return Some(HttpConfig {
            root_path: env::current_dir().unwrap(),
            index: String::from("index.html"),
            port: String::from("8000"),
            vhosts: null
        });
    }

    pub fn get_root_path(&self) -> Box<&str> {
        return Box::new(self.root_path.to_str().unwrap());
    }

    pub fn get_index(&self) -> Box<&String> {
        return Box::new(&self.index);
    }

    pub fn get_port(&self) -> Box<&String> {
        return Box::new(&self.port);
    }
}
