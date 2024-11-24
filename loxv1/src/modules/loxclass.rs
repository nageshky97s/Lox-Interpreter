use std::{fmt,rc::Rc,cell::RefCell };
use super::{loxcallable,loxinstance, loxfunction,token};
use std::collections::HashMap;

#[derive(Debug,Clone)]

pub struct LoxClass{
    pub name:String,
    pub methods: HashMap<String,loxfunction::LoxFunction>,
    pub super_class: Option<Box<LoxClass>>,

}

impl LoxClass {
    pub fn new(name_:String,methods: HashMap<String, loxfunction::LoxFunction>,super_class: Option<LoxClass>,)->Self {
        LoxClass { name: name_,methods:methods,super_class:super_class.map(Box::new), }
    }
    pub fn find_method(&self,name:String) -> Option<&loxfunction::LoxFunction> {
        if self.methods.contains_key(&name){
            return self.methods.get(&name);
        }
        if let Some(sc) = &self.super_class {
            return sc.find_method(name);
        }
        None
    }
}


impl fmt::Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Class", self.name)
    }
}

impl loxcallable::LoxCallable for LoxClass {
    fn call( &self,
            _interpreter: &mut super::interpreter::Interpreter,
            _arguments: &[super::token::Literals],)->Result<super::token::Literals,super::interpreter::Exit> {
        let instance = Rc::new(RefCell::new(loxinstance::LoxInstance::new(Rc::new(self.clone()))));
        return Ok(token::Literals::Callable(loxcallable::Callable::Instance(Rc::clone(&instance,))));
    }
    fn arity(&self)->usize {
        return 0;
    }
}