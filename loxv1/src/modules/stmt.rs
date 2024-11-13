use crate::modules::expr;
use super::token;

pub enum Stmt{
     Expression(Expression),
     Print(Print),
     Var(Var),
}
pub  struct Expression{
    pub expression :expr::Expr
}
pub struct Print{
    pub expression :expr::Expr
}
pub struct Var{
    pub name : token::Token,
    pub initializer :expr::Expr
}
pub trait StmtVisitor<R> {
    fn visit_expression_stmt(&mut self, visitor: &Expression) -> R;
    fn visit_print_stmt(&mut self, visitor: &Print) -> R;
    fn visit_var_stmt(&mut self, visitor: &Var) -> R;
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