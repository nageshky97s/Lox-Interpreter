use crate::modules::expr;
use crate::modules::token;
use crate::modules::lox;
use crate::modules::stmt;

static mut UUID: usize = 0;

pub fn uuid_next() -> usize {
    unsafe {
        UUID += 1;
        UUID
    }
}


pub struct Parser{
    tokens: Vec<token::Token>,
    current:i32,
}
#[derive(Debug)]
pub struct ParseError;

impl Parser {
   pub fn new(t:Vec<token::Token>)->Self{
        Parser{tokens:t,current:0,}
    }
    fn expression(&mut self,lox_obj:&mut lox::Lox) ->Result<expr::Expr,ParseError>{
        self.assignment(lox_obj)
        
    }
    fn assignment(&mut self, lox_obj:&mut lox::Lox)->Result<expr::Expr,ParseError>{
        let exp=self.or(lox_obj)?;
        if self.match_(vec![token::TokenType::Equal]){
            let equals=self.previous();
            let value=self.assignment(lox_obj)?;
            if let expr::Expr::Variable(name)= exp {
                let name_=name.name.clone();
                return Ok(expr::Expr::Assign(expr::Assign{name:name_,value:Box::new(value),uuid:uuid_next()}))
            }else if let expr::Expr::Get(g) =exp{
                return Ok(expr::Expr::Set(expr::Set{uuid:uuid_next(),object:g.object,name:g.name,value:Box::new(value),}))              
            }
            else{
                self.error(equals, "Invalid assignment target.".to_string(), lox_obj);
                return Err(ParseError);
            }
            
        }
        Ok(exp)
    }
    fn or(&mut self, lox_obj:&mut lox::Lox)->Result<expr::Expr,ParseError>{
        let mut exp=self.and(lox_obj)?;
        while self.match_(vec![token::TokenType::Or]) {
            let operator =self.previous();
            let right = self.and(lox_obj)?;
            exp = expr::Expr::Logical(expr::Logical{uuid:uuid_next(),left:Box::new(exp),operator:operator,right:Box::new(right)});
        }
        Ok(exp)
    }
    fn and(&mut self, lox_obj:&mut lox::Lox)->Result<expr::Expr,ParseError>{
        let mut exp=self.equality(lox_obj)?;
        while self.match_(vec![token::TokenType::And]) {
            let operator =self.previous();
            let right = self.equality(lox_obj)?;
            exp=expr::Expr::Logical(expr::Logical{uuid:uuid_next(),left:Box::new(exp),operator:operator,right:Box::new(right)});
        }
       Ok(exp)

    }
    fn equality(&mut self,lox_obj:&mut lox::Lox) ->Result<expr::Expr,ParseError>{
        let mut exp:expr::Expr =self.comparison(lox_obj)?;
        while self.match_(vec![token::TokenType::BangEqual,token::TokenType::EqualEqual]){
            let  operator:token::Token = self.previous();
            let  right:expr::Expr=self.comparison(lox_obj)?;
            exp=expr::Expr::Binary(expr::Binary{uuid:uuid_next(),left:Box::new(exp),operator:operator,right:Box::new(right)});
        }
        Ok(exp)

    }
    fn comparison(&mut self,lox_obj:&mut lox::Lox) ->Result<expr::Expr,ParseError>{
       let mut exp:expr::Expr = self.term(lox_obj)?;
        while  self.match_(vec![token::TokenType::Greater,
            token::TokenType::GreaterEqual,
            token::TokenType::Lesser,
            token::TokenType::LesserEqual])
            {
                let  operator:token::Token=self.previous();
                let  right:expr::Expr=self.term(lox_obj)?;
                exp= expr::Expr::Binary(expr::Binary{uuid:uuid_next(),left:Box::new(exp),operator:operator,right:Box::new(right)});
            }
            Ok(exp)
    }
    fn term(&mut self,lox_obj:&mut lox::Lox)->Result<expr::Expr,ParseError>{
        let mut exp:expr::Expr = self.factor(lox_obj)?;
        while self.match_(vec![token::TokenType::Minus,token::TokenType::Plus]){
            let  operator =self.previous();
            let  right:expr::Expr=self.factor(lox_obj)?;
            exp= expr::Expr::Binary(expr::Binary{uuid:uuid_next(),left:Box::new(exp),operator:operator,right:Box::new(right)});
        }
        Ok(exp)

    }
    fn factor(&mut self,lox_obj:&mut lox::Lox)-> Result<expr::Expr,ParseError>{
        let mut exp:expr::Expr = self.unary(lox_obj)?;
        while self.match_(vec![token::TokenType::Slash,token::TokenType::Star]) {
            let  operator =self.previous();
            let  right:expr::Expr=self.unary(lox_obj)?;
            exp= expr::Expr::Binary(expr::Binary{uuid:uuid_next(),left:Box::new(exp),operator:operator,right:Box::new(right)});
            
        }
        Ok(exp)

    }
    fn unary(&mut self,lox_obj:&mut lox::Lox)->Result<expr::Expr,ParseError>{
        if self.match_(vec![token::TokenType::Bang,token::TokenType::Minus]){
            let operator =self.previous();
            let right:expr::Expr=self.unary(lox_obj)?;
            return Ok(expr::Expr::Unary(expr::Unary{uuid:uuid_next(),operator:operator,right:Box::new(right)}));
        }

        return self.call(lox_obj);
    }
    fn call(&mut self,lox_obj:&mut lox::Lox)->Result<expr::Expr,ParseError>{

        let mut exp=self.primary(lox_obj)?;
        loop {
            if self.match_(vec![token::TokenType::LeftParen]){
                exp=self.finish_call(exp,lox_obj)?;
            }
            else if self.match_(vec![token::TokenType::Dot]) {
                let name=self.consume(token::TokenType::Identifier, "Expect property name after '.'.".to_string(), lox_obj)?;
                exp=expr::Expr::Get(expr::Get{uuid:uuid_next(),object:Box::new(exp),name:name})
            }
            else{
                break;
            }
        }
        return Ok(exp);
    }

    fn finish_call(&mut self,callee:expr::Expr,lox_obj:&mut lox::Lox)->Result<expr::Expr,ParseError>{

        let mut arguments:Vec<expr::ExprBox>=Vec::new();
        if !self.check(token::TokenType::RightParen){
            loop {
                if arguments.len()>=255{
                    let tok=self.peek();
                    self.error_non_throw(tok, "Can't have more than 255 arguments.".to_string(), lox_obj)
                }
                arguments.push(Box::new(self.expression(lox_obj)?));
                if !self.match_(vec![token::TokenType::Comma]){
                    break;
                }
            }
        }
        let paren=self.consume(token::TokenType::RightParen, "Expect ')' after arguments.".to_string(), lox_obj)?;
        return Ok(expr::Expr::Call(expr::Call { uuid:uuid_next(),callee: Box::new(callee), paren: paren, arguments: arguments }));

    }

    fn primary(&mut self,lox_obj:&mut lox::Lox) ->Result<expr::Expr,ParseError>{
        
        if self.match_(vec![token::TokenType::False]){
        return Ok(expr::Expr::Literal(expr::Literal{uuid:uuid_next(),value:token::Literals::BooleanLit { boolval: false }})); }
        else if self.match_(vec![token::TokenType::True]){
        return Ok(expr::Expr::Literal(expr::Literal{uuid:uuid_next(),value:token::Literals::BooleanLit { boolval: true }})); }
        else if self.match_(vec![token::TokenType::Nil]){
        return Ok(expr::Expr::Literal(expr::Literal{uuid:uuid_next(),value:token::Literals::Nil})); }
        else if self.match_(vec![token::TokenType::Number,token::TokenType::String]){
            return Ok(expr::Expr::Literal(expr::Literal{uuid:uuid_next(),value:self.previous().literal}));                   
        }
        else if self.match_(vec![token::TokenType::Super]){
            let keyword=self.previous();
            self.consume(token::TokenType::Dot, "Expect '.' after 'super'.".to_string(), lox_obj)?;
            let method=self.consume(token::TokenType::Identifier, "Expect superclass method name.".to_string(), lox_obj)?;
            return Ok(expr::Expr::Super(expr::Super { uuid: uuid_next(), keyword: keyword.clone(), method: method.clone() }));
        }
        else if self.match_(vec![token::TokenType::This]) {
            return Ok(expr::Expr::This(expr::This { uuid: uuid_next(), keyword: self.previous() }))
        }
        else if self.match_(vec![token::TokenType::Identifier]){
            return Ok(expr::Expr::Variable(expr::Variable { uuid:uuid_next(),name: self.previous() }))
        }
        else if self.match_(vec![token::TokenType::LeftParen]){
            let  exp:expr::Expr =self.expression(lox_obj)? ;
            self.consume(token::TokenType::RightParen, "Expect ')' after expression".to_string(),lox_obj)?;
            return Ok(expr::Expr::Grouping(expr::Grouping{uuid:uuid_next(),expression:Box::new(exp)}));      
        
        }
        else{
            let peek =self.peek();
            self.error(peek, " Expect expression.".to_string(), lox_obj);
            return Err(ParseError)
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
    fn consume(&mut self,typ:token::TokenType,message:String,lox_obj:&mut lox::Lox)-> Result<token::Token,ParseError>{
        if self.check(typ){
            return Ok(self.advance());
        }
            let peek = self.peek();
            self.error(peek, message, lox_obj);
            return Err(ParseError);
        
    }
    fn error_non_throw(&mut self,tok:token::Token,message:String,lox_obj:&mut lox::Lox){
        lox_obj.errorp(tok, message);
    }
    fn error(&mut self,tok:token::Token,message:String,lox_obj:&mut lox::Lox){
        lox_obj.errorp(tok, message);
        
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

  pub fn parse_new(&mut self,lox_obj:&mut lox::Lox)->Result<Vec<stmt::Stmt>,ParseError>{
    let mut statements:Vec<stmt::Stmt>= Vec::new();
    let mut error = false;
    while !self.is_at_end() {
        let s = self.declaration(lox_obj);
        match &s {
            Ok(_) => statements.push(s.unwrap()),
            Err(_) => error = true,
        }
    }

    if error {
        Err(ParseError)
    } else {
        Ok(statements)
    }
  }
  fn declaration(&mut self,lox_obj:&mut lox::Lox)->Result<stmt::Stmt,ParseError>{

       let p =  if self.match_(vec![token::TokenType::Class]){
             self.class_decalration(lox_obj)
       }
        else if self.match_(vec![token::TokenType::Fun]){
             self.function("function".to_string(),lox_obj)
        }
        else if self.match_(vec![token::TokenType::Var]){
             self.var_declaration(lox_obj)
        }else{
            self.statement(lox_obj)
        };
     
        match &p {
            Ok(_)=>{return p;}
            Err(_)=>{
                println!("Parsing Error");
                self.synchronize();
                return Err(ParseError)
            },
            
        }
    
  }

  fn class_decalration(&mut self,lox_obj:&mut lox::Lox)->Result<stmt::Stmt,ParseError>{
    let name = self.consume(token::TokenType::Identifier, "Expect class name.".to_string(), lox_obj)?;
    let mut superclass=None;
    if self.match_(vec![token::TokenType::Lesser]){
        self.consume(token::TokenType::Identifier, "Expect superclass name.".to_string(), lox_obj)?;
        superclass=Some(expr::Expr::Variable(expr::Variable {uuid: uuid_next(),name: self.previous(),}));
    }
    self.consume(token::TokenType::LeftBrace, "Expect '{' before class body.".to_string(), lox_obj)?;

    let mut methods:Vec<stmt::Stmt>=Vec::new();
    while !self.check(token::TokenType::RightBrace) && !self.is_at_end(){
        methods.push(self.function("method".to_string(), lox_obj)?);
    }
    self.consume(token::TokenType::RightBrace, "Expect '}' after class body.".to_string(), lox_obj)?;
    Ok(stmt::Stmt::Class(stmt::Class { name: name, methods: methods,super_class:superclass }))

  }

  fn function(&mut self,kind:String,lox_obj:&mut lox::Lox)->Result<stmt::Stmt,ParseError>{
    let name=self.consume(token::TokenType::Identifier, "Expect ".to_string()+&kind+&" name".to_string(), lox_obj)?;
    self.consume(token::TokenType::LeftParen, "Expect ".to_string()+&kind+&" name".to_string(), lox_obj)?;
    let mut parameters:Vec<token::Token>=Vec::new();
    if !self.check(token::TokenType::RightParen){
        loop {
            if parameters.len()>=255{
                let p=self.peek();
                self.error_non_throw(p, "Can't have more than 255 parameters.".to_string(), lox_obj);
            }
            parameters.push(self.consume(token::TokenType::Identifier, "Expect parameter name.".to_string(), lox_obj)?);
            if !self.match_(vec![token::TokenType::Comma]){
                break;
            }               
        }  
         
    }
    self.consume(token::TokenType::RightParen, "Expect ')' after parameters.".to_string(), lox_obj)?;   
    self.consume(token::TokenType::LeftBrace, "Expect '{' before ".to_string()+&kind+&" body.".to_string(), lox_obj)?;
    let body=self.block(lox_obj)?;
    return Ok(stmt::Stmt::Function(stmt::Function { name: name, params: parameters, body: body }));
  }

  fn var_declaration(&mut self,lox_obj:&mut lox::Lox)->Result<stmt::Stmt,ParseError>{
    let name:token::Token=self.consume(token::TokenType::Identifier, "Expect variable name.".to_string(), lox_obj)?;
    let mut initializer=expr::Expr::Literal(expr::Literal{uuid:uuid_next(),value:token::Literals::Nil});
    if self.match_(vec![token::TokenType::Equal]){
        initializer=self.expression(lox_obj)?;
    }
    self.consume(token::TokenType::Semicolon, "Expect ';' after variable declaration.".to_string(), lox_obj)?;
    Ok(stmt::Stmt::Var(stmt::Var { name: name, initializer: initializer }))
  }


  fn statement(&mut self,lox_obj:&mut lox::Lox)->Result<stmt::Stmt,ParseError> {
    
    if self.match_(vec![token::TokenType::For]){
        return self.for_statement(lox_obj);
    }

    if self.match_(vec![token::TokenType::If]){
        return self.if_statement(lox_obj);
    }
    if self.match_(vec![token::TokenType::Print]){        
      return self.print_statement(lox_obj);
    }
    if self.match_(vec![token::TokenType::Return]){
        return self.return_statement(lox_obj);
    }
    if self.match_(vec![token::TokenType::While]){
        return self.while_statement(lox_obj);
    }
    if self.match_(vec![token::TokenType::LeftBrace]){
        
        return Ok(stmt::Stmt::Block(stmt::Block{statements:self.block(lox_obj)?}));
    }
    
    
    self.expression_statement(lox_obj)
}

fn return_statement(&mut self,lox_obj:&mut lox::Lox)->Result<stmt::Stmt,ParseError>{
    let keyword=self.previous();
    let mut value:Option<expr::Expr>=None;
    if !self.check(token::TokenType::Semicolon){
        value=Some(self.expression(lox_obj)?);
    }
    self.consume(token::TokenType::Semicolon, "Expect ';' after return value.".to_string(), lox_obj)?;
    return Ok(stmt::Stmt::Return(stmt::Return { keyword: keyword, value: value}));
}

fn for_statement(&mut self,lox_obj:&mut lox::Lox)->Result<stmt::Stmt,ParseError>{
    self.consume(token::TokenType::LeftParen, "Expect '(' after 'for'.".to_string(), lox_obj)?;
    let  initializer:Option<stmt::Stmt>;
    if self.match_(vec![token::TokenType::Semicolon]){
        initializer=None;
    }else if self.match_(vec![token::TokenType::Var]){
        initializer=Some(self.var_declaration(lox_obj)?);
    }else{
        initializer=Some(self.expression_statement(lox_obj)?);
    }
    let mut condition:Option<expr::Expr>=None;
    if !self.check(token::TokenType::Semicolon){
        condition=Some(self.expression(lox_obj)?);
    }
    self.consume(token::TokenType::Semicolon, "Expect ';' after loop condition.".to_string(), lox_obj)?;
    let mut increment:Option<expr::Expr>=None;
    if !self.check(token::TokenType::RightParen){
        increment=Some(self.expression(lox_obj)?);
    }
    self.consume(token::TokenType::RightParen, "Expect ')' after for clauses.".to_string(), lox_obj)?;
    let mut body=self.statement(lox_obj);
    if increment!=None{
        body=Ok(stmt::Stmt::Block(stmt::Block { statements: vec![body?,
            stmt::Stmt::Expression(stmt::Expression { expression: increment.unwrap() })] }));
    }

    if condition==None{
        condition=Some(expr::Expr::Literal(expr::Literal { uuid:uuid_next(),value: token::Literals::BooleanLit { boolval: true } }));
    }
    body=Ok(stmt::Stmt::While(stmt::While { condition: condition.unwrap(), body: Box::new(body?) }));

    if initializer!=None{
        body=Ok(stmt::Stmt::Block(stmt::Block { statements: vec![initializer.unwrap(),body?] }));
    }

    return body;
}

fn while_statement(&mut self,lox_obj:&mut lox::Lox)->Result<stmt::Stmt,ParseError>{
    self.consume(token::TokenType::LeftParen, "Expect '(' after 'while'.".to_string(), lox_obj)?;
    let condition=self.expression(lox_obj)?;
    self.consume(token::TokenType::RightParen, "Expect ')' after condition.".to_string(), lox_obj)?;
    let body = self.statement(lox_obj)?;
    return Ok(stmt::Stmt::While(stmt::While { condition: condition, body: Box::new(body) }));
}

fn if_statement(&mut self,lox_obj:&mut lox::Lox)->Result<stmt::Stmt,ParseError> {
    self.consume(token::TokenType::LeftParen, "Expect '(' after 'if'.".to_string(), lox_obj)?;
    let condition=self.expression(lox_obj)?;
    self.consume(token::TokenType::RightParen, "Expect ')' after if condition.".to_string(), lox_obj)?;
    let then_branch=self.statement(lox_obj)?;
    let mut else_branch:Option<stmt::Stmt>=None;
    if self.match_(vec![token::TokenType::Else]){
        else_branch=Some(self.statement(lox_obj)?);
    }
    return Ok(stmt::Stmt::If(stmt::If { condition: condition, then_branch: Box::new(then_branch), else_branch: Box::new(else_branch) }));
}
fn print_statement(&mut self,lox_obj:&mut lox::Lox)->Result<stmt::Stmt,ParseError> {
    
    let value:expr::Expr=self.expression(lox_obj)?;
    self.consume(token::TokenType::Semicolon, "Expect ';' after value.".to_string(), lox_obj)?;
    Ok(stmt::Stmt::Print(stmt::Print{expression:value}))
    
        
}
fn expression_statement(&mut self,lox_obj:&mut lox::Lox,)->Result<stmt::Stmt,ParseError> {
    let exp:expr::Expr=self.expression(lox_obj)?;
    self.consume(token::TokenType::Semicolon, "Expect ';' after value.".to_string(), lox_obj)?;
    Ok(stmt::Stmt::Expression(stmt::Expression{expression:exp}))


}
fn block(&mut self,lox_obj:&mut lox::Lox,)->Result<Vec<stmt::Stmt>,ParseError> {
    let mut statements:Vec<stmt::Stmt>=Vec::new();
    while !self.check(token::TokenType::RightBrace) && !self.is_at_end() {
        statements.push(self.declaration(lox_obj)?);     
    }
    
    self.consume(token::TokenType::RightBrace, "Expect '}' after block.".to_string(), lox_obj)?;
    Ok(statements)
}

}