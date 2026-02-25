use crate::{expr::Expr, token::Token};

pub enum Stmt {
    Expression {
        expression: Box<Expr>,
    },
    Print {
        expression: Box<Expr>,
    },
    Var {
        name: Token,
        initializer: Box<Expr>,
    },
    Block {
        statements: Vec<Box<Stmt>>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
}
