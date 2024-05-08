use crate::server::server::Server;

mod http;
mod server;

fn main() {
    let ip: String = String::from("127.0.0.1:8080");
    let server: Server = Server::new(ip);
    server.run();
}
