use std::{
    io::{self, prelude::*},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

use super::thread_pool::*;

pub struct Instance {
    listener: TcpListener,
    thread_pool: ThreadPool,
}

impl Instance {
    /// Create a server instance on `address` socket
    /// and assign it `thread_count` threads (`Worker`s)
    ///
    /// # Errors
    /// Returns an error if binding a `TcpListener` to the given socket fails
    pub fn new<A>(address: A, thread_count: usize) -> io::Result<Self>
    where
        A: ToSocketAddrs,
    {
        Ok(Self {
            listener: TcpListener::bind(address)?,
            thread_pool: ThreadPool::new(thread_count),
        })
    }

    /// Loops over incoming connections and dispatches tasks of handling them
    ///
    /// # Panics and Errors
    /// All error handling is delegated to
    pub fn run(&self) {
        for stream in self.listener.incoming().flatten() {
            self.thread_pool.execute(|| handle_connection(stream))
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

    ///# Test helper function
    fn connect_and_send<A>(address: A, message: &str) -> io::Result<usize>
    where
        A: ToSocketAddrs,
    {
        let mut connection = TcpStream::connect(address).unwrap();
        connection.write(message.as_bytes())
    }

    /////////
    //TESTS//
    /////////
    #[test]
    fn instance_constructs_on_valid_address() {
        assert!(Instance::new("127.0.0.1:25001", 4).is_ok())
    }

    #[test]
    fn instance_errors_on_invalid_address() {
        assert!(Instance::new("266.788.123.1:7878", 5).is_err())
    }

    #[test]
    fn instance_accepts_connections() {
        const ADDRESS: &str = "127.0.0.1:1234";
        let _server = Instance::new(ADDRESS, 4);

        connect_and_send(ADDRESS, "Don't panic").unwrap();
    }
}
