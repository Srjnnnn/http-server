pub mod server {
    use crate::http::Request;
    use std::convert::TryFrom;
    use std::{io::Read, net::TcpListener};

    pub struct Server {
        addr: String,
    }

    impl Server {
        pub fn new(addr: String) -> Self {
            Self { addr }
        }

        pub fn run(self) {
            println!("Listening on {}", self.addr);

            let listener: TcpListener = TcpListener::bind(&self.addr).unwrap();

            loop {
                match listener.accept() {
                    Ok((mut str, _)) => {
                        let mut buf = [0; 1024];
                        match str.read(&mut buf) {
                            Ok(_) => {
                                println!("Received a request: {}", String::from_utf8_lossy(&buf));

                                match Request::try_from(&buf as &[u8]) {
                                    Ok(_request) => {}
                                    Err(e) => println!("Failed to parse the error: {}", e),
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
