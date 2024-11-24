use crate::modules::expr;
use super::token;


#[derive(PartialEq,Clone,Debug)]
pub enum Stmt{
     Expression(Expression),
     Print(Print),
     Var(Var),
     Block(Block),
     If(If),
     While(While),
     Function(Function),
     Return(Return),
     Class(Class),
}
#[derive(PartialEq,Clone,Debug)]
pub struct Class{
    pub name:token::Token,
    pub methods:Vec<Stmt>,
    pub super_class: Option<expr::Expr>,

}
#[derive(PartialEq,Clone,Debug)]
pub struct Return{
    pub keyword:token::Token,
    pub value:Option<expr::Expr>,
}
#[derive(PartialEq,Clone,Debug)]
pub struct Function{
    pub name :token::Token,
    pub params :Vec<token::Token>,
    pub body:Vec<Stmt>,
}
#[derive(PartialEq,Clone,Debug)]
pub struct While{
    pub condition :expr::Expr,
    pub body:Box<Stmt>,
}#[derive(PartialEq,Clone,Debug)]
pub struct If{
    pub condition :expr::Expr,
    pub then_branch :Box<Stmt>,
    pub else_branch :Box<Option<Stmt>>,
}#[derive(PartialEq,Clone,Debug)]
pub struct Block{
    pub statements:Vec<Stmt>,
}#[derive(PartialEq,Clone,Debug)]
pub  struct Expression{
    pub expression :expr::Expr
}#[derive(PartialEq,Clone,Debug)]
pub struct Print{
    pub expression :expr::Expr
}#[derive(PartialEq,Clone,Debug)]
pub struct Var{
    pub name : token::Token,
    pub initializer :expr::Expr
}
pub trait StmtVisitor<R> {
    fn visit_expression_stmt(&mut self, visitor: &Expression) -> R;
    fn visit_print_stmt(&mut self, visitor: &Print) -> R;
    fn visit_var_stmt(&mut self, visitor: &Var) -> R;
    fn visit_block_stmt(&mut self, visitor: &Block) -> R;
    fn visit_if_stmt(&mut self, visitor: &If) -> R;
    fn visit_while_stmt(&mut self, visitor: &While) -> R;
    fn visit_function_stmt(&mut self, visitor: &Function) -> R;
    fn visit_return_stmt(&mut self, visitor: &Return) -> R;
    fn visit_class_stmt(&mut self, visitor: &Class) -> R;
}
pub trait StmtAccept<R> {
    fn accept<V: StmtVisitor<R>>(&self, visitor: &mut V) -> R;
}
impl<R> StmtAccept<R> for Stmt {
    fn accept<V: StmtVisitor<R>>(&self, visitor: &mut V) -> R {
        match self {
            
            Stmt::Expression(x) => visitor.visit_expression_stmt(x),
            Stmt::Print(x) => visitor.visit_print_stmt(x),
            Stmt::Var(x) => visitor.visit_var_stmt(x),
            Stmt::Block(x)=>visitor.visit_block_stmt(x),
            Stmt::If(x)=>visitor.visit_if_stmt(x),
            Stmt::While(x)=>visitor.visit_while_stmt(x),
            Stmt::Function(x)=>visitor.visit_function_stmt(x),
            Stmt::Return(x)=>visitor.visit_return_stmt(x),
            Stmt::Class(x)=>visitor.visit_class_stmt(x),
        }
    }
}

impl<R> StmtAccept<R> for Expression {
    fn accept<V: StmtVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_expression_stmt(self)
    }
}
impl<R> StmtAccept<R> for Print {
    fn accept<V: StmtVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_print_stmt(self)
    }
}

impl<R> StmtAccept<R> for Var {
    fn accept<V: StmtVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_var_stmt(self)
    }
}
impl<R> StmtAccept<R> for Block {
    fn accept<V: StmtVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_block_stmt(self)
    }
}

impl<R> StmtAccept<R> for If {
    fn accept<V: StmtVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_if_stmt(self)
    }
}
impl<R> StmtAccept<R> for While {
    fn accept<V: StmtVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_while_stmt(self)
    }
}
impl<R> StmtAccept<R> for Function {
    fn accept<V: StmtVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_function_stmt(self)
    }
}

impl<R> StmtAccept<R> for Return {
    fn accept<V: StmtVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_return_stmt(self)
    }
}


impl<R> StmtAccept<R> for Class {
    fn accept<V: StmtVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_class_stmt(self)
    }
}