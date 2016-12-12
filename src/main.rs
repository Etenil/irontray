// Copyright (c) 2015 Guillaume Pasquet
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use std::io::Write;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::Arc;
use std::env;
use std::str;
use std::path::PathBuf;
use std::fs::File;
extern crate getopts;
use getopts::Options;

mod http;
use http::request::HttpRequest;
use http::response::HttpResponse;
use http::traits::FromString;

mod config;
use config::httpconfig::HttpConfig;

#[macro_use]
extern crate log;
use log::LogLevelFilter;
extern crate syslog;
use syslog::Facility;

fn serve_client(mut client: TcpStream, config: Arc<HttpConfig>) {
    let mut buf = [0u8; 512];
    let mut buffer = String::new();

    loop {
        let len = client.read(&mut buf[0..512])
            .ok()
            .expect("Couldn't read from TCP client");

        if len > 0 {
            buffer = buffer + str::from_utf8(&buf).unwrap();

            if buffer.contains("\r\n\r\n") {
                // End of headers.
                break;
            }
        }
    }

    let req = HttpRequest::from_string(buffer)
        .ok()
        .expect("Couldn't read request.");

    let root_path: &str = *config.get_root_path();
    let mut file_path: PathBuf = PathBuf::new();
    file_path.push(root_path);
    file_path.push(req.path.clone().trim_matches('/'));

    if file_path.is_dir() {
        file_path.push(*config.get_index());
    }

    let response: HttpResponse;
    match File::open(file_path) {
        Ok(mut file) => {
            let mut content = "".to_string();
            match file.read_to_string(&mut content) {
                Ok(file_length) => {
                    info!(
                        "{} {} 200 {}",
                        client.peer_addr().unwrap(),
                        req.to_string(),
                        file_length
                    );
                    response = HttpResponse::success_with_content(content);
                },
                Err(e) => {
                    info!(
                        "{} {} 404 0",
                        client.peer_addr().unwrap(),
                        req.to_string()
                    );
                    error!("Couldn't read file: {}", e.to_string());
                    response = HttpResponse::quick_not_found("File not found!".to_string());
                }
            }
        },
        Err(e) => {
            info!(
                "{} {} 500 0",
                client.peer_addr().unwrap(),
                req.to_string()
            );
            error!("Couldn't open file: {:?}", e);
            response = HttpResponse::quick_server_error("File not found!".to_string());
        }
    }
    client.write(response.to_string().as_bytes());
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    match syslog::init(Facility::LOG_USER, log::LogLevelFilter::Info, None) {
        Err(e) => println!("Couldn't connect to syslog! {:?}", e),
        Ok(()) => {}
    };

    info!("Starting Irontray server");

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optopt("i", "ip-address", "set listening IP address", "0.0.0.0");
    opts.optopt("p", "port", "set TCP port to listen on", "8000");
    opts.optopt("c", "conf", "path to the config file", "8000");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(v)  => { v },
        Err(e) => { panic!(e.to_string()) }
    };

    let mut ip_address = "0.0.0.0".to_string();

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    if matches.opt_present("i") {
        ip_address = matches.opt_str("i").unwrap();
    }


    let config: Arc<HttpConfig>;
    if matches.opt_present("c") {
        let filename = matches.opt_str("c").unwrap();
        match HttpConfig::new_from_file(filename) {
            Ok(httpconf) => {
                config = Arc::new(httpconf);
            },
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        }
    } else {
        config = Arc::new(HttpConfig::new_defaults().unwrap());
    }

    let address_proto: String;
    if matches.opt_present("p") {
        let port = String::from(matches.opt_str("p").unwrap());
        address_proto = format!("{}:{}", ip_address, port);
    } else {
        address_proto = format!("{}:{}", ip_address, *config.get_port());
    }
    println!("{:?}", address_proto);
    let proto: &str = &address_proto;
    let listener = TcpListener::bind(proto).unwrap();

    info!("Listening on {}", proto);

    for stream in listener.incoming() {
        let conf = config.clone();
        thread::spawn(|| {
            serve_client(stream.unwrap(), conf);
        });
    }
}
