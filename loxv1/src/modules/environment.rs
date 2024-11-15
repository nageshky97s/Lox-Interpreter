use std::collections::HashMap;
use super::{token,interpreter};

#[derive(Debug,Clone,PartialEq)]
pub struct Environment{
    
    pub env_values:Vec<HashMap<String,token::Literals>>,
}

impl Environment {
    pub fn new()->Self{
        Environment{env_values:vec![HashMap::new()]}
    }

    pub fn new_env(&mut self){
        self.env_values.push(HashMap::new());
    }
    pub fn define(&mut self,name:String,value:token::Literals){
        self.env_values.last_mut().unwrap().insert(name, value);
    }

    pub fn getval(&mut self,name:&token::Token)->token::Literals{
        
        for map in self.env_values.iter_mut().rev(){
            if map.contains_key(&name.lexeme){
                return map.get(&name.lexeme).unwrap().clone();
            }
        }      
        std::panic::panic_any(interpreter::RuntimeError{tok:name.clone(),mess:"Undefined variable ".to_string()+&name.lexeme+&" .".to_string()});

    }
    pub fn assign(&mut self,name:&token::Token,value:token::Literals){
        
        for map in self.env_values.iter_mut().rev(){
            if map.contains_key(&name.lexeme){
                map.insert(name.lexeme.clone(),value);
                return;
            }
        }                
        std::panic::panic_any(interpreter::RuntimeError{tok:name.clone(),mess:"Undefined variable ".to_string()+&name.lexeme+&" .".to_string()});

    }
    
}