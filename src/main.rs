use std::{
    io,
    net::{TcpListener, ToSocketAddrs},
};

struct Instance {
    listener: TcpListener,
}

impl Instance {
    fn new<A>(address: A) -> io::Result<Self>
    where
        A: ToSocketAddrs,
    {
        Ok(Self {
            listener: TcpListener::bind(address)?,
        })
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use std::{net::TcpStream, io::Write};

    use super::*;

    // Helper function
    fn connect_and_send<A>(address: A, message: &str)  -> io::Result<usize>
    where A: ToSocketAddrs
    {
        let mut connection = TcpStream::connect(address).unwrap();
        let buf = b"Messsage";

        connection.write(buf)
    }

    #[test]
    fn instance_constructs_on_valid_address() {
        assert!(Instance::new("127.0.0.1:25001").is_ok())
    }

    #[test]
    fn instance_errors_on_invalid_address() {
        assert!(Instance::new("266.788.123.1:7878").is_err())
    }
}
