use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::env;

fn serve_client(mut toto: TcpStream) {
    println!("Request from {}\n", toto.peer_addr().unwrap());
    
    toto.write(b"Hello world\n").unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let ip_address = args[1].clone();
    let port = args[2].clone();
    let address_proto = format!("{}:{}", ip_address, port);
    let proto: &str = &address_proto;
    
    let listener = TcpListener::bind(proto).unwrap();
    
    for stream in listener.incoming() {
        thread::spawn(|| {
            serve_client(stream.unwrap());
        });
    }
}
