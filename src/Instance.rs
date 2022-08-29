use std::{
    io::{self, prelude::*},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

use super::ThreadPool::*;

pub struct Instance {
    listener: TcpListener,
    thread_pool: ThreadPool,
}

impl Instance {
    pub fn new<A>(address: A, thread_count: usize) -> io::Result<Self>
    where
        A: ToSocketAddrs,
    {
        Ok(Self {
            listener: TcpListener::bind(address)?,
            thread_pool: ThreadPool::new(thread_count),
        })
    }

    pub fn run(&self) {
        for stream in self.listener.incoming() {
            if let Ok(connection) = stream {
                self.thread_pool.execute(|| handle_connection(connection))
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            println!("{}", String::from_utf8_lossy(&buffer));
        }
        Err(e) => eprintln!("{}", e),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Helper function
    fn connect_and_send<A>(address: A, message: &str) -> io::Result<usize>
    where
        A: ToSocketAddrs,
    {
        let mut connection = TcpStream::connect(address).unwrap();
        let buf = b"Messsage";

        connection.write(buf)
    }

    #[test]
    fn instance_constructs_on_valid_address() {
        assert!(Instance::new("127.0.0.1:25001", 4).is_ok())
    }

    #[test]
    fn instance_errors_on_invalid_address() {
        assert!(Instance::new("266.788.123.1:7878", 5).is_err())
    }
}
