use crate::modules::token;
#[derive(Debug)]
pub enum Expr {
    //Empty,
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Box<Literal>),
    Unary(Box<Unary>),
}
pub type ExprBox = Box<Expr>;

#[derive(Debug)]
pub struct Binary {
    pub left: ExprBox,
    pub operator: token::Token,
    pub right: ExprBox,
}
#[derive(Debug)]
pub struct Grouping {
    pub expression: ExprBox,
}
#[derive(Debug,Clone)]
pub struct Literal {
    pub value: token::Literals,
}
#[derive(Debug)]
pub struct Unary {
    pub operator: token::Token,
    pub right: ExprBox,
}

pub trait AstVisitor<R> {
    fn visit_binary(&mut self, visitor: &Binary) -> R;
    fn visit_grouping(&mut self, visitor: &Grouping) -> R;
    fn visit_literal(&mut self, visitor: &Literal) -> R;
    fn visit_unary(&mut self, visitor: &Unary) -> R;
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
        }
    }
}
impl<R> Accept<R> for Binary {
    fn accept<V: AstVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_binary(self)
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
impl<R> Accept<R> for Unary {
    fn accept<V: AstVisitor<R>>(&self, visitor: &mut V) -> R {
        visitor.visit_unary(self)
    }
}