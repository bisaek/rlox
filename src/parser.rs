use std::ops::Mul;

use crate::expr::Expr;
use crate::literal::Literal;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration());
        }

        statements
    }
    fn declaration(&mut self) -> Stmt {
        if self.match_token(vec![TokenType::Var]) {
            return self.var_declaration();
        }
        self.statement()
    }
    fn var_declaration(&mut self) -> Stmt {
        let name = self.consume(TokenType::Identifier, "Expect variable name");

        let mut initializer = Box::new(Expr::Literal {
            value: Literal::None,
        });
        if self.match_token(vec![TokenType::Equal]) {
            initializer = Box::new(self.expresstion());
        }
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration",
        );

        Stmt::Var { name, initializer }
    }
    fn statement(&mut self) -> Stmt {
        if self.match_token(vec![TokenType::For]) {
            return self.for_statement();
        }
        if self.match_token(vec![TokenType::If]) {
            return self.if_statement();
        }
        if self.match_token(vec![TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_token(vec![TokenType::While]) {
            return self.while_statement();
        }
        if self.match_token(vec![TokenType::LeftBrace]) {
            return Stmt::Block {
                statements: self.block(),
            };
        }
        self.expression_statement()
    }
    fn for_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.");

        let mut initializer: Option<Stmt> = None;
        if self.match_token(vec![TokenType::Var]) {
            initializer = Some(self.var_declaration());
        } else if !self.match_token(vec![TokenType::Semicolon]) {
            initializer = Some(self.expression_statement());
        }

        let mut condition: Expr = Expr::Literal {
            value: Literal::Bool(true),
        };
        if !self.check(TokenType::Semicolon) {
            condition = self.expresstion();
        }
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition");

        let mut increment: Option<Expr> = None;
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expresstion());
        }
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.");
        let mut body = self.statement();

        increment.map(|increment| {
            body = Stmt::Block {
                statements: vec![
                    Box::new(body.clone()),
                    Box::new(Stmt::Expression {
                        expression: Box::new(increment),
                    }),
                ],
            };
        });

        body = Stmt::While {
            condition: Box::new(condition),
            body: Box::new(body),
        };

        initializer.map(|initializer| {
            body = Stmt::Block {
                statements: vec![Box::new(initializer), Box::new(body.clone())],
            }
        });

        body
    }
    fn while_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'");
        let condition = Box::new(self.expresstion());
        self.consume(TokenType::RightParen, "Expect ')' after condition");
        let body = Box::new(self.statement());

        Stmt::While { condition, body }
    }
    fn if_statement(&mut self) -> Stmt {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
        let condition = Box::new(self.expresstion());
        self.consume(TokenType::RightParen, "Expect ')' after if condition");

        let then_branch = Box::new(self.statement());
        let else_branch = match self.match_token(vec![TokenType::Else]) {
            true => Some(Box::new(self.statement())),
            false => None,
        };

        Stmt::If {
            condition,
            then_branch,
            else_branch,
        }
    }

    fn print_statement(&mut self) -> Stmt {
        let value = self.expresstion();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        Stmt::Print {
            expression: Box::new(value),
        }
    }
    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expresstion();
        self.consume(TokenType::Semicolon, "Expect ';' after expression");
        Stmt::Expression {
            expression: Box::new(expr),
        }
    }
    fn block(&mut self) -> Vec<Box<Stmt>> {
        let mut statements: Vec<Box<Stmt>> = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(Box::new(self.declaration()));
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block");
        statements
    }
    fn assignment(&mut self) -> Expr {
        let expr = self.or();

        if self.match_token(vec![TokenType::Equal]) {
            let equals = self.previous();
            let value = Box::new(self.assignment());

            match expr {
                Expr::Variable { name } => return Expr::Assign { name, value },
                _ => panic!("{} Invalid assignment targer", equals),
            }
        }
        expr
    }
    fn or(&mut self) -> Expr {
        let mut expr = self.and();

        while self.match_token(vec![TokenType::Or]) {
            let operator = self.previous();
            let right = Box::new(self.and());
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right,
            }
        }

        expr
    }
    fn and(&mut self) -> Expr {
        let mut expr = self.equality();

        while self.match_token(vec![TokenType::And]) {
            let operator = self.previous();
            let right = Box::new(self.equality());
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right,
            }
        }

        expr
    }

    fn expresstion(&mut self) -> Expr {
        self.assignment()
    }
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        expr
    }
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_token(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }

        self.call()
    }
    fn call(&mut self) -> Expr {
        let mut expr = self.primary();

        loop {
            if self.match_token(vec![TokenType::LeftParen]) {
                expr = self.finish_call(Box::new(expr));
            } else {
                break;
            }
        }
        expr
    }
    fn finish_call(&mut self, calle: Box<Expr>) -> Expr {
        let mut arguments: Vec<Box<Expr>> = vec![];
        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    panic!("Can't have more than 255 arguments.")
                }
                arguments.push(Box::new(self.expresstion()));
                if !self.match_token(vec![TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.");
        Expr::Call {
            callee: calle,
            paren,
            arguments,
        }
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::False]) {
            return Expr::Literal {
                value: Literal::Bool(false),
            };
        }
        if self.match_token(vec![TokenType::True]) {
            return Expr::Literal {
                value: Literal::Bool(true),
            };
        }
        if self.match_token(vec![TokenType::Nil]) {
            return Expr::Literal {
                value: Literal::None,
            };
        }

        if self.match_token(vec![TokenType::Number, TokenType::String]) {
            return Expr::Literal {
                value: self.previous().literal,
            };
        }

        if self.match_token(vec![TokenType::Identifier]) {
            return Expr::Variable {
                name: self.previous(),
            };
        }

        if self.match_token(vec![TokenType::LeftParen]) {
            let expr = self.expresstion();
            self.consume(TokenType::RightParen, "Expect ')' after expression");
            return Expr::Grouping {
                expresstion: Box::new(expr),
            };
        }

        panic!("paaaaaaaaaaaaaaaaaaaanic: {}", self.peek());
    }

    fn match_token(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }
    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check(token_type) {
            return self.advance();
        }

        panic!("{}: {}", self.peek(), message);
    }
    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1
        }
        self.previous()
    }
    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
    fn peek(&mut self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }
    fn previous(&mut self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }
}
