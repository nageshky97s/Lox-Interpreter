use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::{interpreter::{self, RuntimeError}, token};

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

pub fn get(&self, name: &token::Token) ->Result<token::Literals,interpreter::Exit> {
    if self.values.contains_key(&name.lexeme) {
       return Ok(self.values.get(&name.lexeme).unwrap().clone());
        } else if self.enclosing.is_some() {
           return self.enclosing.as_ref().unwrap().borrow().get(name)
        } else {
            return Err(interpreter::Exit::RuntimeErr(RuntimeError{tok:name.clone(),mess:"Undefined variable ".to_string()+&name.lexeme+&" .".to_string()}));
           
    }
}

pub fn get_at(&self,distance:usize,name: &token::Token)->Result<token::Literals,interpreter::Exit>{
    if distance  == 0 {
        self.get(&name)
    } else {
        self.enclosing
            .as_ref()
            .unwrap()
            .borrow()
            .get_at(distance - 1, name)
    }
}

pub fn assign(&mut self, name: &token::Token, value: token::Literals)->Result<(),interpreter::Exit>{
    if self.values.contains_key(&name.lexeme) {
        self.values.insert(name.lexeme.clone(), value);
        return Ok(());
        
        } 
    else if let Some(enclosing) = &self.enclosing {
        enclosing.borrow_mut().assign(name, value)?;
        return Ok(());
        
    } else {
        return Err(interpreter::Exit::RuntimeErr(RuntimeError{tok:name.clone(),mess:"Undefined variable ".to_string()+&name.lexeme+&" .".to_string()}));
        
        }
    }
    pub fn assign_at(&mut self, distance: usize, name: token::Token, value: token::Literals) {
        if distance == 0 {
            self.define(name.lexeme, value);
        } else {
            self.enclosing
                .as_ref()
                .unwrap()
                .borrow_mut()
                .assign_at(distance - 1, name, value);
        }
    }
}
