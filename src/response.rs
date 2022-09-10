use crate::chat::Message;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
enum Response {
    SendMessages(Vec<Message>),
    MessageAdded(),
    Invalid,
}
