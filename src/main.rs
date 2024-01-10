mod message;

use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener},
};

use message::Message;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = [0; 1024 * 1024];
        let num_of_bytes = stream.read(&mut buffer).unwrap();

        let message_input = match String::from_utf8_lossy(&buffer[..num_of_bytes]) {
            std::borrow::Cow::Borrowed(str) => String::from(str.to_owned()),
            std::borrow::Cow::Owned(str) => str,
        };

        let message_output = match Command::parse_command(message_input) {
            Ok(command) => {
                println!("{}", command.command);
                println!("{:?}", command.args);
                command.command
            },
            Err(err) => err,
        };

        stream
            .write(format!("+{}\r\n", message_output).as_bytes())
            .unwrap();

        stream.shutdown(Shutdown::Both).unwrap();
    }

    Ok(())
}

struct Command {
    command: String,
    args: Vec<Message>,
}

impl Command {
    fn parse_command(raw_message: String) -> Result<Self, String> {
        if let Ok(Message::Array(message)) = Message::new(raw_message) {
            let mut tokens = message.into_iter();
            Ok(Command {
                command: match tokens.next().unwrap() {
                    Message::BulkString(command) => command,
                    _ => return Err(String::from("Invalid message!")),
                },
                args: tokens.collect::<Vec<Message>>(),
            })
        } else {
            Err(String::from("-Invalid\r\n"))
        }
    }
}
