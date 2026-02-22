use crate::token_type::TokenType;
use std::fmt;

#[derive(Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

#[derive(Clone)]
pub enum Literal {
    Number(f64),
    Str(String),
    Bool(bool),
    None,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Str(s) => write!(f, "{}", s),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::None => write!(f, "nil"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {:?} {}",
            self.token_type, self.lexeme, self.literal
        )
    }
}
