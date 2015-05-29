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

use std::str::FromStr;

use http::protocol::HttpVersion;
use http::traits::FromString;

#[derive(PartialEq)]
enum HttpMethod {
    OPTIONS,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    TRACE,
    CONNECT
}

impl FromStr for HttpMethod {
    type Err = ();
    
    fn from_str(s: &str) -> Result<HttpMethod, ()> {
        match s {
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            "GET"     => Ok(HttpMethod::GET),
            "HEAD"    => Ok(HttpMethod::HEAD),
            "POST"    => Ok(HttpMethod::POST),
            "PUT"     => Ok(HttpMethod::PUT),
            "DELETE"  => Ok(HttpMethod::DELETE),
            "TRACE"   => Ok(HttpMethod::TRACE),
            "CONNECT" => Ok(HttpMethod::CONNECT),
            _ => Err(()),
        }
    }
}

impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        match *self {
            HttpMethod::OPTIONS => "OPTIONS".to_string(),
            HttpMethod::GET     => "GET".to_string(),
            HttpMethod::HEAD    => "HEAD".to_string(),
            HttpMethod::POST    => "POST".to_string(),
            HttpMethod::PUT     => "PUT".to_string(),
            HttpMethod::DELETE  => "DELETE".to_string(),
            HttpMethod::TRACE   => "TRACE".to_string(),
            HttpMethod::CONNECT => "CONNECT".to_string(),
        }
    }
}

struct HttpHeader {
    name: String,
    value: String
}

impl FromStr for HttpHeader {
    type Err = ();
    
    fn from_str(s: &str) -> Result<HttpHeader, ()> {
        let header_def: Vec<&str> = s.split(": ").collect();
        if header_def.len() < 2 {
            return Err(());
        }
        
        return Ok(HttpHeader{
            name: header_def[0].to_string(),
            value: header_def[1].to_string(),
        });
    }
}

pub struct HttpRequest {
    method: HttpMethod,
    pub path: String,
    http_version: HttpVersion,
    host: String,
    user_agent: String,
    length: usize,
}

impl ToString for HttpRequest {
    fn to_string(&self) -> String {
        let mut buf: String = "".to_string();
        
        return format!(
            "{} {} {}\r\nHost: {}\r\nUser-Agent: {}",
            self.method.to_string(),
            self.path,
            self.http_version.to_string(),
            self.host,
            self.user_agent
        );
    }
}

impl FromString for HttpRequest {
    type Err = ();
    
    fn from_string(request_string: String) -> Result<HttpRequest, ()> {
        // Slicing and dicing.
        let mut header_lines: Vec<&str> = request_string.split("\r\n").collect();
        
        // Splitting the first header.
        let first_line: Vec<&str> = header_lines[0].split(" ").collect();
        
        if first_line.len() < 3 {
            return Err(());
        }
        
        let req_meth = HttpMethod::from_str(first_line[0])
            .ok()
            .expect("Unknown request method.");
        let req_path = first_line[1];
        let req_version = HttpVersion::from_str(first_line[2])
            .ok()
            .expect("Uknown HTTP version.");
        
        let mut req_host: String = "".to_string();
        let mut req_user_agent: String = "".to_string();
        let mut req_length = 0usize;
        
        // Pop the first line now.
        header_lines.remove(0);
        for line in &header_lines {
            let header = HttpHeader::from_str(line);
            match header {
                Ok(v) => {
                    let sname: &str = &v.name;
                    match sname {
                        "Host" => req_host = v.value,
                        "User-Agent" => req_user_agent = v.value,
                        "Content-Length" => req_length = usize::from_str(&v.value).unwrap(),
                        _ => {}
                    }
                },
                Err(e) => {
                    // Pass.
                }
            }
        }
        
        return Ok(HttpRequest {
            method:       req_meth,
            path:         req_path.to_string(),
            http_version: req_version,
            host:         req_host,
            user_agent:   req_user_agent,
            length:       req_length,
        });
    }
}

#[test]
fn from_string_works() {
    let request = "GET /blob HTTP/1.1\r\nHost: john.com\r\nUser-Agent: rust-test";
    let req = HttpRequest::from_string(request.to_string());
    match req {
        Ok(v) => {
            assert!(v.method == HttpMethod::GET);
            assert!(v.path == "/blob".to_string());
            assert!(v.http_version == HttpVersion::HTTP1dot1);
            assert!(v.host == "john.com".to_string());
            assert!(v.user_agent == "rust-test".to_string());
        },
        Err(e) => {
            panic!(e);
        }
    }
}
