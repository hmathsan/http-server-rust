use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening to {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 2048];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                },
                                Err(err) => println!("Failed to parse a request: {}", err)
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
