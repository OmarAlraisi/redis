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
                            "ping" => DB::handle_ping_echo(
                                command.as_str(),
                                iter.collect::<Vec<RESPData>>(),
                            ),
                            "echo" => DB::handle_ping_echo(
                                command.as_str(),
                                iter.collect::<Vec<RESPData>>(),
                            ),
                            "set" => self.handle_set(iter.collect::<Vec<RESPData>>()),
                            "get" => self.handle_get(iter.collect::<Vec<RESPData>>()),
                            "exists" => self.handle_exists(iter.collect::<Vec<RESPData>>()),
                            "del" => self.handle_del(iter.collect::<Vec<RESPData>>()),
                            "incr" => self.handle_incr(iter.collect::<Vec<RESPData>>()),
                            "decr" => self.handle_decr(iter.collect::<Vec<RESPData>>()),
                            "lpush" => self.handle_lpush(iter.collect::<Vec<RESPData>>()),
                            "rpush" => self.handle_rpush(iter.collect::<Vec<RESPData>>()),
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

    fn handle_set(&mut self, args: Vec<RESPData>) -> RESPData {
        if args.len() != 2 {
            RESPData::Error(String::from("ERR syntax error"))
        } else {
            let key = match &args[0] {
                RESPData::BulkString(key) => key.to_owned(),
                _ => return RESPData::Error(String::from("ERR syntax error")),
            };
            let val = args[1].copy();

            self.db.insert(key, val);
            RESPData::SimpleString(String::from("OK"))
        }
    }

    fn handle_get(&self, args: Vec<RESPData>) -> RESPData {
        if args.len() != 1 {
            RESPData::Error(String::from(
                "ERR wrong number of arguments for 'GET' command",
            ))
        } else {
            let key = match &args[0] {
                RESPData::BulkString(key) => key.to_owned(),
                _ => return RESPData::Error(String::from("ERR syntax error")),
            };

            match self.db.get(&key) {
                Some(val) => val.copy(),
                None => RESPData::Null,
            }
        }
    }

    fn handle_exists(&self, args: Vec<RESPData>) -> RESPData {
        if args.len() == 0 {
            return RESPData::Error(String::from(
                "ERR wrong number of arguments for 'EXISTS' command",
            ));
        }

        let mut keys_exist = 0;
        for key in args {
            match key {
                RESPData::BulkString(key) => {
                    if self.db.get(&key).is_some() {
                        keys_exist += 1;
                    }
                }
                _ => return RESPData::Error(String::from("ERR syntax error.")),
            }
        }

        RESPData::Integer(keys_exist)
    }

    fn handle_del(&mut self, args: Vec<RESPData>) -> RESPData {
        if args.len() == 0 {
            return RESPData::Error(String::from(
                "ERR wrong number of arguments for 'DEL' command",
            ));
        }

        let mut deleted_keys = 0;
        for key in args {
            match key {
                RESPData::BulkString(key) => {
                    if self.db.remove(&key).is_some() {
                        deleted_keys += 1;
                    }
                }
                _ => return RESPData::Error(String::from("ERR syntax error.")),
            }
        }

        RESPData::Integer(deleted_keys)
    }

    fn handle_incr(&mut self, args: Vec<RESPData>) -> RESPData {
        if args.len() != 1 {
            return RESPData::Error(String::from(
                "ERR wrong number of arguments for 'INCR' command",
            ));
        }

        let key = if let Some(RESPData::BulkString(key)) = args.get(0) {
            key.clone()
        } else {
            String::new()
        };

        let new_val = if let Some(val) = self.db.get(&key) {
            let val = match val.copy() {
                RESPData::BulkString(val) => val,
                _ => return RESPData::Error(String::from("Invalid message!")),
            };

            match val.parse::<i32>() {
                Ok(val) => val + 1,
                Err(_) => {
                    return RESPData::Error(String::from(
                        "ERR value is not an integer or out of range",
                    ))
                }
            }
        } else {
            1
        };

        self.db
            .insert(key, RESPData::BulkString(format!("{}", new_val)));
        RESPData::Integer(new_val)
    }

    fn handle_decr(&mut self, args: Vec<RESPData>) -> RESPData {
        if args.len() != 1 {
            return RESPData::Error(String::from(
                "ERR wrong number of arguments for 'INCR' command",
            ));
        }

        let key = if let Some(RESPData::BulkString(key)) = args.get(0) {
            key.clone()
        } else {
            String::new()
        };

        let new_val = if let Some(val) = self.db.get(&key) {
            let val = match val.copy() {
                RESPData::BulkString(val) => val,
                _ => return RESPData::Error(String::from("Invalid message!")),
            };

            match val.parse::<i32>() {
                Ok(val) => val - 1,
                Err(_) => {
                    return RESPData::Error(String::from(
                        "ERR value is not an integer or out of range",
                    ))
                }
            }
        } else {
            -1
        };

        self.db
            .insert(key, RESPData::BulkString(format!("{}", new_val)));
        RESPData::Integer(new_val)
    }

    fn handle_lpush(&mut self, args: Vec<RESPData>) -> RESPData {
        if args.len() < 2 {
            return RESPData::Error(String::from(
                "ERR wrong number of arguments for 'LPUSH' command",
            ));
        }

        let mut iter = args.into_iter();

        let key = match iter.next().unwrap() {
            RESPData::BulkString(key) => key,
            _ => return RESPData::Error(String::from("Invalid message!")),
        };

        let mut arr = iter.collect::<Vec<RESPData>>();
        arr.reverse();

        if let Some(RESPData::Array(array)) = self.db.get_mut(&key) {
            for item in arr {
                array.insert(0, item);
            }
        } else {
            self.db.insert(key, RESPData::Array(arr));
        }

        RESPData::SimpleString(String::from("OK"))
    }

    fn handle_rpush(&mut self, args: Vec<RESPData>) -> RESPData {
        if args.len() < 2 {
            return RESPData::Error(String::from(
                "ERR wrong number of arguments for 'LPUSH' command",
            ));
        }

        let mut iter = args.into_iter();

        let key = match iter.next().unwrap() {
            RESPData::BulkString(key) => key,
            _ => return RESPData::Error(String::from("Invalid message!")),
        };

        if let Some(RESPData::Array(array)) = self.db.get_mut(&key) {
            array.append(&mut iter.collect::<Vec<RESPData>>());
        } else {
            self.db
                .insert(key, RESPData::Array(iter.collect::<Vec<RESPData>>()));
        }

        RESPData::SimpleString(String::from("OK"))
    }
}
