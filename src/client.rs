use std::io;
use std::net::{SocketAddr, TcpStream};

use serde::de::Error;

use crate::chat::{Chat, Message};
use crate::response::Response;

struct Client {
    server_addr: SocketAddr,
    user: User,
    connection: Option<TcpStream>,
    chat: Chat,
}

struct User {
    name: String,
}

impl Client {
    pub fn new<T>(server_addr: T, user: User) -> Self
    where
        T: Into<SocketAddr>,
    {
        let a: SocketAddr = server_addr.into();

        Self {
            server_addr: a,
            user,
            connection: None,
            chat: Chat::new(),
        }
    }

    /// Connects to a server at address given when creating the client instance
    pub fn connect(&mut self) -> io::Result<()> {
        self.connection = Some(TcpStream::connect(self.server_addr)?);
        Ok(())
    }

    /// Returns chat messages starting at index `since`
    pub fn get_messages(&self, since: usize) -> &[Message] {
        &self.chat.get_messages(since)
    }

    pub fn handle_response(&mut self, response: &Response) -> io::Result<()> {
        match response {
            Response::Messages(messages) => {
                println!("Got {} messages", messages.len());
            }
            Response::MessageAdded() => {
                println!("Message sent and added succesfully.");
            }
            Response::Invalid => {
                println!("Server received invalid request.");
            }
        }
        Ok(())
    }
}
impl User {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{net::TcpListener, str::FromStr};

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
    fn client_handles_responses() {
        let addr = SocketAddr::from_str("127.0.0.1:38659").unwrap();
        let mut client = Client::new(addr, User::new("name"));

        assert!(client.handle_response(&Response::Invalid).is_ok());
        assert!(client.handle_response(&Response::MessageAdded()).is_ok());
        assert!(client.handle_response(&Response::Messages(vec![])).is_ok());
    }
}
