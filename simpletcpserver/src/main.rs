use std::net::TcpListener;
use std::io::{Read, Write};
use std::ops::Add;

fn main() {
    let tcp_listener = TcpListener::bind("0.0.0.0:8888").unwrap();
    for tcp_stream in tcp_listener.incoming() {
        let mut stream = tcp_stream.unwrap();


        let mut buffer = [0; 2048];
        stream.read(&mut buffer).unwrap();
        let mut content = String::from_utf8_lossy(&buffer);

        println!("server receiver: {:?}", content);

        let response = format!("HTTP/1.1 200 OK\r\n content-type:text/html;charset=utf-8\r\n   \r\n\r\n {} \n", "hai, rust");
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
