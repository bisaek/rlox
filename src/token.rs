use crate::token_type::TokenType;
use std::fmt;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: u64,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: u64) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

pub enum Literal {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}
