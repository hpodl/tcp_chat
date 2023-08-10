use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpStream};

use internet_chat::chat::Chat;
use internet_chat::message::{Message, MessageProto};
use internet_chat::request::Request;
use internet_chat::response::Response;

pub struct Client {
    server_addr: SocketAddr,
    user: User,
    connection: Option<TcpStream>,
    chat: Chat,
}

pub struct User {
    name: String,
}

impl Client {
    pub fn new<T>(server_addr: T, user: User) -> Self
    where
        T: Into<SocketAddr>,
    {
        Self {
            server_addr: server_addr.into(),
            user,
            connection: None,
            chat: Chat::new(),
        }
    }

    /// Connects to a server at address provided in `Client::new()`
    pub fn connect(&mut self) -> io::Result<()> {
        self.connection = Some(TcpStream::connect(self.server_addr)?);
        Ok(())
    }

    /// Returns chat messages starting at index `since`
    pub fn local_messages(&self, since: usize) -> &[Message] {
        self.chat.get_messages(since)
    }

    /// Sends a message with `message` contents to a server at adress provided in `Client::new()`
    pub fn send_message(&mut self, message: &str) -> io::Result<()> {
        let mut stream = self.get_connection()?;

        let request_buf =
            serde_json::to_vec(&Request::Send(MessageProto::new(message, &self.user.name)))?;
        stream.write_all(&request_buf)?;
        stream.write_all(b"\n")?;

        
        let mut reader = BufReader::new(stream);
        let mut buf = String::new();
        reader.read_line(&mut buf)?;

        println!("Got {}", buf);
        
        Ok(())
    }

    pub fn request_messages(&mut self) -> io::Result<()> {
        let stream = self.get_connection()?;

        let mut writer = (*stream).try_clone()?;
        let reader = BufReader::new(stream);

        writer.write_all(&serde_json::to_vec(&Request::FetchSince(
            self.chat.current_id(),
        ))?)?;

        writer.write_all(b"\n")?;

        for response in reader.lines() {
            let response = response?;
            if response.is_empty() {
                continue;
            }

            match serde_json::from_str::<Response>(&response)? {
                Response::Messages(messages) => {
                    for message in messages {
                        self.chat.add(message.content)
                    }
                }
                Response::Invalid => {
                    println!("Response: Invalid.")
                }
                _ => {
                    unreachable!()
                }
            };
            break;
        }

        Ok(())
    }

    fn get_connection(&self) -> io::Result<&TcpStream> {
        self.connection.as_ref().ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Not connected to a server.",
        ))
    }
}
impl User {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use std::net::TcpListener;
    use std::str::FromStr;

    #[test]
    fn user_exists() {
        User::new("username");
    }

    #[test]
    fn client_exists() {
        Client::new(
            SocketAddr::from_str("127.0.0.1:4545").unwrap(),
            User::new("some_username"),
        );
    }

    #[test]
    fn client_connect_existing_ok() {
        let addr = SocketAddr::from_str("127.0.0.1:44856").unwrap();

        // named, so that it's not dropped instantly
        let _listener = TcpListener::bind(addr).unwrap();

        let mut client = Client::new(addr, User::new("name"));
        assert!(client.connect().is_ok());
    }

    #[test]
    fn client_connect_nonexisting_errors() {
        let addr = SocketAddr::from_str("127.0.0.1:38659").unwrap();

        let mut client = Client::new(addr, User::new("name"));
        assert!(client.connect().is_err());
    }

    #[test]
    fn client_sends_message() {
        let addr = SocketAddr::from_str("127.0.0.1:38659").unwrap();
        let mut client = Client::new(addr, User::new("name"));

        // named, so that it's not dropped instantly
        let listener = TcpListener::bind(addr).unwrap();

        client.connect().unwrap();

        let message_content = "Don't panic.";
        assert!(client.send_message(message_content).is_ok());

        let mut stream = listener.incoming().next().unwrap().unwrap();

        let mut buf = [0u8; 1024];
        let bytes_read = stream.read(&mut buf).unwrap();

        // not empty
        assert!(!buf.starts_with(&[0]));

        let message = MessageProto::new(message_content, &client.user.name);
        let buf_should_be = serde_json::to_vec(&Request::Send(message)).unwrap();

        // Skipping newline
        assert_eq!(buf[..(bytes_read - 1)], buf_should_be[..]);
    }
}
