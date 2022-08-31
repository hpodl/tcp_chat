mod chat;
mod server;

fn main() {
    if let Ok(server) = server::Instance::new("127.0.0.1:4545") {
        server.run();
    } else {
        eprintln!("Failed to initialize an Instance.");
    }
}
