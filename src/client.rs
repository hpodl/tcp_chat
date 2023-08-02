use std::io;
use std::net::{SocketAddr, TcpStream};

#[allow(unused)]
struct Client {
    server_addr: SocketAddr,
    user: User,
    connection: Option<TcpStream>,
}

struct User {
    name: String,
}

impl Client {
    fn new<T>(server_addr: T, user: User) -> Self
    where
        T: Into<SocketAddr>,
    {
        let a: SocketAddr = server_addr.into();

        Self {
            server_addr: a,
            user,
            connection: None,
        }
    }

    /// Connects to a server at address given when creating the client instance
    fn connect(&mut self) -> io::Result<()> {
        self.connection = Some(TcpStream::connect(self.server_addr)?);
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
    fn connect_existing() {
        let addr = SocketAddr::from_str("127.0.0.1:44856").unwrap();

        // named, so that it's not dropped instantly
        let _listener = TcpListener::bind(addr).unwrap();

        let mut client = Client::new(addr, User::new("name"));
        assert!(client.connect().is_ok());
    }

    #[test]
    fn connect_nonexisting_errors() {
        let addr = SocketAddr::from_str("127.0.0.1:38659").unwrap();

        let mut client = Client::new(addr, User::new("name"));
        assert!(client.connect().is_err());
    }
}
