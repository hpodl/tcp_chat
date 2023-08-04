use crate::message::MessageProto;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Response {
    Messages(Vec<MessageProto>), // currently there's no simple way to serialize slices of arbitrary size
    MessageAdded(),
    Invalid,
}

impl Response {
    pub fn parse<'a>(response: &'a [u8]) -> Response {
        serde_json::from_slice::<Response>(response).unwrap_or(Response::Invalid)
    }
}
