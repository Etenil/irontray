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

/**
 * Common HTTP stuff.
 */

#[derive(PartialEq)]
pub enum HttpVersion {
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
