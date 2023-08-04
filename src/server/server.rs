use std::{
    io::{self, prelude::*},
    net::{TcpListener, ToSocketAddrs},
};

use internet_chat::chat::Chat;
use internet_chat::request::Request;

pub struct Instance {
    listener: TcpListener,
    chat: Chat,
}

impl Instance {
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

    /// Main server looop.
    ///
    /// Loops over incoming connections and handles them
    ///
    /// # Panics and Errors
    /// All error handling is delegated to
    pub fn run(&mut self) {
        for mut stream in self.listener.incoming().flatten() {
            let mut buffer = [0; 1024];

            if let Ok(bytes_read) = stream.read(&mut buffer) {
                println!(
                    "Received {}.",
                    String::from_utf8_lossy(&buffer[..bytes_read])
                );
                let requests = buffer[..bytes_read].split(|x| *x == b'\n');
                for request in requests {
                    match Request::parse(request) {
                        Request::Send(msg) => self.chat.add(msg),
                        Request::FetchSince(since) => {
                            match stream.write_all(&self.chat.get_messages(since).iter().fold(
                                Vec::<u8>::new(),
                                |mut all, current| {
                                    all.append(&mut serde_json::to_vec(current).unwrap());
                                    all.push(b'\n');
                                    all
                                },
                            )) {
                                Ok(_) => {}
                                Err(_) => println!("Couldn't write into buffer."),
                            }
                        }
                        Request::Invalid(_) => {
                            match stream.write_all(b"Invalid request") {
                                Ok(_) => {}
                                Err(_) => println!("Couldn't write into buffer."),
                            };
                        }
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::net::TcpStream;

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
