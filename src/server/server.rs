use std::io::{self, BufWriter};
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};

use internet_chat::chat::Chat;
use internet_chat::message::Message;
use internet_chat::request::Request;
use internet_chat::response::Response;

pub struct Instance {
    address: SocketAddr,
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
        let address = address
            .to_socket_addrs()?
            .next()
            .ok_or(io::Error::new(io::ErrorKind::Other, "Incorrect address."))?;

        Ok(Self {
            address: address,
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
        let listener = TcpListener::bind(self.address).expect("Couldn't bind listener address.");

        for stream in listener.incoming().flatten() {
            match self.handle_stream(&stream) {
                Ok(_) => {
                    println!("Stream handled succesfully.")
                }
                Err(_) => {
                    println!("Error handling stream with: {:?}.", stream.peer_addr())
                }
            };
        }
    }

    fn handle_stream(&mut self, stream: &TcpStream) -> io::Result<()> {
        let reader = BufReader::new(stream.try_clone()?);
        let mut writer = stream;

        let mut write_with_newline = |data: &[u8]| -> io::Result<()> {
            writer.write_all(data)?;
            writer.write_all(b"\n")
        };

        for line in reader.lines() {
            let line = line?;
            match Request::parse_str(&line) {
                Request::Send(msg) => {
                    self.chat.add(msg);
                    write_with_newline(b"Ok")?;
                }
                Request::FetchSince(since) => {
                    let messages: Vec<Message> = self.chat.get_messages(since).iter().map(|x| x.clone()).collect();
                    write_with_newline(&serde_json::to_vec(&Response::Messages(messages))?)?;
                }
                Request::Invalid(_) => {
                    write_with_newline(b"Invalid request")?;
                }
            };
        }

        Ok(())
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
        connection.write(message.as_bytes()).unwrap();
        connection.write(b"\n")
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
}
