use std::collections::HashMap;

use crate::resp_data::RESPData;

pub struct DB {
    db: HashMap<String, RESPData>,
}

impl DB {
    pub fn new() -> Self {
        DB { db: HashMap::new() }
    }

    pub fn proccess_message(&mut self, message: RESPData) -> RESPData {
        match message {
            RESPData::Array(array) => {
                let mut iter = array.into_iter();
                match iter.next() {
                    Some(command) => match command {
                        RESPData::BulkString(command) => match command.to_lowercase().as_str() {
                            "ping" => DB::handle_ping_echo(command.as_str(), iter.collect::<Vec<RESPData>>()),
                            "echo" => DB::handle_ping_echo(command.as_str(), iter.collect::<Vec<RESPData>>()),
                            "set" => DB::handle_ping(iter.collect::<Vec<RESPData>>()),
                            "get" => DB::handle_ping(iter.collect::<Vec<RESPData>>()),
                            "exists" => DB::handle_ping(iter.collect::<Vec<RESPData>>()),
                            "del" => DB::handle_ping(iter.collect::<Vec<RESPData>>()),
                            "incr" => DB::handle_ping(iter.collect::<Vec<RESPData>>()),
                            "decr" => DB::handle_ping(iter.collect::<Vec<RESPData>>()),
                            "lpush" => DB::handle_ping(iter.collect::<Vec<RESPData>>()),
                            "rpush" => DB::handle_ping(iter.collect::<Vec<RESPData>>()),
                            "save" => DB::handle_ping(iter.collect::<Vec<RESPData>>()),
                            _ => RESPData::Error(format!("Invalid command {}!", command)),
                        },
                        _ => RESPData::Error(String::from("Invalid RESP message!")),
                    },
                    None => RESPData::Error(String::from("Invalid RESP message!")),
                }
            }
            _ => RESPData::Error(String::from("Invalid RESP message!")),
        }
    }

    fn handle_ping_echo(command: &str, args: Vec<RESPData>) -> RESPData {
        if args.len() == 0 {
            RESPData::SimpleString(String::from("PONG"))
        } else if args.len() == 1 {
            args[0].copy()
        } else {
            RESPData::Error(format!(
                "ERR wrong number of arguments for '{}' command",
                command
            ))
        }
    }
    fn handle_ping(args: Vec<RESPData>) -> RESPData {
        if args.len() == 0 {
            RESPData::SimpleString(String::from("PONG"))
        } else if args.len() == 1 {
            RESPData::BulkString(format!("{}", args[0]))
        } else {
            RESPData::Error(String::from(
                "ERR wrong number of arguments for 'ping' command",
            ))
        }
    }
}
// - `PING`: Checks if connection is established.
// - `ECHO`: Similar to `PING`.
// - `SET`: Sets a value to a key.
// - `GET`: Returns a the value of a given key.
// - `EXISTS`: Check if a key is present.
// - `DEL`: Delete one or more keys.
// - `INCR`: Increment a stored number by one.
// - `DECR`: Decrement a stored number by one.
// - `LPUSH`: Insert all the values at the head of a list.
// - `RPUSH`: Insert all the values at the tail of a list.
// - `SAVE`: Save the database state to disk, you should also implement load on startup alongside this.
