use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Message {
    pub content: MessageProto,
    pub id: usize,
}

impl Message {
    pub fn new(content: MessageProto, id: usize) -> Self {
        Self { content, id }
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct MessageProto {
    pub content: String,
    pub user: String,
}

impl MessageProto {
    pub fn new(content: &str, user: &str) -> Self {
        Self {
            content: content.to_owned(),
            user: user.to_owned(),
        }
    }
}
