use std::collections::HashMap;
use super::{token,interpreter};
pub struct Environment{
    pub values:HashMap<String,token::Literals>
}

impl Environment {
    pub fn new()->Self{
        Environment{values:HashMap::new()}
    }

    pub fn define(&mut self,name:String,value:token::Literals){
        self.values.insert(name,value);
    }

    pub fn get(&mut self,name:token::Token)->token::Literals{
        if self.values.contains_key(&name.lexeme){
            return self.values.get(&name.lexeme).unwrap().clone();
        }
        std::panic::panic_any(interpreter::RuntimeError{tok:name.clone(),mess:"Undefined variable".to_string()+&name.lexeme+&" .".to_string()});

    }
    pub fn assign(&mut self,name:token::Token,value:token::Literals){
        if self.values.contains_key(&name.lexeme){
            self.values.insert(name.lexeme,value);
            return;
        }
        std::panic::panic_any(interpreter::RuntimeError{tok:name.clone(),mess:"Undefined variable".to_string()+&name.lexeme+&" .".to_string()});

    }
    
}