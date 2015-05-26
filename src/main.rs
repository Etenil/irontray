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
use std::path::Path;
use std::fs::File;

mod http;
use http::{HttpRequest, FromString};

fn serve_client(mut client: TcpStream) {
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
    
    let req_file = Path::new(req.path);
    
    if !req_file.exists() {
        println!("Requested file doesn't exissst!");
        // TODO: Implement 404.
    }
    else {
        let mut file = try!(File::open(req.path));
        let file_size = req_file.stat().size;
        
        
        client.write(b"Hello world\n").unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut ip_address;
    let mut port;
    let mut address_proto;
    
    if args.len() >= 3 {
        ip_address = args[1].clone();
        port = args[2].clone();
        address_proto = format!("{}:{}", ip_address, port);
    } else {
        address_proto = format!("0.0.0.0:8000");
    }
    
    let proto: &str = &address_proto;
    
    let listener = TcpListener::bind(proto).unwrap();
    
    println!("Irontray server started on {}", proto);
    
    for stream in listener.incoming() {
        thread::spawn(|| {
            serve_client(stream.unwrap());
        });
    }
}
