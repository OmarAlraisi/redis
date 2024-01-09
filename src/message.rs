use std::iter::Peekable;

#[derive(Debug)]
pub enum Message {
    SimpleString(String),
    Error(String),
    Integer(i32),
    BulkString(String),
    Array(Vec<Message>),
    Null,
}

impl PartialEq for Message {
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

impl Message {
    pub fn new(message: String) -> Result<Self, String> {
        let mut tokens = message.chars().peekable();
        Message::serialize_message(&mut tokens)
    }

    fn serialize_message<I: Iterator<Item = char>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Self, String> {
        if tokens.peek().is_none() {
            return Err(String::from("Empty message!"));
        }

        let message = match Message::parse_message(tokens) {
            Ok(message) => message,
            Err(err) => return Err(err),
        };

        if tokens.next().is_some() {
            Err(String::from("Invalid Message!!"))
        } else {
            Ok(message)
        }
    }

    fn parse_message<I: Iterator<Item = char>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Message, String> {
        match tokens.next().unwrap() {
            '+' => match Message::parse_simple_string(tokens) {
                Ok(string) => Ok(Message::SimpleString(string)),
                Err(err) => Err(err),
            },
            '-' => match Message::parse_simple_string(tokens) {
                Ok(error) => Ok(Message::Error(error)),
                Err(err) => Err(err),
            },
            ':' => match Message::parse_integer(tokens) {
                Ok(integer) => Ok(Message::Integer(integer)),
                Err(err) => Err(err),
            },
            '$' => match Message::parse_bulk_string(tokens) {
                Ok(blkstr) => Ok(Message::BulkString(blkstr)),
                Err(err) => {
                    if err == String::from("NULL") {
                        Ok(Message::Null)
                    } else {
                        Err(err)
                    }
                }
            },
            '*' => match Message::parse_arrays(tokens) {
                Ok(array) => Ok(Message::Array(array)),
                Err(err) => Err(err),
            },
            _ => return Err(String::from("Invalid Data Type!!")),
        }
    }

    fn get_argument<I: Iterator<Item = char>>(tokens: &mut Peekable<I>) -> Option<String> {
        let mut argument = String::new();
        while let Some(ch) = tokens.next() {
            match ch {
                '\\' => {
                    let mut escaping = String::from("\\");
                    for _ in 0..3 {
                        match tokens.next() {
                            Some(ch) => escaping.push(ch),
                            None => return None,
                        }
                    }

                    if escaping != "\\r\\n" {
                        return None;
                    }

                    return Some(argument);
                }
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

    fn parse_simple_string<I: Iterator<Item = char>>(
        tokens: &mut Peekable<I>,
    ) -> Result<String, String> {
        match Message::get_argument(tokens) {
            Some(simple_string) => Ok(simple_string),
            None => Err(String::from("Could not parse simple string")),
        }
    }

    fn parse_integer<I: Iterator<Item = char>>(tokens: &mut Peekable<I>) -> Result<i32, String> {
        match Message::get_argument(tokens) {
            Some(argument) => match argument.parse::<i32>() {
                Ok(integer) => Ok(integer),
                Err(_) => Err(String::from("Invalid integer!")),
            },
            None => Err(String::from("Invalid integer!")),
        }
    }

    fn parse_bulk_string<I: Iterator<Item = char>>(
        tokens: &mut Peekable<I>,
    ) -> Result<String, String> {
        let length = match Message::get_argument(tokens) {
            Some(arg) => match arg.parse::<i32>() {
                Ok(len) => len,
                Err(_) => return Err(String::from("Invalid bulk string length!")),
            },
            None => return Err(String::from("Invalid bulk string message!")),
        };

        if length == -1 {
            return Err(String::from("NULL"));
        }

        let blk_string = match Message::get_argument(tokens) {
            Some(arg) => arg,
            None => return Err(String::from("Invalid bulk string message!")),
        };

        if blk_string.len() as i32 != length {
            return Err(String::from("Invalid bulk string message!"));
        } else {
            Ok(blk_string)
        }
    }

    fn parse_arrays<I: Iterator<Item = char>>(
        tokens: &mut Peekable<I>,
    ) -> Result<Vec<Message>, String> {
        let len = match Message::get_argument(tokens) {
            Some(arg) => match arg.parse::<usize>() {
                Ok(len) => len,
                Err(_) => return Err(String::from("Invalid array length!")),
            },
            None => return Err(String::from("Invalid array length!")),
        };

        let mut array: Vec<Message> = Vec::with_capacity(len);

        for _ in 0..len {
            match Message::parse_message(tokens) {
                Ok(message) => array.push(message),
                Err(err) => return Err(err),
            }
        }

        Ok(array)
    }
}

#[cfg(test)]
mod tests {
    use super::Message;

    #[test]
    fn null_bulk_string() {
        let message = String::from("$-1\\r\\n");
        assert_eq!(Message::new(message), Ok(Message::Null));
    }

    #[test]
    fn one_element_array() {
        let message = String::from("*1\\r\\n$4\\r\\nping\\r\\n");
        let should_be = Message::Array(vec![Message::BulkString(String::from("ping"))]);
        assert_eq!(Message::new(message), Ok(should_be));
    }

    #[test]
    fn two_elements_array() {
        let message = String::from("*2\\r\\n$4\\r\\necho\\r\\n$11\\r\\nhello world\\r\\n");
        let should_be = Message::Array(vec![
            Message::BulkString(String::from("echo")),
            Message::BulkString(String::from("hello world")),
        ]);
        assert_eq!(Message::new(message), Ok(should_be));
    }

    #[test]
    fn another_two_elements_array() {
        let message = String::from("*2\\r\\n$3\\r\\nget\\r\\n$3\\r\\nkey\\r\\n");
        let should_be = Message::Array(vec![
            Message::BulkString(String::from("get")),
            Message::BulkString(String::from("key")),
        ]);
        assert_eq!(Message::new(message), Ok(should_be));
    }

    #[test]
    fn simple_string() {
        let message = String::from("+OK\\r\\n");
        assert_eq!(
            Message::new(message),
            Ok(Message::SimpleString(String::from("OK")))
        );
    }

    #[test]
    fn error() {
        let message = String::from("-Error message\\r\\n");
        assert_eq!(
            Message::new(message),
            Ok(Message::Error(String::from("Error message")))
        );
    }

    #[test]
    fn empty_bulk_string() {
        let message = String::from("$0\\r\\n\\r\\n");
        assert_eq!(
            Message::new(message),
            Ok(Message::BulkString(String::from("")))
        );
    }

    #[test]
    fn another_simple_string() {
        let message = String::from("+hello world\\r\\n");
        assert_eq!(
            Message::new(message),
            Ok(Message::SimpleString(String::from("hello world")))
        );
    }

    #[test]
    fn integer() {
        let message = String::from(":19\\r\\n");
        assert_eq!(Message::new(message), Ok(Message::Integer(19)));
    }

    #[test]
    fn invalid_data_type() {
        let message = String::from("#19\\r\\n");
        assert_eq!(
            Message::new(message),
            Err(String::from("Invalid Data Type!!"))
        );
    }

    #[test]
    fn invalid_integer() {
        let message = String::from(":Hello world\\r\\n");
        assert_eq!(Message::new(message), Err(String::from("Invalid integer!")));
    }
}
