mod commands;
mod scanner;

use std::io::{stdin, stdout, Write};
use crate::scanner::{TokenKind, Scanner, Token};

/// Reads the user input from the command line and tokenizes that input.
pub fn read() -> Vec<RedisToken> {
    let mut line = String::new();

    print!(">> ");
    let _ = stdout().flush(); // Flushes the input buffer. Used for formatting.
    stdin().read_line(&mut line).unwrap();

    let mut scanner = Scanner::from(&line);
    let mut tokens: Vec<Token> = Vec::default();

    loop {
        let token = scanner.scan_token();
        scanner.advance();

        tokens.push(token);
    }


    return tokens;
}

pub fn parse() {

}

pub fn eval(tokens: Vec<RedisToken>) -> String {
    for token in tokens.iter() {
        match token {
            RedisToken::LITERAL(_) => {}
            RedisToken::SET => {}
            RedisToken::GET => {}
            RedisToken::DEL => {}
            RedisToken::LPUSH => {}
            RedisToken::LPOP => {}
            RedisToken::LRANGE => {}
            RedisToken::HSET => {}
            RedisToken::HGET => {}
            RedisToken::EX => {}
            RedisToken::PX => {}
            RedisToken::EXAT => {}
            RedisToken::PXAT => {}
            RedisToken::NX => {}
            RedisToken::XX => {}
            RedisToken::KEEPTTL => {}
        }
    }
    "".to_string()
}

pub fn print(result: String) {

}
