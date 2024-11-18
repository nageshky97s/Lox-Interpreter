use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::{token,interpreter};

#[derive(Debug, Clone, Default)]
pub struct Environment {
pub values: HashMap<String, token::Literals>,
pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
    Default::default()
}

pub fn new_env(enclosing: Rc<RefCell<Environment>>) -> Self {
    Environment {
        values: HashMap::new(),
        enclosing: Some(enclosing),
    }
}

pub fn define(&mut self, name: String, value: token::Literals) {
    self.values.insert(name, value);
}

pub fn get(&self, name: &token::Token) ->token::Literals {
    if self.values.contains_key(&name.lexeme) {
       return self.values.get(&name.lexeme).unwrap().clone()
        } else if self.enclosing.is_some() {
           return self.enclosing.as_ref().unwrap().borrow().get(name)
        } else {
            std::panic::panic_any(interpreter::RuntimeError{tok:name.clone(),mess:"Undefined variable ".to_string()+&name.lexeme+&" .".to_string()});
    }
}

pub fn assign(&mut self, name: &token::Token, value: token::Literals){
    if self.values.contains_key(&name.lexeme) {
        self.values.insert(name.lexeme.clone(), value);
        
        } 
    else if let Some(enclosing) = &self.enclosing {
        enclosing.borrow_mut().assign(name, value);
        
    } else {
        std::panic::panic_any(interpreter::RuntimeError{tok:name.clone(),mess:"Undefined variable ".to_string()+&name.lexeme+&" .".to_string()});
        }
    }
}
