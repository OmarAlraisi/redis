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

        match Message::new(message_input) {
            Err(err) => {
                println!("{}", err);
                let new_string = format!("-{}", err);
                stream.write(new_string.as_bytes()).unwrap();
            }
            Ok(message) => match message {
                Message::BulkString(message) => {
                    println!("{}", message);
                    stream.write(message.as_bytes()).unwrap();
                }
                _ => {}
            },
        }

        stream.write(b"+PONG\r\n").unwrap();

        stream.shutdown(Shutdown::Both).unwrap();
    }

    Ok(())
}
