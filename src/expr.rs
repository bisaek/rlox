use crate::literal::Literal;
use crate::token::Token;
use std::fmt::{self, write};

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expresstion: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Literal { value } => {
                write!(f, "{}", value)
            }

            Expr::Grouping { expresstion } => {
                write!(f, "(group {})", expresstion)
            }

            Expr::Unary { operator, right } => {
                // bruger Token::Display her 👇
                write!(f, "({} {})", operator, right)
            }

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                // Token formatteres via Display
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expr::Variable { name } => write!(f, "(var {})", name),
        }
    }
}
