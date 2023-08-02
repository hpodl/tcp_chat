mod chat;
mod request;
mod server;
mod client;

fn main() {
    if let Ok(mut server) = server::Instance::new("127.0.0.1:4545") {
        server.run();
    } else {
        eprintln!("Failed to initialize an Instance.");
    }
}
