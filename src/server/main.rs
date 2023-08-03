mod server;
use server::*;


fn main() {
    let mut server = Instance::new("127.0.0.1:5353").unwrap();
    server.run();
}