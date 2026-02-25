use std::collections::HashMap;

use crate::{literal::Literal, token::Token};

pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }
    pub fn get(&self, name: Token) -> Literal {
        let value = self.values.get(&name.lexeme);
        return match value {
            Some(v) => v.clone(),
            None => panic!("Undefined variable '{}'.", name.lexeme),
        };
    }
}
