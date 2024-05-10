pub mod server {
    use crate::http::{ParseError, Request, Response, StatusCode};
    use std::convert::TryFrom;
    use std::{io::Read, io::Write, net::TcpListener};

    pub trait RequestHandler {
        fn handle_request(&mut self, request: &Request) -> Response;

        fn handle_bad_request(&mut self, e: &ParseError) -> Response {
            println!("Failed to parse the request: {}", e);
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

        pub fn run(self, mut handler: impl RequestHandler) {
            println!("Listening on {}", self.addr);

            let listener: TcpListener = TcpListener::bind(&self.addr).unwrap();

            loop {
                match listener.accept() {
                    Ok((mut str, _)) => {
                        let mut buf = [0; 1024];
                        match str.read(&mut buf) {
                            Ok(_) => {
                                println!("Received a request: {}", String::from_utf8_lossy(&buf));

                                let response = match Request::try_from(&buf[..]) {
                                    Ok(request) => handler.handle_request(&request),
                                    Err(e) => handler.handle_bad_request(&e),
                                };
                                if let Err(e) = response.send(&mut str) {
                                    println!("Dailed to send the response: {}", e);
                                }
                            }
                            Err(e) => println!("Failed to read from connection: {}", e),
                        }
                    }
                    Err(e) => {
                        println!("Failed to establish the connection: {}", e);
                        continue;
                    }
                }
            }
        }
    }
}
