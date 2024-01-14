mod resp;
mod resp_data;

use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener},
};

use resp::RESP;
use resp_data::RESPData;

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

        let message_output = RESP::deserialize(message_input);
        stream
            .write(format!("{}", message_output).as_bytes())
            .unwrap();

        stream.shutdown(Shutdown::Both).unwrap();
    }

    Ok(())
}
