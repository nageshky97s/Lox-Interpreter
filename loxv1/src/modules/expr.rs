use crate::modules::token;
#[derive(Debug,PartialEq)]
pub enum Expr {
    //Empty,
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Box<Literal>),
    Unary(Box<Unary>),
    Variable(Box<Variable>),
    Assign(Box<Assign>),
    Logical(Box<Logical>)
}
pub type ExprBox = Box<Expr>;

#[derive(Debug,PartialEq)]
pub struct Logical{
    pub left:ExprBox,
    pub operator:token::Token,
    pub right:ExprBox,
}



#[derive(Debug,PartialEq)]
pub struct Assign{
    pub name: token::Token,
    pub value:ExprBox,
}


#[derive(Debug,PartialEq)]
pub struct Binary {
    pub left: ExprBox,
    pub operator: token::Token,
    pub right: ExprBox,
}
#[derive(Debug,PartialEq)]
pub struct Grouping {
    pub expression: ExprBox,
}
#[derive(Debug,Clone,PartialEq)]
pub struct Literal {
    pub value: token::Literals,
}
#[derive(Debug,PartialEq)]
pub struct Unary {
    pub operator: token::Token,
    pub right: ExprBox,
}
#[derive(Debug,Clone,PartialEq)]
pub struct Variable{
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