use std::fmt;

#[derive(Debug,Clone)]

pub struct LoxClass{
    pub name:String,

}

impl LoxClass {
    pub fn new(name_:String)->Self {
        LoxClass { name: name_ }
    }
}


impl fmt::Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Class", self.name)
    }
}