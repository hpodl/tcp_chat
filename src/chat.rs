use crate::message::{Message, MessageProto};

#[derive(Default)]
pub struct Chat {
    messages: Vec<Message>,
    current_id: usize,
}

impl Chat {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds the message to chat history
    pub fn add(&mut self, message_content: MessageProto) {
        self.messages
            .push(Message::new(message_content, self.current_id));
        self.current_id += 1;
    }

    /// Returns a slice of messages with id greater
    /// than `since`.
    ///
    /// It is assumed that `id` corresponds to the index in `self.messages`;
    /// this can change later on.
    ///
    /// If `since` is greater than the highest id, an empty
    /// slice is returned
    pub fn get_messages(&self, since: usize) -> &[Message] {
        if since < self.messages.len() {
            &self.messages[since..]
        } else {
            &[]
        }
    }

    pub fn current_id(&self) -> usize {
        self.current_id
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
        chat.add(MessageProto::new("ABC", "author"))
    }

    #[test]
    fn chat_contains_added_message() {
        let mut chat = Chat::new();
        let message = MessageProto::new("ABC", "author");
        let message_cmp = MessageProto::new("ABC", "author");

        chat.add(message);

        assert_eq!(chat.messages[0].content, message_cmp)
    }

    #[test]
    fn message_constructs() {
        MessageProto::new("ABC", "RustFan");
    }

    #[test]
    fn message_holds_content() {
        const CONTENT: &str = "This is a message";
        assert_eq!(
            MessageProto::new(CONTENT, "someone").content,
            CONTENT.to_string()
        );
    }

    #[test]
    fn message_holds_author() {
        const AUTHOR: &str = "Nickname";
        assert_eq!(MessageProto::new("ffff", AUTHOR).user, AUTHOR.to_string());
    }

    #[test]
    fn fetch_returns_corrent_num_of_messages() {
        const COUNT: usize = 4;
        const SINCE: usize = 2;

        let mut chat = Chat::new();
        for _ in 0..COUNT {
            chat.add(MessageProto::new("ABC", "author"));
        }

        assert_eq!(chat.get_messages(SINCE).len(), 2)
    }
}
