use serde::{Deserialize, Serialize};
use serde_json::from_slice;

use crate::chat::MessageProto;

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum Request<'a> {
    Send(MessageProto),
    FetchSince(usize),
    Invalid(&'a str),
}

impl<'a> Request<'a> {
    pub fn parse(request: &'a [u8]) -> Request<'a> {
        from_slice::<Request<'a>>(request).unwrap_or(Request::Invalid("Invalid request."))
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
        let req = to_vec(&Request::Send(MessageProto::new(content, author))).unwrap();
        assert_eq!(
            Request::parse(&req),
            Request::Send(MessageProto::new(content, author)),
        )
    }

    #[test]
    fn fetch_parses_correctly() {
        let since = 123usize;

        let req = to_vec(&Request::FetchSince(since)).unwrap();

        assert_eq!(Request::parse(&req), Request::FetchSince(since),)
    }

    #[test]
    fn invalid_requests_are_invalid() {
        assert_eq!(
            Request::parse(b"This shouldn't be a valid request"),
            Request::Invalid("Invalid request.")
        )
    }
}
