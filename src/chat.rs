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

    pub fn get_messages(&self, since: usize) -> &[Message] {
        &self.messages[since..]
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Message {
    content: String,
    author: String,
}

#[cfg(test)]
mod test {
    use super::*;

    impl Message {
        pub fn new(content: &str, author: &str) -> Message {
            Message {
                content: content.to_string(),
                author: author.to_string(),
            }
        }
    }

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
    fn chat_contains_added_message() {
        let mut chat = Chat::new();
        let message = Message::new("ABC", "author");
        let message_cmp = Message::new("ABC", "author");

        chat.add(message);

        assert_eq!(chat.messages[0], message_cmp)
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

    #[test]
    fn fetch_returns_corrent_num_of_messages() {
        const COUNT: usize = 4;
        const SINCE: usize = 2;

        let mut chat = Chat::new();
        for _ in 0..COUNT {
            chat.add(Message::new("ABC", "author"));
        }

        assert_eq!(chat.get_messages(SINCE).len(), 2)
    }
}
