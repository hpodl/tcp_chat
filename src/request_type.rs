#[derive(Debug, PartialEq)]
pub enum ReqType<'a> {
    SendRequest((&'a [u8], &'a [u8])),
    FetchSince(usize),
    Invalid(&'a str),
}

impl<'a> ReqType<'a> {
    const HEADER_BYTES: usize = 20;
    const REQUEST_TYPE_BYTES: usize = 4;

    pub fn parse(request: &'a [u8]) -> ReqType<'a> {
        if request.len() < Self::HEADER_BYTES {
            return Self::Invalid("Request too short.");
        }

        println!("{:?}", &request[..Self::REQUEST_TYPE_BYTES]);
        match &request[..Self::REQUEST_TYPE_BYTES] {
            b"SEND" => Self::SendRequest((
                &request[Self::HEADER_BYTES..],
                &request[Self::REQUEST_TYPE_BYTES..Self::HEADER_BYTES],
            )),
            b"TAKE" => match str::parse(
                &request[Self::REQUEST_TYPE_BYTES..Self::HEADER_BYTES]
                    .escape_ascii()
                    .to_string(),
            ) {
                Ok(val) => Self::FetchSince(val),
                Err(e) => {
                    println!("{}", e);
                    Self::Invalid("Couldn't parse the request.")
                }
            },
            _ => Self::Invalid("Not a valid request type."),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn send_request_correctly_parses() {
        assert_eq!(
            ReqType::parse(b"SEND__________authorab"),
            ReqType::SendRequest((b"ab", b"__________author"))
        )
    }

    #[test]
    fn too_short_request_is_invalid() {
        assert_eq!(
            ReqType::parse(b"not enough"),
            ReqType::Invalid("Request too short.")
        );
    }

    #[test]
    fn invalid_request_type_is_invalid() {
        assert_eq!(
            ReqType::parse(b"SEDN________________asd"),
            ReqType::Invalid("Not a valid request type.")
        );
    }

    #[test]
    fn take_request_correctly_parses() {
        assert_eq!(
            ReqType::parse(b"TAKE0000000000000123"),
            ReqType::FetchSince(123)
        )
    }

    #[test]
    fn take_request_invalid_bytes() {
        assert_eq!(
            ReqType::parse(b"TAKE________________123"),
            ReqType::Invalid("Couldn't parse the request.")
        )
    }
}
