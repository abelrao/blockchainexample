use std::net::TcpListener;
use std::io::{Read, Write};

mod thread;
mod http;

use thread::ThreadPool;
use log::{info, LevelFilter};

pub struct Engine {}

impl Engine {
    pub fn run(adder: &str) {
        env_logger::builder().filter_level(LevelFilter::Info).init();
        let tcp_listener = TcpListener::bind(adder).expect("run server error");
        let pool = ThreadPool::new(4);
        for stream in tcp_listener.incoming() {
            info!("rec: request {:?}", stream);
            pool.execute(|| {
                let mut stream = stream.unwrap();
                let mut buff = [0; 1024];
                stream.read(&mut buff).unwrap();
                println!("{}", String::from_utf8_lossy(&buff));
                let response = r"HTTP/1.1 200 OK\r\n\r\n";
                stream.write(response.as_bytes());
                stream.flush().unwrap();
            })
        }
    }
}




