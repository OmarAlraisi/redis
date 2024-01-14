use std::iter::Peekable;

use crate::resp_data::RESPData;

pub struct RESP;

impl RESP {
    pub fn deserialize(input_message: String) -> RESPData {
        let mut tokens = input_message.chars().peekable();
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

    // fn parse_resp_data<I: Iterator<Item = char>>(
    //     tokens: &mut Peekable<I>,
    // ) -> RESPData {
    //     match tokens.next().unwrap() {
    //         '+' => RESPData::parse_simple_string(tokens),
    //         '-' => RESPData::parse_error(tokens),
    //         ':' => RESPData::parse_integer(tokens),
    //         '$' => RESPData::parse_bulk_string(tokens),
    //         '*' => RESPData::parse_arrays(tokens), 
    //         _ => RESPData::Error(String::from("Invalid Data Type!!")),
    //     }
    // }
}
