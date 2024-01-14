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

        let simple_string = RESPData::SimpleString(String::from("Simple String"));
        let error = RESPData::Error(String::from("Error"));
        let integer = RESPData::Integer(100);
        let bulk_string = RESPData::BulkString(String::from("Bulk Stirng"));
        let array = RESPData::Array(vec![
            RESPData::SimpleString(String::from("Element 1")),
            RESPData::SimpleString(String::from("Element 2")),
        ]);
        stream
            .write(format!("{}", array).as_bytes())
            .unwrap();

        stream.shutdown(Shutdown::Both).unwrap();
    }

    Ok(())
}
