use super::{interpreter, token,environment,stmt,loxcallable};
use std::borrow::BorrowMut;
use std::{fmt,rc::Rc};
use std::cell::RefCell;


#[derive(Clone)]
pub struct LoxFunction{
    pub declaration:Box<stmt::Function>,
    pub closure: Rc<RefCell<environment::Environment>>,
}
impl LoxFunction {
    pub fn new(declaration: stmt::Function,
        closure: Rc<RefCell<environment::Environment>>,)->Self{
        LoxFunction {
            declaration: Box::new(declaration),
            closure:closure,
        }
    }

}

impl loxcallable::LoxCallable for LoxFunction {

     fn call( &self,interpreter: &mut interpreter::Interpreter,arguments: &[token::Literals],)->Result<token::Literals,interpreter::Exit> {
        let mut environment=environment::Environment::new_env(Rc::clone(&self.closure));
        for i in 0..self.declaration.params.len(){
            environment.borrow_mut().define(self.declaration.params.get(i).unwrap().lexeme.clone(), arguments.get(i).unwrap().clone());
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