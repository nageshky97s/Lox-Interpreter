use std::collections::HashMap;
use super::{token,interpreter};

#[derive(Debug,Clone,PartialEq)]
pub struct Environment{
    pub values:HashMap<String,token::Literals>,
    pub enclosing:Option<Box<Environment>>,
}

impl Environment {
    pub fn new()->Self{
        Environment{values:HashMap::new(),enclosing:None}
    }

    pub fn new_oop(e:Environment)->Self{
        Environment{values:HashMap::new(),enclosing:Some(Box::new(e))}
    }
    pub fn define(&mut self,name:String,value:token::Literals){
        self.values.insert(name,value);
    }

    pub fn getval(&mut self,name:&token::Token)->token::Literals{
        
        if self.values.contains_key(&name.lexeme){
            return self.values.get(&name.lexeme).unwrap().clone();
        }
        
        if self.enclosing!=None{
            return self.enclosing.as_mut().unwrap().getval(name);
        }
        std::panic::panic_any(interpreter::RuntimeError{tok:name.clone(),mess:"Undefined variable ".to_string()+&name.lexeme+&" .".to_string()});

    }
    pub fn assign(&mut self,name:&token::Token,value:token::Literals){
        
        if self.values.contains_key(&name.lexeme){
            self.values.insert(name.lexeme.clone(),value);
            return;
        }
        if self.enclosing!=None{
            return self.enclosing.as_mut().unwrap().assign(name,value);
        }

        
        std::panic::panic_any(interpreter::RuntimeError{tok:name.clone(),mess:"Undefined variable ".to_string()+&name.lexeme+&" .".to_string()});

    }
    
}