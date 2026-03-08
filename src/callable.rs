use std::fmt;

use crate::interpreter::Interpreter;
use crate::literal::Literal;

pub trait Callable: fmt::Debug {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Literal>) -> Literal;
}
