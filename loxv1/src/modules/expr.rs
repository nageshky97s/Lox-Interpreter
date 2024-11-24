use std::hash::Hash;

use crate::modules::token;

#[derive(Debug,Clone)]
pub enum Expr {
    //Empty,
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Variable(Variable),
    Assign(Assign),
    Logical(Logical),
    Call(Call),
    Get(Get),
    Set(Set),
    This(This),
    Super(Super),
}
pub type ExprBox = Box<Expr>;


#[derive(Debug,PartialEq,Clone)]
pub struct Super{
    pub uuid:usize,
    pub keyword:token::Token,
    pub method:token::Token,
}
#[derive(Debug,PartialEq,Clone)]
pub struct This{
    pub uuid:usize,
    pub keyword:token::Token
}

#[derive(Debug,PartialEq,Clone)]
pub struct Set{
    pub uuid:usize,
    pub object:ExprBox,
    pub name:token::Token,
    pub value:ExprBox,
}
#[derive(Debug,PartialEq,Clone)]
pub struct Get{
    pub uuid:usize,
    pub object: ExprBox,
    pub name:token::Token,
}

#[derive(Debug,PartialEq,Clone)]

pub struct Call{
    pub uuid: usize,
    pub callee:ExprBox,
    pub paren:token::Token,
    pub arguments:Vec<ExprBox>,
}



#[derive(Debug,PartialEq,Clone)]
pub struct Logical{
    pub uuid: usize,
    pub left:ExprBox,
    pub operator:token::Token,
    pub right:ExprBox,
}



#[derive(Debug,PartialEq,Clone)]
pub struct Assign{
    pub uuid: usize,
    pub name: token::Token,
    pub value:ExprBox,
}


#[derive(Debug,PartialEq,Clone)]
pub struct Binary {
    pub uuid: usize,
    pub left: ExprBox,
    pub operator: token::Token,
    pub right: ExprBox,
}
#[derive(Debug,PartialEq,Clone)]
pub struct Grouping {
    pub uuid: usize,
    pub expression: ExprBox,
}
#[derive(Debug,Clone,PartialEq)]
pub struct Literal {
    pub uuid: usize,
    pub value: token::Literals,
}
#[derive(Debug,PartialEq,Clone)]
pub struct Unary {
    pub uuid: usize,
    pub operator: token::Token,
    pub right: ExprBox,
}
#[derive(Debug,Clone,PartialEq)]
pub struct Variable{
    pub uuid: usize,
    pub name:token::Token,
}

pub trait AstVisitor<R> {
    fn visit_binary(&mut self, visitor: &Binary) -> R;
    fn visit_grouping(&mut self, visitor: &Grouping) -> R;
    fn visit_literal(&mut self, visitor: &Literal) -> R;
    fn visit_unary(&mut self, visitor: &Unary) -> R;
    fn visit_variable(&mut self, visitor: &Variable) -> R;
    fn visit_assign(&mut self, visitor: &Assign) -> R;
    fn visit_logical(&mut self, visitor: &Logical) -> R;
    fn visit_call(&mut self, visitor: &Call) -> R;
    fn visit_get(&mut self, visitor: &Get) -> R;
    fn visit_set(&mut self, visitor: &Set) -> R;
    fn visit_this(&mut self, visitor: &This) -> R;
    fn visit_super(&mut self, visitor: &Super) -> R;

}
pub trait Accept<R> {
    fn accept<V: AstVisitor<R>>(&self, visitor: &mut V) -> R;
}
impl<R> Accept<R> for Expr {
    fn accept<V: AstVisitor<R>>(&self, visitor: &mut V) -> R {
        match self {
            // Expr::Empty => {
            //     panic!("Cannot visit empty");
            // }
            Expr::Binary(x) => visitor.visit_binary(x),
            Expr::Grouping(x) => visitor.visit_grouping(x),
            Expr::Literal(x) => visitor.visit_literal(x),
            Expr::Unary(x) => visitor.visit_unary(x),
            Expr::Variable(x) => visitor.visit_variable(x),
            Expr::Assign(x)=>visitor.visit_assign(x),
            Expr::Logical(x)=>visitor.visit_logical(x),
            Expr::Call(x)=>visitor.visit_call(x),
            Expr::Get(x)=>visitor.visit_get(x),
            Expr::Set(x)=>visitor.visit_set(x),
            Expr::This(x)=>visitor.visit_this(x),
            Expr::Super(x)=>visitor.visit_super(x),


        }
    }
    
}
impl Expr {
    fn get_uid(&self) -> usize {
        match self {
            Expr::Assign(e) => e.uuid,
            Expr::Binary(e) => e.uuid,
            Expr::Grouping(e) => e.uuid,
            Expr::Literal(e) => e.uuid,
            Expr::Logical(e) => e.uuid,
            Expr::Unary(e) => e.uuid,
            Expr::Variable(e) => e.uuid,
            Expr::Call(e) => e.uuid,
            Expr::Get(e) => e.uuid,
            Expr::Set(e) => e.uuid,
            Expr::This(e) => e.uuid,
            Expr::Super(e) => e.uuid,
        }
    }
}
impl<R> Accept<R> for Assign {
    fn accept<V: AstVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_assign(self)
    }
}

impl<R> Accept<R> for Binary {
    fn accept<V: AstVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_binary(self)
    }
}
impl<R> Accept<R> for Call {
    fn accept<V: AstVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_call(self)
    }
}
impl<R> Accept<R> for Grouping {
    fn accept<V: AstVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_grouping(self)
    }
}
impl<R> Accept<R> for Literal {
    fn accept<V: AstVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_literal(self)
    }
}
impl<R> Accept<R> for Logical {
    fn accept<V: AstVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_logical(self)
    }
}
impl<R> Accept<R> for Unary {
    fn accept<V: AstVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_unary(self)
    }
}
impl<R> Accept<R> for Variable {
    fn accept<V: AstVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_variable(self)
    }
}
impl<R> Accept<R> for Get {
    fn accept<V: AstVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_get(self)
    }
}
impl<R> Accept<R> for Set {
    fn accept<V: AstVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_set(self)
    }
}
impl<R> Accept<R> for This {
    fn accept<V: AstVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_this(self)
    }
}
impl<R> Accept<R> for Super {
    fn accept<V: AstVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_super(self)
    }
}
impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.get_uid() == other.get_uid()
    }
}

impl Eq for Expr {}

impl Hash for Expr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // core::mem::discriminant(self).hash(state);
        self.get_uid().hash(state);
    }
}