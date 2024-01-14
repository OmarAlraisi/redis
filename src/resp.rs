use std::iter::Peekable;

use crate::resp_data::RESPData;

pub struct RESP;

impl RESP {
    pub fn deserialize(input_message: &[u8]) -> RESPData {
        let message = match String::from_utf8_lossy(input_message) {
            std::borrow::Cow::Borrowed(str) => String::from(str),
            std::borrow::Cow::Owned(str) => str,
        };
        let mut tokens = message.chars().peekable();
        RESP::deserialze_tokens(&mut tokens)
    }

    fn deserialze_tokens<I: Iterator<Item = char>>(tokens: &mut Peekable<I>) -> RESPData {
        if tokens.peek().is_none() {
            return RESPData::Error(String::from("Empty message!"));
        }

        let message = RESPData::parse_message(tokens);

        if tokens.next().is_some() {
            RESPData::Error(String::from("Invalid Message!!"))
        } else {
            message
        }
    }

    pub fn serialize(resp_data: RESPData) -> Vec<u8> {
        format!("{}", resp_data).as_bytes().to_owned()
    }
}
