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

// This is an implementation of a standard HTTP response. To wrap data into.


use std::str::FromStr;

use http::protocol::HttpVersion;
use http::traits::FromU16;

enum HttpStatus {
    // 100s
    CONTINUE,
    SWITCH_PROTO,
    
    // 200s
    OK,
    CREATED,
    ACCEPTED,
    NON_AUTHORITATIVE_INFO,
    NO_CONTENT,
    RESET_CONTENT,
    PARTIAL_CONTENT,
    
    // 300s
    MULTIPLE_CHOICES,
    MOVED_PERMANENTLY,
    FOUND,
    SEE_OTHER,
    NOT_MODIFIED,
    USE_PROXY,
    SWITCH_PROXY,
    TEMP_REDIRECT,
    PERM_REDIRECT,
    
    // 400s
    BAD_REQUEST,
    UNAUTHORIZED,
    PAYMENT_REQUIRED,
    FORBIDDEN,
    NOT_FOUND,
    METHOD_NOT_ALLOWED,
    NOT_ACCEPTABLE,
    PROXY_AUTH_REQUIRED,
    REQUEST_TIMEOUT,
    CONFLICT,
    GONE,
    LENGTH_REQUIRED,
    PRECONDITION_FAILED,
    REQUEST_ENTITY_TOO_LARGE,
    REQUEST_URI_TOO_LONG,
    UNSUPPORTED_MEDIA_TYPE,
    REQUEST_RANGE_NOT_SATISFIABLE,
    EXPECTATION_FAILED,
    IM_A_TEAPOT,
    AUTH_TIMEOUT,
    MISDIRECTED_REQUEST,
    UPGRADE_REQUIRED,
    PRECONDITION_REQUIRED,
    TOO_MANY_REQUESTS,
    REQUEST_HEADER_FIELDS_TOO_LARGE,
    
    // Pfew... 500s now
    INTERNAL_SERVER_ERROR,
    NOT_IMPLEMENTED,
    BAD_GATEWAY,
    SERVICE_UNAVAILABLE,
    GATEWAY_TIMEOUT,
    HTTP_VERSION_NOT_SUPPORTED,
    VARIANT_ALSO_NEGOCIATES,
    NOT_EXTENDED,
    NETWORK_AUTH_REQUIRED,
}

impl FromStr for HttpStatus {
    type Err = ();
    
    fn from_str(s: &str) -> Result<HttpStatus, ()> {
        match s {
            "100 Continue"                        => Ok(HttpStatus::CONTINUE),
            "101 Switching Protocols"             => Ok(HttpStatus::SWITCH_PROTO),
            
            "200 OK"                              => Ok(HttpStatus::OK),
            "201 Created"                         => Ok(HttpStatus::CREATED),
            "202 Accepted"                        => Ok(HttpStatus::ACCEPTED),
            "203 Non-Authoritative Information"   => Ok(HttpStatus::NON_AUTHORITATIVE_INFO),
            "204 No Content"                      => Ok(HttpStatus::NO_CONTENT),
            "205 Reset Content"                   => Ok(HttpStatus::RESET_CONTENT),
            "206 Reset Content"                   => Ok(HttpStatus::PARTIAL_CONTENT),
            
            // 300s
            "300 Multiple Choices"                => Ok(HttpStatus::MULTIPLE_CHOICES),
            "301 Moved Permanently"               => Ok(HttpStatus::MOVED_PERMANENTLY),
            "302 Found"                           => Ok(HttpStatus::FOUND),
            "303 See Other"                       => Ok(HttpStatus::SEE_OTHER),
            "304 Not Modified"                    => Ok(HttpStatus::NOT_MODIFIED),
            "305 Use Proxy"                       => Ok(HttpStatus::USE_PROXY),
            "306 Switch Proxy"                    => Ok(HttpStatus::SWITCH_PROXY),
            "307 Temporary Redirect"              => Ok(HttpStatus::TEMP_REDIRECT),
            "308 Permanent Redirect"              => Ok(HttpStatus::PERM_REDIRECT),
            
            // 400s
            "400 Bad Request"                     => Ok(HttpStatus::BAD_REQUEST),
            "401 Unauthorized"                    => Ok(HttpStatus::UNAUTHORIZED),
            "402 Payment Required"                => Ok(HttpStatus::PAYMENT_REQUIRED),
            "403 Forbidden"                       => Ok(HttpStatus::FORBIDDEN),
            "404 Not Found"                       => Ok(HttpStatus::NOT_FOUND),
            "405 Method Not Allowed"              => Ok(HttpStatus::METHOD_NOT_ALLOWED),
            "406 Not Acceptable"                  => Ok(HttpStatus::NOT_ACCEPTABLE),
            "407 Proxy Authentication Required"   => Ok(HttpStatus::PROXY_AUTH_REQUIRED),
            "408 Request Timeout"                 => Ok(HttpStatus::REQUEST_TIMEOUT),
            "409 Conflict"                        => Ok(HttpStatus::CONFLICT),
            "410 Gone"                            => Ok(HttpStatus::GONE),
            "411 Length Required"                 => Ok(HttpStatus::LENGTH_REQUIRED),
            "412 Precondition Failed"             => Ok(HttpStatus::PRECONDITION_FAILED),
            "413 Request Entity Too Large"        => Ok(HttpStatus::REQUEST_ENTITY_TOO_LARGE),
            "414 Request-URI Too Long"            => Ok(HttpStatus::REQUEST_URI_TOO_LONG),
            "415 Unsupported Media Type"          => Ok(HttpStatus::UNSUPPORTED_MEDIA_TYPE),
            "416 Requested Range Not Satisfiable" => Ok(HttpStatus::REQUEST_RANGE_NOT_SATISFIABLE),
            "417 Expectation Failed"              => Ok(HttpStatus::EXPECTATION_FAILED),
            "418 I'm a teapot"                    => Ok(HttpStatus::IM_A_TEAPOT),
            "419 Authentication Timeout"          => Ok(HttpStatus::AUTH_TIMEOUT),
            "421 Misdirected Request"             => Ok(HttpStatus::MISDIRECTED_REQUEST),
            "426 Upgrade Required"                => Ok(HttpStatus::UPGRADE_REQUIRED),
            "428 Precondition Required"           => Ok(HttpStatus::PRECONDITION_REQUIRED),
            "429 Too Many Requests"               => Ok(HttpStatus::TOO_MANY_REQUESTS),
            "431 Request Header Fields Too Large" => Ok(HttpStatus::REQUEST_HEADER_FIELDS_TOO_LARGE),
            
            // Pfew... 500s now
            "500 Internal Server Error"           => Ok(HttpStatus::INTERNAL_SERVER_ERROR),
            "501 Not Implemented"                 => Ok(HttpStatus::NOT_IMPLEMENTED),
            "502 Bad Gateway"                     => Ok(HttpStatus::BAD_GATEWAY),
            "503 Service Unavailable"             => Ok(HttpStatus::SERVICE_UNAVAILABLE),
            "504 Gateway Timeout"                 => Ok(HttpStatus::GATEWAY_TIMEOUT),
            "505 HTTP Version Not Supported"      => Ok(HttpStatus::HTTP_VERSION_NOT_SUPPORTED),
            "506 Variant Also Negociates"         => Ok(HttpStatus::VARIANT_ALSO_NEGOCIATES),
            "510 Not Extended"                    => Ok(HttpStatus::NOT_EXTENDED),
            "511 Network Authentication Required" => Ok(HttpStatus::NETWORK_AUTH_REQUIRED),
            
            _ => Err(()),
        }
    }
}

impl FromU16 for HttpStatus {
    type Err = ();
    
    fn from_u16(num: u16) -> Result<HttpStatus, ()> {
        match num {
            100u16 => Ok(HttpStatus::CONTINUE),
            101u16 => Ok(HttpStatus::SWITCH_PROTO),
            
            200u16 => Ok(HttpStatus::OK),
            201u16 => Ok(HttpStatus::CREATED),
            202u16 => Ok(HttpStatus::ACCEPTED),
            203u16 => Ok(HttpStatus::NON_AUTHORITATIVE_INFO),
            204u16 => Ok(HttpStatus::NO_CONTENT),
            205u16 => Ok(HttpStatus::RESET_CONTENT),
            206u16 => Ok(HttpStatus::PARTIAL_CONTENT),
            
            // 300s
            300u16 => Ok(HttpStatus::MULTIPLE_CHOICES),
            301u16 => Ok(HttpStatus::MOVED_PERMANENTLY),
            302u16 => Ok(HttpStatus::FOUND),
            303u16 => Ok(HttpStatus::SEE_OTHER),
            304u16 => Ok(HttpStatus::NOT_MODIFIED),
            305u16 => Ok(HttpStatus::USE_PROXY),
            306u16 => Ok(HttpStatus::SWITCH_PROXY),
            307u16 => Ok(HttpStatus::TEMP_REDIRECT),
            308u16 => Ok(HttpStatus::PERM_REDIRECT),
            
            // 400s
            400u16 => Ok(HttpStatus::BAD_REQUEST),
            401u16 => Ok(HttpStatus::UNAUTHORIZED),
            402u16 => Ok(HttpStatus::PAYMENT_REQUIRED),
            403u16 => Ok(HttpStatus::FORBIDDEN),
            404u16 => Ok(HttpStatus::NOT_FOUND),
            405u16 => Ok(HttpStatus::METHOD_NOT_ALLOWED),
            406u16 => Ok(HttpStatus::NOT_ACCEPTABLE),
            407u16 => Ok(HttpStatus::PROXY_AUTH_REQUIRED),
            408u16 => Ok(HttpStatus::REQUEST_TIMEOUT),
            409u16 => Ok(HttpStatus::CONFLICT),
            410u16 => Ok(HttpStatus::GONE),
            411u16 => Ok(HttpStatus::LENGTH_REQUIRED),
            412u16 => Ok(HttpStatus::PRECONDITION_FAILED),
            413u16 => Ok(HttpStatus::REQUEST_ENTITY_TOO_LARGE),
            414u16 => Ok(HttpStatus::REQUEST_URI_TOO_LONG),
            415u16 => Ok(HttpStatus::UNSUPPORTED_MEDIA_TYPE),
            416u16 => Ok(HttpStatus::REQUEST_RANGE_NOT_SATISFIABLE),
            417u16 => Ok(HttpStatus::EXPECTATION_FAILED),
            418u16 => Ok(HttpStatus::IM_A_TEAPOT),
            419u16 => Ok(HttpStatus::AUTH_TIMEOUT),
            421u16 => Ok(HttpStatus::MISDIRECTED_REQUEST),
            426u16 => Ok(HttpStatus::UPGRADE_REQUIRED),
            428u16 => Ok(HttpStatus::PRECONDITION_REQUIRED),
            429u16 => Ok(HttpStatus::TOO_MANY_REQUESTS),
            431u16 => Ok(HttpStatus::REQUEST_HEADER_FIELDS_TOO_LARGE),
            
            // Pfew... 500s now
            500u16 => Ok(HttpStatus::INTERNAL_SERVER_ERROR),
            501u16 => Ok(HttpStatus::NOT_IMPLEMENTED),
            502u16 => Ok(HttpStatus::BAD_GATEWAY),
            503u16 => Ok(HttpStatus::SERVICE_UNAVAILABLE),
            504u16 => Ok(HttpStatus::GATEWAY_TIMEOUT),
            505u16 => Ok(HttpStatus::HTTP_VERSION_NOT_SUPPORTED),
            506u16 => Ok(HttpStatus::VARIANT_ALSO_NEGOCIATES),
            510u16 => Ok(HttpStatus::NOT_EXTENDED),
            511u16 => Ok(HttpStatus::NETWORK_AUTH_REQUIRED),
            
            _ => Err(()),
        }
    }
}

impl ToString for HttpStatus {
    fn to_string(&self) -> String {
        match *self {
            HttpStatus::CONTINUE                        => "100 Continue".to_string(),
            HttpStatus::SWITCH_PROTO                    => "101 Switching Protocols".to_string(),
            
            HttpStatus::OK                              => "200 OK".to_string(),
            HttpStatus::CREATED                         => "201 Created".to_string(),
            HttpStatus::ACCEPTED                        => "202 Accepted".to_string(),
            HttpStatus::NON_AUTHORITATIVE_INFO          => "203 Non-Authoritative Information".to_string(),
            HttpStatus::NO_CONTENT                      => "204 No Content".to_string(),
            HttpStatus::RESET_CONTENT                   => "205 Reset Content".to_string(),
            HttpStatus::PARTIAL_CONTENT                 => "206 Reset Content".to_string(),
            
            // 300s
            HttpStatus::MULTIPLE_CHOICES                => "300 Multiple Choices".to_string(),
            HttpStatus::MOVED_PERMANENTLY               => "301 Moved Permanently".to_string(),
            HttpStatus::FOUND                           => "302 Found".to_string(),
            HttpStatus::SEE_OTHER                       => "303 See Other".to_string(),
            HttpStatus::NOT_MODIFIED                    => "304 Not Modified".to_string(),
            HttpStatus::USE_PROXY                       => "305 Use Proxy".to_string(),
            HttpStatus::SWITCH_PROXY                    => "306 Switch Proxy".to_string(),
            HttpStatus::TEMP_REDIRECT                   => "307 Temporary Redirect".to_string(),
            HttpStatus::PERM_REDIRECT                   => "308 Permanent Redirect".to_string(),
            
            // 400s
            HttpStatus::BAD_REQUEST                     => "400 Bad Request".to_string(),
            HttpStatus::UNAUTHORIZED                    => "401 Unauthorized".to_string(),
            HttpStatus::PAYMENT_REQUIRED                => "402 Payment Required".to_string(),
            HttpStatus::FORBIDDEN                       => "403 Forbidden".to_string(),
            HttpStatus::NOT_FOUND                       => "404 Not Found".to_string(),
            HttpStatus::METHOD_NOT_ALLOWED              => "405 Method Not Allowed".to_string(),
            HttpStatus::NOT_ACCEPTABLE                  => "406 Not Acceptable".to_string(),
            HttpStatus::PROXY_AUTH_REQUIRED             => "407 Proxy Authentication Required".to_string(),
            HttpStatus::REQUEST_TIMEOUT                 => "408 Request Timeout".to_string(),
            HttpStatus::CONFLICT                        => "409 Conflict".to_string(),
            HttpStatus::GONE                            => "410 Gone".to_string(),
            HttpStatus::LENGTH_REQUIRED                 => "411 Length Required".to_string(),
            HttpStatus::PRECONDITION_FAILED             => "412 Precondition Failed".to_string(),
            HttpStatus::REQUEST_ENTITY_TOO_LARGE        => "413 Request Entity Too Large".to_string(),
            HttpStatus::REQUEST_URI_TOO_LONG            => "414 Request-URI Too Long".to_string(),
            HttpStatus::UNSUPPORTED_MEDIA_TYPE          => "415 Unsupported Media Type".to_string(),
            HttpStatus::REQUEST_RANGE_NOT_SATISFIABLE   => "416 Requested Range Not Satisfiable".to_string(),
            HttpStatus::EXPECTATION_FAILED              => "417 Expectation Failed".to_string(),
            HttpStatus::IM_A_TEAPOT                     => "418 I'm a teapot".to_string(),
            HttpStatus::AUTH_TIMEOUT                    => "419 Authentication Timeout".to_string(),
            HttpStatus::MISDIRECTED_REQUEST             => "421 Misdirected Request".to_string(),
            HttpStatus::UPGRADE_REQUIRED                => "426 Upgrade Required".to_string(),
            HttpStatus::PRECONDITION_REQUIRED           => "428 Precondition Required".to_string(),
            HttpStatus::TOO_MANY_REQUESTS               => "429 Too Many Requests".to_string(),
            HttpStatus::REQUEST_HEADER_FIELDS_TOO_LARGE => "431 Request Header Fields Too Large".to_string(),
            
            // Pfew... 500s now
            HttpStatus::INTERNAL_SERVER_ERROR           => "500 Internal Server Error".to_string(),
            HttpStatus::NOT_IMPLEMENTED                 => "501 Not Implemented".to_string(),
            HttpStatus::BAD_GATEWAY                     => "502 Bad Gateway".to_string(),
            HttpStatus::SERVICE_UNAVAILABLE             => "503 Service Unavailable".to_string(),
            HttpStatus::GATEWAY_TIMEOUT                 => "504 Gateway Timeout".to_string(),
            HttpStatus::HTTP_VERSION_NOT_SUPPORTED      => "505 HTTP Version Not Supported".to_string(),
            HttpStatus::VARIANT_ALSO_NEGOCIATES         => "506 Variant Also Negociates".to_string(),
            HttpStatus::NOT_EXTENDED                    => "510 Not Extended".to_string(),
            HttpStatus::NETWORK_AUTH_REQUIRED           => "511 Network Authentication Required".to_string(),
        }
    }
}

struct HttpResponse {
    http_version: HttpVersion,
    status: HttpStatus,
    content_type: String,
    length: u64,
    content: String,
}

impl ToString for HttpResponse {
    fn to_string(&self) -> String {
        let mut buf: String = "".to_string();
        
        buf = format!("{} {}", self.http_version.to_string(), self.status.to_string());
        buf = format!("{}\r\nContent-Type: {}", buf, self.content_type);
        buf = format!("{}\r\nContent-Length: {}", buf, self.length);
        
        // End of header and content.
        buf = format!("{}\r\n\r\n{}", buf, self.content);
        
        return buf;
    }
}
