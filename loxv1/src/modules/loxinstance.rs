use std::{ fmt, rc::Rc };
use super::{interpreter, loxcallable::Callable, loxclass, token};
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Debug,Clone)]
pub struct LoxInstance{
    pub class: Rc<loxclass::LoxClass>,
    pub fields:HashMap<String, token::Literals>,
}
impl LoxInstance {
    pub fn new(class: Rc<loxclass::LoxClass>)->Self{
        LoxInstance { class: class,fields:HashMap::new() }
    }
    pub fn get(&mut self,name:&token::Token)->Result<token::Literals,interpreter::Exit>{
       
        if self.fields.contains_key(&name.lexeme){
            return Ok(self.fields.get(&name.lexeme).unwrap().clone());
        }else if let Some(method) = self.class.find_method(name.lexeme.clone()) {
           return Ok(token::Literals::Callable(Callable::Function(
            method.bind(Rc::new(RefCell::new(self.to_owned()))),
            )))
        }

        return Err(interpreter::Exit::RuntimeErr(interpreter::RuntimeError { tok: name.clone(), mess:"Undefined property '".to_string()+&name.lexeme+"'."  }))
    }
    pub fn set(&mut self,name:&token::Token,value:&token::Literals){
        self.fields.insert(name.lexeme.clone(), value.clone());
    }
}


impl fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Instance", self.class.name)
    }
}