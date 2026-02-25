use crate::{
    environment::Environment, expr::Expr, literal::Literal, stmt::Stmt, token::Token,
    token_type::TokenType,
};

pub fn interpret(statements: Vec<Stmt>) {
    let mut interpreter = Interpreter {
        enviroment: Environment::new(),
    };
    for statement in statements {
        interpreter.execute(Box::new(statement));
    }
}

struct Interpreter {
    enviroment: Environment,
}

impl Interpreter {
    fn evaluate(&mut self, expr: Box<Expr>) -> Literal {
        match *expr {
            Expr::Grouping { expresstion } => self.evaluate(expresstion),
            Expr::Literal { value } => value,
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right);

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
                let left = self.evaluate(left);
                let right = self.evaluate(right);

                match operator.token_type {
                    TokenType::Greater => left.greater(right),
                    TokenType::GreaterEqual => left.greater_or_equal(right),
                    TokenType::Less => left.less(right),
                    TokenType::LessEqual => left.less_or_equal(right),
                    TokenType::BangEqual => Literal::Bool(left != right),
                    TokenType::EqualEqual => Literal::Bool(left == right),
                    TokenType::Minus => left - right,
                    TokenType::Slash => left / right,
                    TokenType::Star => left * right,
                    TokenType::Plus => left + right,
                    _ => Literal::None,
                }
            }
            Expr::Variable { name } => self.enviroment.get(name),
        }
    }

    fn execute(&mut self, stmt: Box<Stmt>) {
        match *stmt {
            Stmt::Print { expression } => {
                let value = self.evaluate(expression);
                println!("{}", value);
            }
            Stmt::Expression { expression } => {
                self.evaluate(expression);
            }
            Stmt::Var { name, initializer } => {
                let value = self.evaluate(initializer);
                self.enviroment.define(name.lexeme, value);
            }
        }
    }
}
