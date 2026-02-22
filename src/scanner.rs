use crate::token_type::TokenType;
use crate::{token::*, token_type};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while (!self.is_at_end()) {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_owned(),
            Literal::None,
            self.line,
        ));
        return self.tokens.to_vec();
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token_type = if self.match_token('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.match_token('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_token('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_token('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type);
            }
            '/' => {
                if (self.match_token('/')) {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            '0'..'9' => self.number(),
            other => panic!("unexpected char: {:?}", other),
        }
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            println!("{:?}", self.peek());
            self.advance();
        }
        //for i in '0'..'9' {
        //    println!("{:?}", i);
        //}
        println!("{:?} {:?}", self.peek(), self.peek_next());
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        self.add_token_with_literal(
            TokenType::Number,
            Literal::Number(
                self.source[self.start..self.current]
                    .parse::<f64>()
                    .unwrap(),
            ),
        );
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            panic!("{} Unterminated string", self.line);
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_with_literal(TokenType::String, Literal::Str(value));
    }

    fn advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current] as char;
        self.current += 1;
        return c;
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.as_bytes()[self.current] as char != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.as_bytes()[self.current] as char;
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.as_bytes()[self.current + 1] as char;
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, Literal::None)
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) {
        let text: String = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
