use crate::literal::Literal;
use crate::token::Token;
use std::fmt::{self, write};

#[derive(Clone, Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Box<Expr>>,
    },
    Grouping {
        expresstion: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
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
            Expr::Assign { name, value } => write!(f, "{} = {}", name, value),
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expr::Call {
                callee,
                paren: _,
                arguments,
            } => {
                write!(
                    f,
                    "{}({:?})",
                    callee,
                    arguments
                        .iter()
                        .map(|arg| format!("{}", arg))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}
