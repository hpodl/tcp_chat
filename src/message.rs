use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Message {
    pub content: MessageProto,
    pub id: usize,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}]{}: {}", self.id, self.content.user, self.content.content))
    }
}

impl Message {
    pub fn new(content: MessageProto, id: usize) -> Self {
        Self { content, id }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
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
