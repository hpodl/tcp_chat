use crate::chat::MessageProto;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Response {
    Messages(Vec<MessageProto>), // currently there's no simple way to serialize slices of arbitrary size
    MessageAdded(),
    Invalid,
}
