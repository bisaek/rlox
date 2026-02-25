use std::collections::HashMap;

use crate::{literal::Literal, token::Token};

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Literal>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }
    pub fn get(&self, name: Token) -> Literal {
        let value = self.values.get(&name.lexeme);
        return match (value, self.enclosing.as_deref()) {
            (Some(v), _) => v.clone(),
            (None, Some(e)) => e.get(name),
            (None, None) => panic!("Undefined variable '{}'.", name.lexeme),
        };
    }
    pub fn assign(&mut self, name: Token, value: Literal) {
        match (
            self.values.contains_key(&name.lexeme),
            self.enclosing.as_deref_mut(),
        ) {
            (true, _) => {
                self.values.insert(name.lexeme, value);
            }
            (false, Some(e)) => {
                e.assign(name, value);
            }
            (false, None) => panic!("{} Undifined variable '{}'.", name, name.lexeme),
        };
    }
}
