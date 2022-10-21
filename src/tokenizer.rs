pub enum RedisTokens {
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
    CONSTANT(String),
}

pub fn tokenize(line: String) -> Vec<RedisTokens> {
    let mut tokens: Vec<RedisTokens> = Vec::default();

    // todo: when there is a string like "kristoff finley" >> in " "
    let mut split = line.split(" ");

    for word in split.into_iter() {
        match word {
            "SET" => { tokens.push(RedisTokens::SET) },
            "GET" => { tokens.push(RedisTokens::GET) },
            "DEL" => { tokens.push(RedisTokens::DEL) },
            _ => { todo!() },
        }
    };

    return tokens;
}