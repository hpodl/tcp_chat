mod client;
use std::{net::SocketAddr, str::FromStr};

use client::*;

fn main() {
    let mut client = Client::new(
        SocketAddr::from_str("127.0.0.1:5353").expect("Not a valid address."),
        User::new("Uname"),
    );

    client
        .connect()
        .expect("Couldn't connect to server at given address");
    client
        .send_message("Hey, this is a message.")
        .expect("Failed when sending a message");
    client
        .send_message("This is also a message.")
        .expect("Failed when sending a message");

    client.request_messages().unwrap();

    for message in client.local_messages(0) {
        println!("{}", message);
    }
}
