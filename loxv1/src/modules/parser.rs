use crate::modules::expr;
use crate::modules::token;
use crate::modules::lox;
use crate::modules::stmt;
use std::panic::AssertUnwindSafe;


pub struct Parser{
    tokens: Vec<token::Token>,
    current:i32,
}

pub struct ParseError;

impl Parser {
   pub fn new(t:Vec<token::Token>)->Self{
        Parser{tokens:t,current:0,}
    }
    fn expression(&mut self,lox_obj:&mut lox::Lox) ->expr::Expr{
        self.assignment(lox_obj)
        
    }
    fn assignment(&mut self, lox_obj:&mut lox::Lox)->expr::Expr{
        let exp=self.equality(lox_obj);
        if self.match_(vec![token::TokenType::Equal]){
            let equals=self.previous();
            let value=self.assignment(lox_obj);
            if let expr::Expr::Variable(name)= exp {
                let name_=(*name).name.clone();
                return expr::Expr::Assign(Box::new(expr::Assign{name:name_,value:Box::new(value)}))
            }
            self.error(equals, "Invalid assignment target.".to_string(), lox_obj)
        }
        exp
    }
    fn equality(&mut self,lox_obj:&mut lox::Lox) ->expr::Expr{
        let mut exp:expr::Expr =self.comparison(lox_obj);
        while self.match_(vec![token::TokenType::BangEqual,token::TokenType::EqualEqual]){
            let  operator:token::Token = self.previous();
            let  right:expr::Expr=self.comparison(lox_obj);
            exp=expr::Expr::Binary(Box::new(expr::Binary{left:Box::new(exp),operator:operator,right:Box::new(right)}));
        }
        exp

    }
    fn comparison(&mut self,lox_obj:&mut lox::Lox) ->expr::Expr{
       let mut exp:expr::Expr = self.term(lox_obj);
        while  self.match_(vec![token::TokenType::Greater,
            token::TokenType::GreaterEqual,
            token::TokenType::Lesser,
            token::TokenType::LesserEqual])
            {
                let  operator:token::Token=self.previous();
                let  right:expr::Expr=self.term(lox_obj);
                exp= expr::Expr::Binary(Box::new(expr::Binary{left:Box::new(exp),operator:operator,right:Box::new(right)}));
            }
            exp
    }
    fn term(&mut self,lox_obj:&mut lox::Lox)->expr::Expr{
        let mut exp:expr::Expr = self.factor(lox_obj);
        while self.match_(vec![token::TokenType::Minus,token::TokenType::Plus]){
            let  operator =self.previous();
            let  right:expr::Expr=self.factor(lox_obj);
            exp= expr::Expr::Binary(Box::new(expr::Binary{left:Box::new(exp),operator:operator,right:Box::new(right)}));
        }
        exp

    }
    fn factor(&mut self,lox_obj:&mut lox::Lox)-> expr::Expr{
        let mut exp:expr::Expr = self.unary(lox_obj);
        while self.match_(vec![token::TokenType::Slash,token::TokenType::Star]) {
            let  operator =self.previous();
            let  right:expr::Expr=self.unary(lox_obj);
            exp= expr::Expr::Binary(Box::new(expr::Binary{left:Box::new(exp),operator:operator,right:Box::new(right)}));
            
        }
        exp

    }
    fn unary(&mut self,lox_obj:&mut lox::Lox)->expr::Expr{
        if self.match_(vec![token::TokenType::Bang,token::TokenType::Minus]){
            let operator =self.previous();
            let right:expr::Expr=self.unary(lox_obj);
            return expr::Expr::Unary(Box::new(expr::Unary{operator:operator,right:Box::new(right)}));
        }
        return self.primary(lox_obj).unwrap()
    }

    fn primary(&mut self,lox_obj:&mut lox::Lox) ->Option<expr::Expr>{
        
        if self.match_(vec![token::TokenType::False]){
        return Some(expr::Expr::Literal(Box::new(expr::Literal{value:token::Literals::StringLit{stringval:"false".to_string()}}))); }
        else if self.match_(vec![token::TokenType::True]){
        return Some(expr::Expr::Literal(Box::new(expr::Literal{value:token::Literals::StringLit{stringval:"true".to_string()}}))); }
        else if self.match_(vec![token::TokenType::Nil]){
        return Some(expr::Expr::Literal(Box::new(expr::Literal{value:token::Literals::Nil}))); }
        else if self.match_(vec![token::TokenType::Number,token::TokenType::String]){
            match self.previous().literal {
                Some(x)=>{
                    return Some(expr::Expr::Literal(Box::new(expr::Literal{value:x})));
                }
                None=>{return None;}
            }
        
        }
        else if self.match_(vec![token::TokenType::Identifier]){
            return Some(expr::Expr::Variable(Box::new(expr::Variable { name: self.previous() })))
        }
        else if self.match_(vec![token::TokenType::LeftParen]){
            let  exp:expr::Expr =self.expression(lox_obj) ;
            self.consume(token::TokenType::RightParen, "Expect ')' after expression".to_string(),lox_obj);
            return Some(expr::Expr::Grouping(Box::new(expr::Grouping{expression:Box::new(exp)})));      
        
        }
        else{
            let peek =self.peek();
            self.error(peek, " Expect expression.".to_string(), lox_obj);
        }
        
        }
    
    
    fn match_(&mut self,types:Vec<token::TokenType>)->bool{
        for typ in  types{
            if self.check(typ){
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&mut self,typ:token::TokenType)->bool{
        if self.is_at_end(){ return false;}
        return self.peek().token_type == typ;
    }
    fn advance(&mut self)->token::Token{
        if !self.is_at_end(){self.current+=1;} 
            return self.previous();
    }
    fn is_at_end(&mut self)->bool{
        return self.peek().token_type == token::TokenType::Eof;
    }
    fn peek(&mut self)->token::Token{
        return self.tokens[self.current as usize].clone();
        
        
    }
    fn previous(&mut self) ->token::Token{
        return self.tokens[(self.current as usize)-1].clone();
    }
    fn consume(&mut self,typ:token::TokenType,message:String,lox_obj:&mut lox::Lox)-> token::Token{
        if self.check(typ){
            return self.advance();
        }
            let peek = self.peek();
            self.error(peek, message, lox_obj);
        
    }
    fn error(&mut self,tok:token::Token,message:String,lox_obj:&mut lox::Lox)->!{
        lox_obj.errorp(tok, message);
        std::panic::panic_any(ParseError);
    }

    fn synchronize(&mut self,){
        self.advance();
        while !self.is_at_end(){
            if self.previous().token_type == token::TokenType::Semicolon{return;}
            let p =self.peek().token_type ;
            if p==token::TokenType::Class||
            p==token::TokenType::Fun||
            p==token::TokenType::Var||
            p==token::TokenType::For||
            p==token::TokenType::If||
            p==token::TokenType::While||
            p==token::TokenType::Print||
            p==token::TokenType::Return{
                return;
            }
            self.advance();
        }

    }

//    pub fn parse(&mut self,lox_obj:&mut lox::Lox)->Option<expr::Expr>{

//         let p = std::panic::catch_unwind(AssertUnwindSafe(|| self.expression(lox_obj)));
//         match p {
//             Ok(x)=>{return Some(x)}
//             Err(payload)if payload.is::<ParseError>()=>{
//                 println!("Parsing Error");
//                 return None
//             },
//             Err(payload) => std::panic::resume_unwind(payload),
//         }
//     }

  pub fn parse_new(&mut self,lox_obj:&mut lox::Lox)->Vec<stmt::Stmt>{
    let mut statements:Vec<stmt::Stmt>= Vec::new();

    while !self.is_at_end() {
        match self.declaration(lox_obj) {
            Some(x)=>{
                statements.push(x);
            }
            None=>{

            }
        }
        
    }

    return statements
  }
  fn declaration(&mut self,lox_obj:&mut lox::Lox)->Option<stmt::Stmt>{

    let p = std::panic::catch_unwind(AssertUnwindSafe(|| {
        if self.match_(vec![token::TokenType::Var]){
            return self.var_declaration(lox_obj);
        }
        return self.statement(lox_obj);

    }));
        match p {
            Ok(_)=>{return None }
            Err(payload)if payload.is::<ParseError>()=>{
                println!("Parsing Error");
                self.synchronize();
                return None
            },
            Err(payload) => std::panic::resume_unwind(payload),
        }
    
  }
  fn var_declaration(&mut self,lox_obj:&mut lox::Lox)->stmt::Stmt{
    let name:token::Token=self.consume(token::TokenType::Identifier, "Expect variable name.".to_string(), lox_obj);
    let mut initializer=expr::Expr::Literal(Box::new(expr::Literal{value:token::Literals::Nil}));
    if self.match_(vec![token::TokenType::Equal]){
        initializer=self.expression(lox_obj);
    }
    self.consume(token::TokenType::Semicolon, "Expect ';' after variable declaration.".to_string(), lox_obj);
    stmt::Stmt::Var(stmt::Var { name: name, initializer: initializer })
  }


  fn statement(&mut self,lox_obj:&mut lox::Lox)->stmt::Stmt {
    if self.match_(vec![token::TokenType::Print]){        
      return self.print_statement(lox_obj);
    }
    
    self.expression_statement(lox_obj)
}
fn print_statement(&mut self,lox_obj:&mut lox::Lox)->stmt::Stmt {
    
    let value:expr::Expr=self.expression(lox_obj);
    self.consume(token::TokenType::Semicolon, "Expect ';' after value.".to_string(), lox_obj);
    stmt::Stmt::Print(stmt::Print{expression:value})
    
        
}
fn expression_statement(&mut self,lox_obj:&mut lox::Lox,)->stmt::Stmt {
    let exp:expr::Expr=self.expression(lox_obj);
    self.consume(token::TokenType::Semicolon, "Expect ';' after value.".to_string(), lox_obj);
    stmt::Stmt::Expression(stmt::Expression{expression:exp})


}


}