use std::str::from_utf8;

#[derive(Debug, PartialEq)]
pub enum ReqType<'a> {
    SendRequest(&'a [u8]),
    FetchSince(usize),
    Invalid,
}

impl<'a> ReqType<'a> {
    const HEADER_BYTES: usize = 12;
    const REQUEST_TYPE_BYTES: usize = 4;

    pub fn parse(request: &'a [u8]) -> ReqType<'a> {
        if request.len() < Self::HEADER_BYTES {
            return Self::Invalid;
        }

        match &request[..Self::REQUEST_TYPE_BYTES] {
            b"SEND" => Self::SendRequest(&request[Self::HEADER_BYTES..]),
            b"TAKE" => match str::parse(
                &request[Self::REQUEST_TYPE_BYTES..Self::HEADER_BYTES]
                    .escape_ascii()
                    .to_string(),
            ) {
                Ok(val) => Self::FetchSince(val),
                Err(_) => Self::Invalid,
            },
            _ => Self::Invalid,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn send_request_correctly_parses() {
        assert_eq!(
            ReqType::parse(b"SEND________asd"),
            ReqType::SendRequest(b"asd")
        )
    }

    #[test]
    fn too_short_request_is_invalid() {
        assert_eq!(ReqType::parse(b"not enough"), ReqType::Invalid);
    }

    #[test]
    fn invalid_request_type_is_invalid() {
        assert_eq!(ReqType::parse(b"SEDN________________asd"), ReqType::Invalid);
    }

    #[test]
    fn take_request_correctly_parses() {
        assert_eq!(ReqType::parse(b"TAKE00000123"), ReqType::FetchSince(123))
    }

    #[test]
    fn take_request_invalid_bytes() {
        assert_eq!(ReqType::parse(b"TAKE_____123"), ReqType::Invalid)
    }
}
