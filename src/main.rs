mod db;
mod resp;
mod resp_data;

use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener},
};

use db::DB;
use resp::RESP;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;
    let mut db = DB::new();

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = [0; 1024 * 1024];
        let num_of_bytes = stream.read(&mut buffer).unwrap();
        let input_message = RESP::deserialize(&buffer[..num_of_bytes]);
        let output_message = db.proccess_message(input_message);

        stream.write(&RESP::serialize(output_message)).unwrap();

        stream.shutdown(Shutdown::Both).unwrap();
    }

    Ok(())
}
