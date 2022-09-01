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

    pub fn get_messages(&self) -> Vec<(&str, &str)> {
        self.messages
            .iter()
            .map(|msg| (msg.content.as_str(), msg.author.as_str()))
            .collect()
    }
}

#[derive(Debug)]
pub struct Message {
    content: String,
    author: String,
}

impl Message {
    pub fn new<'b>(content: &[u8], author: &[u8]) -> Message {
        Message {
            content: content.escape_ascii().to_string(),
            author: author.escape_ascii().to_string(),
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
        chat.add(Message::new(b"ABC", b"author"))
    }

    #[test]
    fn chat_contains_messages() {
        let mut chat = Chat::new();
        chat.add(Message::new(b"ABC", b"author"));

        match chat.get_messages().get(0) {
            Some(..) => {}
            _ => panic!(),
        }
    }

    #[test]
    fn message_constructs() {
        Message::new(&[56, 56, 56], b"RustFan");
    }

    #[test]
    fn message_holds_content() {
        const CONTENT: &[u8] = b"This is a message";
        assert_eq!(
            Message::new(CONTENT, b"someone").content,
            CONTENT.escape_ascii().to_string()
        );
    }

    #[test]
    fn message_holds_author() {
        const AUTHOR: &[u8] = b"Nickname";
        assert_eq!(
            Message::new(b"ffff", AUTHOR).author,
            AUTHOR.escape_ascii().to_string()
        );
    }
}
