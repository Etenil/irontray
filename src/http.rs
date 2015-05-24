use std::str::FromStr;

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

enum HttpVersion {
    HTTP1dot0,
    HTTP1dot1,
    HTTP2
}

impl FromStr for HttpVersion {
    type Err = ();
    
    fn from_str(s: &str) -> Result<HttpVersion, ()> {
        match s {
            "HTTP/1.0" => Ok(HttpVersion::HTTP1dot0),
            "HTTP/1.1" => Ok(HttpVersion::HTTP1dot1),
            "HTTP/2.0" => Ok(HttpVersion::HTTP2),
            "HTTP/2"   => Ok(HttpVersion::HTTP2),
            _ => Err(())
        }
    }
}

impl ToString for HttpVersion {
    fn to_string(&self) -> String {
        match *self {
            HttpVersion::HTTP1dot0 => "HTTP/1.0".to_string(),
            HttpVersion::HTTP1dot1 => "HTTP/1.1".to_string(),
            HttpVersion::HTTP2     => "HTTP2".to_string(),
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
    path: String,
    http_version: HttpVersion,
    host: String,
    user_agent: String
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

pub trait FromString {
    type Err;
    
    fn from_string(s: String) -> Result<Self, Self::Err>;
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
        });
    }
}
