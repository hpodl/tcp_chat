use std::slice;
use std::vec;

pub struct Chat<'a> {
    messages: Vec<Message<'a>>,
}

impl<'a> Chat<'a> {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }

    pub fn add(&mut self, message: Message<'a>) {
        println!("Adding message: {:?}", message);
        self.messages.push(message);
    }

    pub fn get_messages(&'a self) -> &'a Vec<Message<'a>> {
        &&self.messages
    }
}

#[derive(Debug)]
pub struct Message<'a> {
    content: Vec<u8>,
    author: &'a str,
}

impl<'a, 'b> Message<'a> {
    pub fn new(content: &'b [u8], author: &'a str) -> Message<'a> {
        Message::<'a> {
            content: content.into(),
            author,
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
        chat.add(Message::new(b"ABC", "author"))
    }

    #[test]
    fn chat_contains_messages() {
        let mut chat = Chat::new();
        chat.add(Message::new(b"ABC", "author"));

        match chat.get_messages().get(0) {
            Some(Message { .. }) => {}
            _ => panic!(),
        }
    }

    #[test]
    fn message_constructs() {
        Message::new(&[56, 56, 56], "RustFan");
    }

    #[test]
    fn message_holds_content() {
        const CONTENT: &[u8] = b"This is a message";
        assert_eq!(Message::new(CONTENT, "someone").content, CONTENT);
    }

    #[test]
    fn message_holds_author() {
        const AUTHOR: &'static str = "Nickname";
        assert_eq!(Message::new(b"ffff", AUTHOR).author, AUTHOR);
    }
}
