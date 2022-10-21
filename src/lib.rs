mod tokenizer;
mod commands;

use std::io::{stdin, stdout, Write};
use crate::tokenizer::{RedisTokens, tokenize};

/// Reads the user input from the command line and tokenizes that input.
pub fn read() -> Vec<RedisTokens> {
    let mut line = String::new();

    print!(">> ");
    let _ = stdout().flush(); // Flushes the input buffer. Used for formatting.
    stdin().read_line(&mut line).unwrap();

    let tokens = tokenize(line);
    return tokens;
}

pub fn parse() {

}

pub fn eval(tokens: Vec<RedisTokens>) -> String {
    for token in tokens.iter() {
        match token {
            RedisTokens::CONSTANT(_) => {}
            RedisTokens::SET => {}
            RedisTokens::GET => {}
            RedisTokens::DEL => {}
            RedisTokens::LPUSH => {}
            RedisTokens::LPOP => {}
            RedisTokens::LRANGE => {}
            RedisTokens::HSET => {}
            RedisTokens::HGET => {}
            RedisTokens::EX => {}
            RedisTokens::PX => {}
            RedisTokens::EXAT => {}
            RedisTokens::PXAT => {}
            RedisTokens::NX => {}
            RedisTokens::XX => {}
            RedisTokens::KEEPTTL => {}
        }
    }
    "".to_string()
}

pub fn print() {

}
