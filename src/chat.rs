use serde::{Deserialize, Serialize};

pub struct Chat {
    messages: Vec<Message>,
}

impl Chat {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }

    pub fn add(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn get_messages(&self) -> &Vec<Message> {
        &self.messages
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Message {
    content: String,
    author: String,
}

impl Message {
    pub fn new(content: &str, author: &str) -> Message {
        Message {
            content: content.to_string(),
            author: author.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn chat_constructs() {
        Chat::new();
    }

    #[test]
    fn chat_supports_adding_messages() {
        let mut chat = Chat::new();
        chat.add(Message::new("ABC", "author"))
    }

    #[test]
    fn chat_contains_messages() {
        let mut chat = Chat::new();
        chat.add(Message::new("ABC", "author"));

        match chat.get_messages().get(0) {
            Some(..) => {}
            _ => panic!(),
        }
    }

    #[test]
    fn message_constructs() {
        Message::new("ABC", "RustFan");
    }

    #[test]
    fn message_holds_content() {
        const CONTENT: &str = "This is a message";
        assert_eq!(
            Message::new(CONTENT, "someone").content,
            CONTENT.to_string()
        );
    }

    #[test]
    fn message_holds_author() {
        const AUTHOR: &str = "Nickname";
        assert_eq!(Message::new("ffff", AUTHOR).author, AUTHOR.to_string());
    }
}
