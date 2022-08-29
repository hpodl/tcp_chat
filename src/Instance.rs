use std::{
    io::{self, prelude::*},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

use super::ThreadPool::*;

struct Instance {
    listener: TcpListener,
    thread_pool: ThreadPool,
}

impl Instance {
    fn new<A>(address: A, thread_count: usize) -> io::Result<Self>
    where
        A: ToSocketAddrs,
    {
        Ok(Self {
            listener: TcpListener::bind(address)?,
            thread_pool: ThreadPool::new(thread_count),
        })
    }

    fn run() {}
}

pub fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // let request_line = buffer.lines().next().unwrap().unwrap();
    while let Some(message) = buffer.lines().next() {
        println!("{:?}", message);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::{io::Write, net::TcpStream};

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
