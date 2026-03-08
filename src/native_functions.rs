use crate::callable::Callable;
use crate::interpreter::Interpreter;
use crate::literal::Literal;

#[derive(Debug)]
pub struct Two;

impl Callable for Two {
    fn arity(&self) -> usize {
        0
    }
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<crate::literal::Literal>,
    ) -> crate::literal::Literal {
        Literal::Number(2.0)
    }
}
