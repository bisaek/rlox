use crate::callable::Callable;
use crate::literal::Literal;
use crate::stmt::Stmt;
use crate::token::Token;

#[derive(Debug)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Box<Stmt>>,
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.params.len()
    }
    fn call(
        &self,
        interpreter: &mut crate::interpreter::Interpreter,
        arguments: Vec<Literal>,
    ) -> crate::literal::Literal {
        let mut environment = interpreter.globals.clone();
        for (param, argument) in self.params.iter().zip(arguments) {
            environment.define(param.lexeme.clone(), argument);
        }

        interpreter.execute_block(self.body.clone(), environment);

        Literal::None
    }
}
