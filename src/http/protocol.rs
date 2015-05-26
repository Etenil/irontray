/**
 * Common HTTP protocol objects.
 */

#[derive(PartialEq)]
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
