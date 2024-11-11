use crate::modules::expr;
pub enum Stmt{
     Expression(Expression),
     Print(Print),
     
}
pub  struct Expression{
    pub expression :expr::Expr
}
pub struct Print{
    pub expression :expr::Expr
}
pub trait StmtVisitor<R> {
    fn visit_expression_stmt(&mut self, visitor: &Expression) -> R;
    fn visit_print_stmt(&mut self, visitor: &Print) -> R;
}
pub trait StmtAccept<R> {
    fn accept<V: StmtVisitor<R>>(&self, visitor: &mut V) -> R;
}
impl<R> StmtAccept<R> for Stmt {
    fn accept<V: StmtVisitor<R>>(&self, visitor: &mut V) -> R {
        match self {
            
            Stmt::Expression(x) => visitor.visit_expression_stmt(x),
            Stmt::Print(x) => visitor.visit_print_stmt(x),
            
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

