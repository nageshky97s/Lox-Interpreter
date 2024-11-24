
use std::collections::HashMap;
use super::{expr::{self, Accept}, interpreter, lox, parser::{self, ParseError}, stmt::{self, Stmt, StmtAccept}, token};


#[derive(Clone, Copy, PartialEq)]
pub enum FunctionType {
    None,
    Function,
    // Initializer,
    Method,
}

pub struct Resolver<'a>{
    pub interpreter:&'a mut interpreter::Interpreter,
    pub lox_obj:&'a mut lox::Lox,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,

}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut interpreter::Interpreter,lox_obj:&'a mut lox::Lox) -> Self {
        Resolver {
            interpreter,
            lox_obj,
            scopes: Vec::new(),
            current_function: FunctionType::None,
        }
    }
    fn begin_scope(&mut self,){
        self.scopes.push(HashMap::new());
    }
   pub fn resolve(&mut self,statements:&[stmt::Stmt])-> Result<(), parser::ParseError> {
        for statement in statements.iter(){
            self.resolve_stmt(statement)?;
        }
        Ok(())
    }
    fn resolve_stmt(&mut self,statement:&Stmt)-> Result<(), parser::ParseError>{
        statement.accept(self)?;
        Ok(())
    }
    fn resolve_expr(&mut self,expression: &expr::Expr){
        let _=expression.accept(self);
    }
    fn endscope(&mut self,){
        self.scopes.pop();
    }
    fn declare(&mut self,name: token::Token,)-> Result<(), parser::ParseError>{
        if !self.scopes.is_empty() {
            if self.scopes.last().unwrap().contains_key(&name.lexeme) {
                self.lox_obj.errorp(name, "Already a variable with this name in this scope.".to_string());
                return Err(parser::ParseError);
                
            }
            self.scopes.last_mut().unwrap().insert(name.lexeme, false);
        }

        Ok(())
    }
    fn define(&mut self,name :token::Token,){
        if !self.scopes.is_empty() {
            self.scopes.last_mut().unwrap().insert(name.lexeme, true);
        }
    }
    fn resolve_local(&mut self,exp:expr::Expr,name:token::Token){

        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(exp.clone(), self.scopes.len() - 1 - i);
            }
        }
    }
    fn resolve_function(&mut self,function:&stmt::Function,typ:FunctionType)-> Result<(),parser::ParseError>{
        let enclosing_function=self.current_function;
        self.current_function=typ;
        self.begin_scope();
        for param in function.params.iter(){
            self.declare(param.clone())?;
            self.define(param.clone());
        }
        self.resolve(&function.body)?;
        self.endscope();
        self.current_function=enclosing_function;
        Ok(())

    }
}

impl<'a> stmt::StmtVisitor<Result<(),parser::ParseError>> for Resolver<'a> {
    fn visit_block_stmt(&mut self, visitor: &stmt::Block) -> Result<(),parser::ParseError> {
        self.begin_scope();
        self.resolve(&visitor.statements)?;
        self.endscope();
        Ok(())
    }
    fn visit_var_stmt(&mut self, visitor: &stmt::Var) -> Result<(),parser::ParseError> {
        self.declare(visitor.name.clone())?;
        self.resolve_expr(&visitor.initializer);
        self.define(visitor.name.clone());
        Ok(())
    }
    fn visit_expression_stmt(&mut self, visitor: &stmt::Expression) -> Result<(),parser::ParseError> {
        self.resolve_expr(&visitor.expression);
        Ok(())
    }
    fn visit_function_stmt(&mut self, visitor: &stmt::Function) -> Result<(),parser::ParseError> {
        self.declare(visitor.name.clone())?;
        self.define(visitor.name.clone());
        self.resolve_function(visitor, FunctionType::Function)?;
        Ok(())
    }
    fn visit_if_stmt(&mut self, visitor: &stmt::If) -> Result<(),parser::ParseError> {
        self.resolve_expr(&visitor.condition);
        self.resolve_stmt(&visitor.then_branch)?;
        if visitor.else_branch.is_some(){
           
            self.resolve_stmt(&(*visitor.else_branch.clone()).unwrap())?;
        }
        Ok(())
    }
    fn visit_print_stmt(&mut self, visitor: &stmt::Print) -> Result<(),parser::ParseError> {
        self.resolve_expr(&visitor.expression);
        Ok(())
    }
    fn visit_return_stmt(&mut self, visitor: &stmt::Return) -> Result<(),parser::ParseError> {
        if self.current_function==FunctionType::None{
            self.lox_obj.errorp(visitor.keyword.clone(), "Can't return from top-level code.".to_string());
            return Err(ParseError);

        }
        if visitor.value!=None{
            self.resolve_expr(visitor.value.as_ref().unwrap());
        }
        Ok(())
    }
    fn visit_while_stmt(&mut self, visitor: &stmt::While) -> Result<(),parser::ParseError> {
        self.resolve_expr(&visitor.condition);
        self.resolve_stmt(&visitor.body)?;
        Ok(())
    }

    fn visit_class_stmt(&mut self, visitor: &stmt::Class) -> Result<(),parser::ParseError> {
        self.declare(visitor.name.clone())?;
        self.begin_scope();
        self.scopes.last_mut().unwrap().insert("this".to_string(), true);
        for method in visitor.methods.iter() 
        {
            let declaration = FunctionType::Method;
            if let Stmt::Function(m) = method {
                               
                self.resolve_function(m, declaration)?;
            }
        }

        
        self.define(visitor.name.clone());
        self.endscope();
        Ok(())
    }
}
impl<'a> expr::AstVisitor<Result<(),parser::ParseError>> for Resolver<'a>  {

    fn visit_this(&mut self, visitor: &expr::This) -> Result<(),parser::ParseError> {
        self.resolve_local(expr::Expr::This(visitor.clone()), visitor.keyword.clone());
        Ok(())
    }

    fn visit_set(&mut self, visitor: &expr::Set) -> Result<(),parser::ParseError> {
        self.resolve_expr(&visitor.value);
        self.resolve_expr(&visitor.object);
        Ok(())
    }
    fn visit_get(&mut self, visitor: &expr::Get) -> Result<(),parser::ParseError> {
        self.resolve_expr(&visitor.object);
        Ok(())
    }
    fn visit_assign(&mut self, visitor: &expr::Assign) -> Result<(),parser::ParseError> {
        self.resolve_expr(&visitor.value);
        self.resolve_local(expr::Expr::Assign(expr::Assign { uuid:visitor.uuid.clone(), name: visitor.name.clone(), value: visitor.value.clone() }), visitor.name.clone());
        Ok(())
    }
    fn visit_binary(&mut self, visitor: &expr::Binary) -> Result<(),parser::ParseError> {
        self.resolve_expr(&visitor.left);
        self.resolve_expr(&visitor.right);
        Ok(())
    }
    fn visit_call(&mut self, visitor: &expr::Call) -> Result<(),parser::ParseError> {
        self.resolve_expr(&visitor.callee);
        for exp in visitor.arguments.iter(){
            self.resolve_expr(&exp);
        }
        Ok(())
    }
    fn visit_grouping(&mut self, visitor: &expr::Grouping) -> Result<(),parser::ParseError> {
        self.resolve_expr(&visitor.expression);
        Ok(())
    }
    fn visit_literal(&mut self, _visitor: &expr::Literal) -> Result<(),parser::ParseError> {
        Ok(())
    }
    fn visit_logical(&mut self, visitor: &expr::Logical) -> Result<(),parser::ParseError> {
        self.resolve_expr(&visitor.left);
        self.resolve_expr(&visitor.right);
        Ok(())
    }
    fn visit_unary(&mut self, visitor: &expr::Unary) -> Result<(),parser::ParseError> {
        self.resolve_expr(&visitor.right);
        Ok(())
    }
    fn visit_variable(&mut self, visitor: &expr::Variable) -> Result<(),parser::ParseError> {
        if !self.scopes.is_empty() &&
        self.scopes.last().unwrap().get(&visitor.name.lexeme)==Some(&false){
            self.lox_obj.errorp(visitor.name.clone(), "Can't read local variable in its own initializer.".to_string());
            return Err(parser::ParseError);
        }
        self.resolve_local(expr::Expr::Variable(expr::Variable {uuid:visitor.uuid.clone(), name: visitor.name.clone() })   ,visitor.name.clone());
        Ok(())
    }
}
