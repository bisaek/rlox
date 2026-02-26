use crate::{
    environment::Environment, expr::Expr, literal::Literal, stmt::Stmt, token::Token,
    token_type::TokenType,
};

pub fn interpret(statements: Vec<Stmt>) {
    let mut interpreter = Interpreter {
        enviroment: Environment::new(None),
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
            Expr::Assign { name, value } => {
                let value = self.evaluate(value);
                self.enviroment.assign(name, value.clone());
                value
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left);

                if operator.token_type == TokenType::Or {
                    if left.clone().is_truthy() {
                        return left;
                    }
                } else {
                    if !left.clone().is_truthy() {
                        return left;
                    }
                }

                self.evaluate(right)
            }
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
            Stmt::Block { statements } => {
                self.execute_block(
                    statements,
                    Environment::new(Some(Box::new(self.enviroment.clone()))),
                );
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => match (self.evaluate(condition), else_branch) {
                (Literal::Bool(true), _) => self.execute(then_branch),
                (Literal::Bool(false), Some(else_branch)) => self.execute(else_branch),
                _ => panic!("test"),
            },
        }
    }
    fn execute_block(&mut self, statements: Vec<Box<Stmt>>, environment: Environment) {
        let previous = self.enviroment.clone();
        self.enviroment = environment;

        for statement in statements {
            self.execute(statement);
        }
        self.enviroment = previous;
    }
}
