use crate::{expr::Expr, literal::Literal, token::Token, token_type::TokenType};

pub fn interpret(expression: Expr) {
    let value = evaluate(Box::new(expression));
    println!("{}", value);
}

fn evaluate(expr: Box<Expr>) -> Literal {
    match *expr {
        Expr::Grouping { expresstion } => evaluate(expresstion),
        Expr::Literal { value } => value,
        Expr::Unary { operator, right } => {
            let right = evaluate(right);

            match operator.token_type {
                TokenType::Bang => !right,
                TokenType::Minus => -right,
                _ => Literal::None,
            }
        }
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let left = evaluate(left);
            let right = evaluate(right);

            match operator.token_type {
                TokenType::Minus => left - right,
                TokenType::Slash => left / right,
                TokenType::Star => left * right,
                TokenType::Plus => left + right,
                _ => Literal::None,
            }
        }
    }
}
