use crate::message::Message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Messages(Vec<Message>), // currently there's no simple way to serialize slices of arbitrary size
    MessageAdded(),
    EndMessageStream(),
    Invalid,
}

impl Response {
    pub fn parse(response: &[u8]) -> Response {
        serde_json::from_slice::<Response>(response).unwrap_or(Response::Invalid)
    }
}
