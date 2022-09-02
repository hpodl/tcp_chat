use std::{
    io::{self, prelude::*},
    net::{TcpListener, ToSocketAddrs, Shutdown},
};

use super::chat::{Chat, Message};
use super::request_type::ReqType;
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
            let mut buffer = String::new();
            buffer.reserve(1024);

            match stream.read_to_string(&mut buffer) {
                Ok(_) => {
                    for request in buffer.split_terminator('\n') {
                        match ReqType::parse(request.as_bytes()) {
                            ReqType::SendRequest((msg, author)) => {
                                println!("Send");
                                self.chat.add(Message::new(msg, author));
                            }
                            ReqType::FetchSince(_) => {
                                println!("Fetch");
                                let to_send = format_messages(self.chat.get_messages());
                                println!("{}", to_send);
                                match stream.write_all(to_send.as_bytes()) {
                                    Ok(_) => {}
                                    Err(e) => eprintln!("Error writing into stream: {}", e),
                                }
                            }
                            ReqType::Invalid(e) => {
                                println!("Invalid: {}", e);
                                if stream.write(b"Invalid request").is_err() {
                                    eprintln!("Error writing into stream: {}", e);
                                };
                            }
                        }
                    }
                    if stream.shutdown(Shutdown::Write).is_err() {
                        eprintln!("Failed to shut down the write side of connection.");
                    };
                }
                Err(e) => eprintln!("{}", e),
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
