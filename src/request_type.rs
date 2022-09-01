#[derive(Debug, PartialEq)]
pub enum ReqType<'a> {
    SendRequest(&'a [u8]),
    FetchSince(usize),
    Invalid,
}

impl<'a> ReqType<'a> {
    const HEADER_BYTES: usize = 20;
    const REQUEST_TYPE_BYTES: usize = 4;

    pub fn parse(request: &'a [u8]) -> ReqType<'a> {
        if request.len() < Self::HEADER_BYTES {
            return Self::Invalid;
        }

        match &request[..Self::REQUEST_TYPE_BYTES] {
            b"SEND" => Self::SendRequest(&request[Self::HEADER_BYTES..]),
            b"TAKE" => Self::FetchSince(unimplemented!()),
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
            ReqType::parse(b"SEND________________asd"),
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
}
