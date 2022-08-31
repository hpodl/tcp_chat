use core::slice::SlicePattern;
use std::{
    io::{self, prelude::*},
    net::{TcpListener, TcpStream, ToSocketAddrs}, slice,
};

use super::chat::{Chat, Message};

pub struct Instance<'a> {
    listener: TcpListener,
    chat: Chat<'a>,
}

impl<'a> Instance<'a> {
    /// Create a server instance on `address` socket
    ///
    /// # Errors
    /// Returns an error if binding a `TcpListener` to the given socket fails
    pub fn new<A>(address: A) -> io::Result<Self>
    where
        A: ToSocketAddrs,
    {
        Ok(Self {
            listener: TcpListener::bind(address)?,
            chat: Chat::new(),
        })
    }

    /// Loops over incoming connections and dispatches tasks of handling them
    ///
    /// # Panics and Errors
    /// All error handling is delegated to
    pub fn run(&mut self) {
        for mut stream in self.listener.incoming().flatten() {
            let mut buffer = [0; 1024];
         
            match stream.read(&mut buffer) {
                Ok(_) => {
                    let msg: &[u8] = buffer.iter().take_while(|&&byte| byte != 0).collect();
                    self.chat.add(Message::new(, "John"));
                }
                Err(e) => eprintln!("{}", e),
            }
        }
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
        assert!(Instance::new("127.0.0.1:25001").is_ok())
    }

    #[test]
    fn instance_errors_on_invalid_address() {
        assert!(Instance::new("266.788.123.1:7878").is_err())
    }

    #[test]
    fn instance_accepts_connections() {
        const ADDRESS: &str = "127.0.0.1:1234";
        let _server = Instance::new(ADDRESS);

        connect_and_send(ADDRESS, "Don't panic").unwrap();
    }
}
