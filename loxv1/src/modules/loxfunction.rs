use super::{interpreter, token,environment,stmt,loxcallable};
use std::borrow::BorrowMut;
use std::{fmt,rc::Rc};
use std::panic::AssertUnwindSafe;
use std::panic;

pub fn catch_unwind_silent<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) -> std::thread::Result<R> {
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(f);
    panic::set_hook(prev_hook);
    result
}

#[derive(Clone)]
pub struct LoxFunction{
    pub declaration:Box<stmt::Function>,
}
impl LoxFunction {
    pub fn new(declaration: stmt::Function,)->Self{
        LoxFunction {
            declaration: Box::new(declaration),
        }
    }

}

impl loxcallable::LoxCallable for LoxFunction {

     fn call( &self,
            interpreter: &mut interpreter::Interpreter,
            arguments: &[token::Literals],)->token::Literals {
        let mut environment=environment::Environment::new_env(Rc::clone(&interpreter.globals));
        for i in 0..self.declaration.params.len(){
            environment.borrow_mut().define(self.declaration.params.get(i).unwrap().lexeme.clone(), arguments.get(i).unwrap().clone());
        }
       
        let res = catch_unwind_silent(AssertUnwindSafe(|| {
            interpreter.execute_block(&self.declaration.body,environment);
        }));
        
        match res {
            Ok(_x)=>{
                return token::Literals::Nil;
            } 
            Err(payload)if payload.is::<interpreter::ReturnVal>()=>{
                
                let ret:interpreter::ReturnVal=*payload.downcast().expect("The value must be of type ReturnVal");
                return ret.value;
               
            },
            Err(payload) => std::panic::resume_unwind(payload),
            
        }
        
               
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