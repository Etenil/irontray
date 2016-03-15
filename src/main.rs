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
use config::HttpConfig;

fn serve_client(mut client: TcpStream, config: HttpConfig) {
    println!("Request from {}\n", client.peer_addr().unwrap());
    
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
    
    
    let mut file_path = config.get_root_path();
    file_path.push(req.path.clone().trim_matches('/'));
    
    println!("Attempting to serve {}", file_path.display());
    
    let file_attempt = File::open(file_path);
    let mut response: HttpResponse = HttpResponse::quick_not_found("File not found!".to_string());
    match file_attempt {
        Ok(mut file) => {
            let mut content = "".to_string();
            let open_file_attempt = file.read_to_string(&mut content);
            match open_file_attempt {
                Ok(file_length) => {
                    response = HttpResponse::success_with_content(content);
                },
                Err(e) => {
                    response = HttpResponse::quick_not_found("File not found!".to_string());
                }
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
    client.write(response.to_string().as_bytes());
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
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
    let mut port = "8000".to_string();
    
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    if matches.opt_present("i") {
        ip_address = matches.opt_str("i").unwrap();
    }
    if matches.opt_present("p") {
        port = matches.opt_str("p").unwrap();
    }
    
    let address_proto = format!("{}:{}", ip_address, port);
    let proto: &str = &address_proto;
    let listener = TcpListener::bind(proto).unwrap();

    let config: HttpConfig;
    if matches.opt_present("c") {
        config = HttpConfig::new_from_file(matches.opt_str("c").unwrap()).unwrap();
    } else {
        config = HttpConfig::new_defaults().unwrap();
    }
    
    println!("Irontray server started on {}", proto);
    
    for stream in listener.incoming() {
        thread::spawn(|| {
            serve_client(stream.unwrap(), config);
        });
    }
}

