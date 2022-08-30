mod chat;
mod server;
mod thread_pool;

fn main() {
    if let Ok(server) = server::Instance::new("127.0.0.1:4545", 4) {
        server.run();
    } else {
        eprintln!("Failed to initialize an Instance.");
    }
}
