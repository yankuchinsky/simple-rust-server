use std::net::TcpListener;
use std::convert::TryFrom;
use std::io::{Read, Write};
use crate::http::{Request, Response, StatusCode, ParseError};

pub struct Server {
    address: String,
}

pub trait Handler {
    fn handleRequest(&mut self, request: &Request) -> Response;

    fn handleBadRequest(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse a request {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

impl Server {
    pub fn new(address: String) -> Self {
        Self {
            address,
        }
    }

    pub fn run(self, mut handler: impl Handler) { 
      let listener = TcpListener::bind(&self.address).unwrap();
      println!("Server is listening port {}", self.address);

      loop {
        match listener.accept() {
            Ok((mut stream, _)) => {
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(_) => {
                        println!("Recieved a request {}", String::from_utf8_lossy(&buffer));
                        
                        let response = match Request::try_from(&buffer[..]) {
                            Ok(request) =>  handler.handleRequest(&request),
                            Err(e) => handler.handleBadRequest(&e),
                        };
                        
                        if let Err(e) = response.send(&mut stream) {
                            println!("Failed to send response {}", e);
                        }
                    },
                    Err(e) => {
                        let response = Response::new(StatusCode::BadRequest, None);
                        response.send(&mut stream);
                        println!("Failed to read from connection {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Error {}", e);
            }
        }
      }
    }
} 

