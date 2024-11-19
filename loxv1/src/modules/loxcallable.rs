use super::{interpreter, loxfunction::LoxFunction, token};
use std::fmt;

pub enum Callable{
    Function(LoxFunction),
}
pub trait LoxCallable{
     fn call( &self,
        interpreter: &mut interpreter::Interpreter,
        arguments: &[token::Literals],)->Result<token::Literals,interpreter::Exit>;
   fn arity(&self)->usize;
}
impl fmt::Debug for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Callable")
    }
}
impl fmt::Display for Callable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Function(_x)=>write!(f,"Function - Callable"),
        }
    }
}

impl Clone for Callable {
    fn clone(&self) -> Self {
        match self {
            Callable::Function(lox_function) => Callable::Function(lox_function.clone()),
            // Callable::Class(class) => Callable::Class(class.clone()),
            // Callable::Instance(ins) => Callable::Instance(ins.clone()),
        }
    }
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}