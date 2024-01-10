# redis
This is a simple Rust implementation of Redis

## Messages

Accepts an array of bulk strings and returns a RESP message of the following types:

- Simple String: Prefixed with `+` and terminated with `\r\n`.
- Error: Prefixed with `-` and terminated with `\r\n`.
- Integers: Prefixed with `:` and terminated with `\r\n`.
- Bulk Stirng: Prefixed with `$`, followed by bytes size, followed by `\r\n`, followed by the string, and terminated with `\r\n`.
- Arrays: Prefixed with `*`, followed by array size, and followed by it's elements (any of the above types).

## Available Commands

- `PING`: Checks if connection is established.
- `ECHO`: Similar to `PING`.
- `SET`: Sets a value to a key.
- `GET`: Returns a the value of a given key.
- `EXISTS`: Check if a key is present.
- `DEL`: Delete one or more keys.
- `INCR`: Increment a stored number by one.
- `DECR`: Decrement a stored number by one.
- `LPUSH`: Insert all the values at the head of a list.
- `RPUSH`: Insert all the values at the tail of a list.
- `SAVE`: Save the database state to disk, you should also implement load on startup alongside this.
