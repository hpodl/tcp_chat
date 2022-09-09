use std::{
    io::{self, prelude::*},
    net::{Shutdown, TcpListener, TcpStream, ToSocketAddrs},
};

use super::chat::{Chat, Message};
use super::request::ReqType;

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

    /// Loops over incoming connections and dispatches tasks of handling them
    ///
    /// # Panics and Errors
    /// All error handling is delegated to
    pub fn run(&mut self) {
        for mut stream in self.listener.incoming().flatten() {
            let mut buffer = [0; 1024];

            if stream.read(&mut buffer).is_ok() {
                match ReqType::parse(&buffer) {
                    ReqType::Send(msg) => self.chat.add(msg),
                    ReqType::FetchSince(_TODO) => {
                        stream
                            .write(&self.chat.get_messages().iter().fold(
                                Vec::<u8>::new(),
                                |mut all, current| {
                                    all.append(&mut serde_json::to_vec(current).unwrap());
                                    all
                                },
                            ))
                            .unwrap();
                    }
                    ReqType::Invalid(_) => {
                        stream.write(b"TODO").unwrap();
                    }
                };
            }
        }
    }
}

fn format_messages<'a>(messages: Vec<(&'a str, &'a str)>) -> String {
    let format_message = |(content, author): (&str, &str)| format!("{}: {}\n", author, content);

    messages
        .iter()
        .map(|&message| format_message(message))
        .collect::<String>()
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
