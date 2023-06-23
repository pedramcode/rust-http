use std::io::{Read};
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::http::parser::HTTPMessage;

pub struct Server {
    port: u32,
    listener: TcpListener,
}

impl Server {
    pub fn new(port: u32) -> Result<Server, std::io::Error> {
        let url = String::from(format!("0.0.0.0:{}", port));
        let listener = TcpListener::bind(url)?;
        Ok(Server {
            listener,
            port,
        })
    }

    pub fn listen(&self) -> Result<(), std::io::Error> {
        println!("Server is listening to http://127.0.0.0:{}", self.port);
        for stream in self.listener.incoming() {
            thread::spawn(move || {
                handle_request(stream.unwrap());
            });
        }
        Ok(())
    }
}

fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(size) => {
            let request = String::from_utf8_lossy(&buffer[..size]);
            let msg = HTTPMessage::from_string(&*request).unwrap();
            println!("{:?}", msg);
        }
        Err(e) => {
            eprintln!("Error reading request: {}", e);
        }
    }
}