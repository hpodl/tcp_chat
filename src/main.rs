use std::{
    io::{self, prelude::*},
    net::{TcpListener, ToSocketAddrs},
};

mod Instance;
mod ThreadPool;

fn main() {
    if let Ok(server) = Instance::Instance::new("127.0.0.1:4545", 4) {
        server.run();
    } else {
        eprintln!("Failed to initialize an Instance.");
    }
}
