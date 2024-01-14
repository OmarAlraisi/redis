use std::{fmt::Display, iter::Peekable};

#[derive(Debug)]
pub enum RESPData {
    SimpleString(String),
    Error(String),
    Integer(i32),
    BulkString(String),
    Array(Vec<RESPData>),
    Null,
}

impl Display for RESPData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RESPData::SimpleString(val) => write!(f, "+{}\r\n", val),
            RESPData::Error(val) => write!(f, "-{}\r\n", val),
            RESPData::Integer(val) => write!(f, ":{}\r\n", val),
            RESPData::BulkString(val) => write!(f, "${}\r\n{}\r\n", val.len(), val),
            RESPData::Array(val) => {
                let mut output = format!("*{}\r\n", val.len());
                for entry in val {
                    output.push_str(format!("{}", entry).as_str());
                }
                write!(f, "{}", output)
            }
            RESPData::Null => write!(f, "_\r\n"),
        }
    }
}

impl PartialEq for RESPData {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::SimpleString(left), Self::SimpleString(right)) => left == right,
            (Self::Error(left), Self::Error(right)) => left == right,
            (Self::Integer(left), Self::Integer(right)) => left == right,
            (Self::BulkString(left), Self::BulkString(right)) => left == right,
            (Self::Array(left), Self::Array(right)) => {
                if left.len() != right.len() {
                    return false;
                }
                for idx in 0..left.len() {
                    if left[idx] != right[idx] {
                        return false;
                    }
                }
                true
            }
            (Self::Null, Self::Null) => true,
            _ => false,
        }
    }
}

impl RESPData {
    fn get_argument<I: Iterator<Item = char>>(tokens: &mut Peekable<I>) -> Option<String> {
        let mut argument = String::new();
        while let Some(ch) = tokens.next() {
            match ch {
                '\r' => match tokens.next() {
                    Some(ch) => {
                        if ch == '\n' {
                            return Some(argument);
                        } else {
                            return None;
                        }
                    }
                    None => return None,
                },
                _ => argument.push(ch),
            }
        }

        if argument.is_empty() {
            return Some(String::new());
        } else {
            // Message was not terminated
            None
        }
    }

    pub fn parse_message<I: Iterator<Item = char>>(tokens: &mut Peekable<I>) -> RESPData {
        match tokens.next().unwrap() {
            '+' => RESPData::parse_simple_string(tokens),
            '-' => RESPData::parse_error(tokens),
            ':' => RESPData::parse_integer(tokens),
            '$' => RESPData::parse_bulk_string(tokens),
            '*' => RESPData::parse_arrays(tokens),
            _ => RESPData::Error(String::from("Invalid Data Type!!")),
        }
    }

    fn parse_simple_string<I: Iterator<Item = char>>(tokens: &mut Peekable<I>) -> Self {
        match RESPData::get_argument(tokens) {
            Some(simple_string) => RESPData::SimpleString(simple_string),
            None => RESPData::Error(String::from("Could not parse simple string")),
        }
    }

    fn parse_error<I: Iterator<Item = char>>(tokens: &mut Peekable<I>) -> Self {
        match RESPData::get_argument(tokens) {
            Some(simple_string) => RESPData::SimpleString(simple_string),
            None => RESPData::Error(String::from("Could not parse error")),
        }
    }

    fn parse_integer<I: Iterator<Item = char>>(tokens: &mut Peekable<I>) -> Self {
        match RESPData::get_argument(tokens) {
            Some(argument) => match argument.parse::<i32>() {
                Ok(integer) => RESPData::Integer(integer),
                Err(_) => RESPData::Error(String::from("Invalid integer!")),
            },
            None => RESPData::Error(String::from("Invalid integer!")),
        }
    }

    fn parse_bulk_string<I: Iterator<Item = char>>(tokens: &mut Peekable<I>) -> Self {
        let length = match RESPData::get_argument(tokens) {
            Some(arg) => match arg.parse::<i32>() {
                Ok(len) => len,
                Err(_) => return RESPData::Error(String::from("Invalid bulk string length!")),
            },
            None => return RESPData::Error(String::from("Invalid bulk string message!")),
        };

        if length == -1 {
            return RESPData::Null;
        }

        let blk_string = match RESPData::get_argument(tokens) {
            Some(arg) => arg,
            None => return RESPData::Error(String::from("Invalid bulk string message!")),
        };

        if blk_string.len() as i32 != length {
            return RESPData::Error(String::from("Invalid bulk string message!"));
        } else {
            RESPData::BulkString(blk_string)
        }
    }

    fn parse_arrays<I: Iterator<Item = char>>(tokens: &mut Peekable<I>) -> Self {
        let len = match RESPData::get_argument(tokens) {
            Some(arg) => match arg.parse::<usize>() {
                Ok(len) => len,
                Err(_) => return RESPData::Error(String::from("Invalid array length!")),
            },
            None => return RESPData::Error(String::from("Invalid array length!")),
        };

        let mut array: Vec<RESPData> = Vec::with_capacity(len);

        for _ in 0..len {
            array.push(RESPData::parse_message(tokens));
        }

        RESPData::Array(array)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::RESPData;

//     #[test]
//     fn null_bulk_string() {
//         let message = String::from("$-1\r\n");
//         assert_eq!(RESPData::new(message), Ok(RESPData::Null));
//     }

//     #[test]
//     fn one_element_array() {
//         let message = String::from("*1\r\n$4\r\nping\r\n");
//         let should_be = RESPData::Array(vec![RESPData::BulkString(String::from("ping"))]);
//         assert_eq!(RESPData::new(message), Ok(should_be));
//     }

//     #[test]
//     fn two_elements_array() {
//         let message = String::from("*2\r\n$4\r\necho\r\n$11\r\nhello world\r\n");
//         let should_be = RESPData::Array(vec![
//             RESPData::BulkString(String::from("echo")),
//             RESPData::BulkString(String::from("hello world")),
//         ]);
//         assert_eq!(RESPData::new(message), Ok(should_be));
//     }

//     #[test]
//     fn another_two_elements_array() {
//         let message = String::from("*2\r\n$3\r\nget\r\n$3\r\nkey\r\n");
//         let should_be = RESPData::Array(vec![
//             RESPData::BulkString(String::from("get")),
//             RESPData::BulkString(String::from("key")),
//         ]);
//         assert_eq!(RESPData::new(message), Ok(should_be));
//     }

//     #[test]
//     fn simple_string() {
//         let message = String::from("+OK\r\n");
//         assert_eq!(
//             RESPData::new(message),
//             Ok(RESPData::SimpleString(String::from("OK")))
//         );
//     }

//     #[test]
//     fn error() {
//         let message = String::from("-Error message\r\n");
//         assert_eq!(
//             RESPData::new(message),
//             Ok(RESPData::Error(String::from("Error message")))
//         );
//     }

//     #[test]
//     fn empty_bulk_string() {
//         let message = String::from("$0\r\n\r\n");
//         assert_eq!(
//             RESPData::new(message),
//             Ok(RESPData::BulkString(String::from("")))
//         );
//     }

//     #[test]
//     fn another_simple_string() {
//         let message = String::from("+hello world\r\n");
//         assert_eq!(
//             RESPData::new(message),
//             Ok(RESPData::SimpleString(String::from("hello world")))
//         );
//     }

//     #[test]
//     fn integer() {
//         let message = String::from(":19\r\n");
//         assert_eq!(RESPData::new(message), Ok(RESPData::Integer(19)));
//     }

//     #[test]
//     fn invalid_data_type() {
//         let message = String::from("#19\r\n");
//         assert_eq!(
//             RESPData::new(message),
//             Err(String::from("Invalid Data Type!!"))
//         );
//     }

//     #[test]
//     fn invalid_integer() {
//         let message = String::from(":Hello world\r\n");
//         assert_eq!(
//             RESPData::new(message),
//             Err(String::from("Invalid integer!"))
//         );
//     }
// }
