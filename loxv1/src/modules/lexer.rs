use std::collections::HashMap;
use crate::modules::token;
use crate::modules::lox;

pub struct Scanner{
        keywords:HashMap<String, token::TokenType>,
        source:String,
       pub tokens:Vec<token::Token>,
        start:usize,
        current:usize,
       pub line:usize,
        
    }
impl Scanner {

    pub fn new(source_:String)->Scanner{
       
        Scanner{            
            keywords:HashMap::from([(String::from("and"),token::TokenType::And),
                                    (String::from("class"),token::TokenType::Class),
                                    (String::from("else"),token::TokenType::Else),
                                    (String::from("false"),token::TokenType::False),
                                    (String::from("for"),token::TokenType::For),
                                    (String::from("fun"),token::TokenType::Fun),
                                    (String::from("if"),token::TokenType::If),
                                    (String::from("nil"),token::TokenType::Nil),
                                    (String::from("or"),token::TokenType::Or),
                                    (String::from("print"),token::TokenType::Print),
                                    (String::from("return"),token::TokenType::Return),
                                    (String::from("super"),token::TokenType::Super),
                                    (String::from("this"),token::TokenType::This),
                                    (String::from("true"),token::TokenType::True),
                                    (String::from("var"),token::TokenType::Var),
                                    (String::from("while"),token::TokenType::While),
                                    ]),
            source:source_,
            tokens:Vec::new(),
            start:0,
            current:0,
            line:1,
            
        }
    }
   pub fn scan_tokens(&mut self,loxobj:&mut lox::Lox) {

        while !(self.is_at_end()) {
            self.start=self.current;
            self.scan_token(loxobj);
        }
       self.tokens.push(token::Token{token_type:token::TokenType::Eof,lexeme:String::new(),literal:None,line:self.line});
        
    }

    fn is_at_end(&mut self)->bool{
       self.current  >=self.source.chars().count()
    }

    fn scan_token(&mut self,loxobj:&mut lox::Lox){

         let c: char  =self.advance();
        match c {
            '(' => self.add_token(token::TokenType::LeftParen),
            ')' => self.add_token(token::TokenType::RightParen),
            '{' => self.add_token(token::TokenType::LeftBrace),
            '}' => self.add_token(token::TokenType::RightBrace),            
            ',' => self.add_token(token::TokenType::Comma),
            '.' => self.add_token(token::TokenType::Dot),
            '-' => self.add_token(token::TokenType::Minus),
            '+' => self.add_token(token::TokenType::Plus),
            ';' => self.add_token(token::TokenType::Semicolon),
            '*' => self.add_token(token::TokenType::Star),
            '!' => { if self.match_ahead('='){self.add_token(token::TokenType::BangEqual)}else{self.add_token(token::TokenType::Bang)}},
            '=' =>{ if self.match_ahead('='){self.add_token(token::TokenType::EqualEqual)}else{self.add_token(token::TokenType::Equal)}},
            '<' => { if self.match_ahead('='){self.add_token(token::TokenType::LesserEqual)}else{self.add_token(token::TokenType::Lesser)}},
            '>' => { if self.match_ahead('='){self.add_token(token::TokenType::GreaterEqual)}else{self.add_token(token::TokenType::Greater)}},
            '/' => {
                     if self.match_ahead('/'){
                        while self.peek()!='\n' && !(self.is_at_end()) {
                            self.advance();
                        }
                     }
                     else{
                        self.add_token(token::TokenType::Slash);
                     }

            }
            ' '|'\r'|'\t' =>{},
            '\n'=> self.line+=1,
            '"' => self.string_token(loxobj),
             _ => {

                if c.is_numeric(){
                    self.number_token();
                }
                else if c.is_alphanumeric() || c=='_' {
                    self.identifier();
                }
                else{
                    loxobj.error(self.line,"Unexpected character".to_string())
                    }     
            
            }
        }
    }
    fn advance(&mut self)->char{
        self.current+=1;
        self.source.chars().nth(self.current-1).unwrap()
    }
    fn add_token(&mut self,token_type:token::TokenType,){
        self.add_token_(token_type,None)
    }
    fn add_token_(&mut self,token_type_:token::TokenType,literal_ :Option<token::Literals>){

        self.tokens.push(token::Token{token_type:token_type_,
            lexeme:self.source.chars().skip(self.start).take(self.current-self.start).collect::<String>(),
            literal:literal_,
            line:self.line});
    }

    fn match_ahead(&mut self,c:char)->bool{
        if self.is_at_end(){
            return false} ;
        if self.source.chars().nth(self.current).unwrap()!= c{
            return false;
        }
        self.current+=1;
        true
    }
    fn peek(&mut self)->char {
        if self.is_at_end(){ return '\0'}
        self.source.chars().nth(self.current).unwrap()
    }
    fn string_token(&mut self,loxobj:&mut lox::Lox){
        while self.peek()!='"' && !(self.is_at_end()) {
            if self.peek() == '\n'{self.line+=1;} 
                self.advance();
        }
        if self.is_at_end(){
            loxobj.error(self.line,"Unterminated string.".to_string())
        }
        self.advance();

        self.add_token_(token::TokenType::String,
            Some(token::Literals::StringLit{stringval:self.source.chars().skip(self.start+1).take(self.current-self.start-2).collect::<String>()}));
        
    }

    fn number_token(&mut self){
        while self.peek().is_numeric() { self.advance();            
        }
        if self.peek()=='.'&& self.peek_next().is_numeric(){
            self.advance();
            while self.peek().is_numeric() {
                self.advance();
            }
        }
       
        
        self.add_token_(token::TokenType::Number,
            Some(token::Literals::NumLit{numval:(self.source.chars().skip(self.start).take(self.current-self.start).collect::<String>()).parse::<f64>().unwrap()}));
         
    }
    fn peek_next(&mut self)->char {
        if self.current+1>=self.source.len()
        { return '\0';}
        self.source.chars().nth(self.current+1).unwrap()

    }
    fn identifier(&mut self){
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let text=self.source.chars().skip(self.start).take(self.current-self.start).collect::<String>();
        
        let tokentype=self.keywords.get(&text);
        match tokentype {
                Some(x) =>{
                    if let token::TokenType::Nil=x{
                        self.add_token_(token::TokenType::Nil, Some(token::Literals::Nil));
                    }else if let token::TokenType::True=x{
                        self.add_token_(token::TokenType::True, Some(token::Literals::BooleanLit { boolval: true }));
                    }
                    else if let token::TokenType::False=x{
                        self.add_token_(token::TokenType::False, Some(token::Literals::BooleanLit { boolval: false }));
                    }
                    else{
                        self.add_token(*x);
                    }
                    
                   }
               ,
            _ =>self.add_token(token::TokenType::Identifier)
        }
    }
}

