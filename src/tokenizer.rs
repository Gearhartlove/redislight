#[derive(PartialEq, Debug)]
pub enum RedisToken {
    SET,
    GET,
    DEL,
    LPUSH,
    LPOP,
    LRANGE,
    HSET,
    HGET,
    EX,
    PX,
    EXAT,
    PXAT,
    NX,
    XX,
    KEEPTTL,
    LITERAL(String),
    EOC, // End of Command
}

pub struct Tokenizer {
    source: String,
    start: usize, 
    current: usize,
    tokens: Vec<RedisToken>,
}

impl Tokenizer {
    pub fn new(source: impl ToString) -> Self {
        Tokenizer {
            source: source.to_string(),
            start: 0,
            current: 0,
            tokens: vec!(),
        }
    }

    pub fn tokenize(&mut self) {
        loop {
            self.skip_whitespace();

            self.start = self.current;

            if self.is_at_end() {
                self.add_token(RedisToken::EOC);
                break;
            }

            let c = self.get_start_char();

            if is_alpha(&c) {
                let token = self.tokenize_identifier();
                self.add_token(token);
            }

            if is_digit(&c) {
                let token = self.tokenize_number();
                self.add_token(token);
            }
        }

        // todo: when there is a string like "kristoff finley" >> in " "

        //for word in self.tokens.iter() {
        //    match word {
        //        "SET" => { tokens.push(RedisToken::SET) },
        //        "GET" => { tokens.push(RedisToken::GET) },
        //        "DEL" => { tokens.push(RedisToken::DEL) },
        //        _ => { tokens.push(RedisToken::LITERAL(word.to_string())) },
        //    }
        //};

    }

    fn add_token(&mut self, token: RedisToken) {
        self.tokens.push(token);
    }

    fn tokenize_identifier(&mut self) -> RedisToken {
        while let Some(peek) = self.peek() {
            if is_alpha(&peek) || is_digit(&peek) {
                self.advance();
            } else {
                break
            }
        }
        let token = self.identifer_kind();
        return token;
    }

    fn tokenize_number(&mut self) -> RedisToken {
        while let Some(peek) = self.peek() {
            if !is_digit(&peek) {
                break;
            } 
            self.advance();
        }

        // Look for a decimal part to the number
        if let Some(peek) = self.peek() {
            if let Some(peek_next) = self.peek_next() {
                if peek == '.' && is_digit(peek_next) {
                    self.advance();

                    // Keep consuming numbers
                    while let Some(peek) = self.peek() {
                        if !is_digit(peek) {
                            break;
                        }
                        self.advance();
                    }
                }
            }
        }
        self
    }

    // ###############################################################
    // tokenizer helper functions
    // ###############################################################

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                _ => return,
            }
        }
    }

    fn is_at_end(&self) -> bool {
        if self.start >= self.source.len() {
            return true
        } else {
            return false
        }
    }

    fn get_start_char(&self) -> char {
        let i: usize = self.start;
        self.source.chars().nth(i).unwrap()
    }

    fn peek(&self) -> Option<char> {
        let i: usize = self.current + 1;
        self.source.chars().nth(i)
    }

    fn advance(&mut self) {
        self.current += 1;
    }
}

// ###############################################################
// Utility Tokenizer Functions
// ###############################################################

fn is_alpha(c: &char) -> bool {
    ('a'..='z').contains(&c)
    || ('A'..='Z').contains(&c)
    || *c == '_'
}
fn is_digit(c: &char) -> bool {
    ('0'..='9').contains(&c)
}


// ###############################################################
// tokenizer unit tests 
// ###############################################################

#[macro_export]
macro_rules! assert_tokens_are {
    ($left_side_tokens:expr, $( $r:expr ),*) => {
        {
            // add right side arguments to a vector
            let mut right_side_tokens = Vec::new();
            $(
            right_side_tokens.push($r);
            )*

            assert_eq!($left_side_tokens, right_side_tokens);
        }
    };
}


mod tests {
    use crate::tokenizer::Tokenizer;
    use crate::tokenizer::RedisToken::*;

    #[test]
    fn tokenize_test() {
        let input = String::from("GET mykey");
        let mut tokenizer = Tokenizer::new(input);
        tokenizer.tokenize(input);
        assert_tokens_are!(tokenizer.tokens, GET, LITERAL("mykey".to_string()));


        let input = String::from("SET myname \"kristoff finley\"");
        let mut tokenizer = Tokenizer::new(input);
        tokenizer.tokenize(input);
        assert_tokens_are!(tokenizer.tokens, SET, LITERAL("kristoff finley".to_string()));
    }
}
