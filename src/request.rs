use serde::{Deserialize, Serialize};
use serde_json::from_slice;

use crate::chat::Message;

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum ReqType<'a> {
    Send(Message),
    FetchSince(usize),
    Invalid(&'a str),
}

impl<'a> ReqType<'a> {
    pub fn parse(request: &'a [u8]) -> ReqType<'a> {
        from_slice::<ReqType<'a>>(request).unwrap_or(ReqType::Invalid("Invalid request."))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::to_vec;

    #[test]
    fn send_requests_parses_correctly() {
        let content = "Content";
        let author = "Author";
        let req = to_vec(&ReqType::Send(Message::new(content, author))).unwrap();
        assert_eq!(
            ReqType::parse(&req),
            ReqType::Send(Message::new(content, author)),
        )
    }

    #[test]
    fn fetch_parses_correctly() {
        let since = 123usize;

        let req = to_vec(&ReqType::FetchSince(since)).unwrap();

        assert_eq!(ReqType::parse(&req), ReqType::FetchSince(since),)
    }

    #[test]
    fn invalid_requests_are_invalid() {
        assert_eq!(
            ReqType::parse(b"This shouldn't be a valid request"),
            ReqType::Invalid("Invalid request.")
        )
    }
}
