mod db;
mod resp;
mod resp_data;

use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};

use db::DB;
use resp::RESP;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;
    let db = Arc::new(Mutex::new(DB::new()));

    for stream in listener.incoming() {
        let mut stream = stream?;
        let db = Arc::clone(&db);

        thread::spawn(move || loop {
            let mut buffer = [0; 1024 * 1024];
            let num_of_bytes = match stream.read(&mut buffer) {
                Ok(num_of_bytes) => num_of_bytes,
                Err(_) => break,
            };
            let input_message = RESP::deserialize(&buffer[..num_of_bytes]);
            let output_message = db.lock().unwrap().proccess_message(input_message);

            match stream.write(&RESP::serialize(output_message)) {
                Ok(_) => {}
                Err(_) => continue,
            };
        });
    }

    Ok(())
}
