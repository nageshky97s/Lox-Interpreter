use super::{interpreter, token,environment,stmt,loxcallable,loxinstance};
use std::{fmt,rc::Rc};
use std::cell::RefCell;


#[derive(Debug,Clone)]
pub struct LoxFunction{
    pub declaration:Box<stmt::Function>,
    pub closure: Rc<RefCell<environment::Environment>>,
    pub is_initializer: bool,
}
impl LoxFunction {
    pub fn new(declaration: stmt::Function,
        closure: Rc<RefCell<environment::Environment>>, is_initializer: bool,)->Self{
        LoxFunction {
            declaration: Box::new(declaration),
            closure:closure,
            is_initializer:is_initializer,
        }
    }

    pub fn bind(&self, instance: Rc<RefCell<loxinstance::LoxInstance>>) -> LoxFunction {
        let environment = Rc::new(RefCell::new(environment::Environment::new_env(Rc::clone(
            &self.closure,
        ))));
        environment.borrow_mut().define("this".to_string(),
        token::Literals::Callable(loxcallable::Callable::Instance(instance)),);
        
        LoxFunction {
            declaration: self.declaration.clone(),
            closure: environment,
            is_initializer: self.is_initializer,
        }
    }
}



impl loxcallable::LoxCallable for LoxFunction {

     fn call( &self,interpreter: &mut interpreter::Interpreter,arguments: &[token::Literals],)->Result<token::Literals,interpreter::Exit> {
        let mut environment=environment::Environment::new_env(Rc::clone(&self.closure));
        for i in 0..self.declaration.params.len(){
            environment.define(self.declaration.params.get(i).unwrap().lexeme.clone(), arguments.get(i).unwrap().clone());
        }
       
        let res = interpreter.execute_block(&self.declaration.body,environment);
      
        
        match res {
            Ok(_) => (),
            Err(e) => {
                if let interpreter::Exit::Return(r) = e {
                    return Ok(r.value.clone());
                } else {
                    return Err(e);
                }
            }
            
        }
        if self.is_initializer {
            return self.closure.borrow().get_at(
                0,
                &token::Token {
                    token_type: token::TokenType::This,
                    lexeme: "this".to_string(),
                    literal: token::Literals::Nil,
                    line: self.declaration.name.line,
                },
            );
        }
        
        Ok(token::Literals::Nil)     
    }
    fn arity(&self)->usize {
        self.declaration.params.len() as usize
    }
}

impl fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} function", self.declaration.name.lexeme)
    }
}
