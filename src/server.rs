use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bas_request(&mut self, err: &ParseError) -> Response {
        println!("Failed to parse request: {}", err);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening to {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 2048];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(err) => handler.handle_bas_request(&err),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        },
                        Err(err) => {
                            println!("Failed to read from connection: {}", err);

                        }
                    }
                },
                Err(err) => {
                    println!("Fail to stablish connection: {}", err);
                    continue;
                }
            }
        }
    }
}
