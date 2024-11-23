use std::fmt;
use crate::modules::loxcallable;
#[derive(Debug, Clone, Copy,PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Lesser, LesserEqual,
    // Literals.
    Identifier, String, Number,
    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,
    Eof
    }
    impl fmt::Display for TokenType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
           match self {
            TokenType::LeftParen => write!(f,"("),
            TokenType::RightParen => write!(f,")"),
            TokenType::LeftBrace => write!(f,"{{"),
            TokenType::RightBrace => write!(f,"}}"),
            TokenType::Comma => write!(f,","),
            TokenType::Dot => write!(f,"."),
            TokenType::Minus => write!(f,"-"),
            TokenType::Plus => write!(f,"+"),
            TokenType::Semicolon => write!(f,";"),
            TokenType::Slash => write!(f,"/"),
            TokenType::Star => write!(f,"*"),
            TokenType::Bang => write!(f,"!"),
            TokenType::BangEqual => write!(f,"!="),
            TokenType::Equal => write!(f,"="),

            TokenType::EqualEqual => write!(f,"=="),
            TokenType::Greater => write!(f,">"),
            TokenType::GreaterEqual => write!(f,">="),
            TokenType::Lesser => write!(f,"<"),
            TokenType::LesserEqual => write!(f,"<="),
            TokenType::Identifier => write!(f,"IDENTIFIER"),
            TokenType::String => write!(f,"STRING"),
            TokenType::Number => write!(f,"NUMBER"),
            TokenType::And => write!(f,"AND"),
            TokenType::Class => write!(f,"CLASS"),
            TokenType::Else => write!(f,"ELSE"),
            TokenType::False => write!(f,"FALSE"),
            TokenType::Fun => write!(f,"FUN"),
            TokenType::For => write!(f,"FOR"),

            TokenType::If => write!(f,"IF"),
            TokenType::Nil => write!(f,"NIL"),
            TokenType::Or => write!(f,"OR"),
            TokenType::Print => write!(f,"PRINT"),
            TokenType::Return => write!(f,"RETURN"),
            TokenType::Super => write!(f,"SUPER"),
            TokenType::This => write!(f,"THIS"),
            TokenType::True => write!(f,"TRUE"),
            TokenType::Var => write!(f,"VAR"),
            TokenType::While => write!(f,"WHILE"),
            TokenType::Eof => write!(f,"EOF"),

           }
        }
    }

#[derive(Debug,Clone,PartialEq)]
pub enum Literals{
    NumLit{numval:f64},
    StringLit{stringval:String},
    BooleanLit{boolval:bool},
    Nil, 
   Callable(loxcallable::Callable),
   
   
}
impl fmt::Display for Literals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
        Self::NumLit { numval }=>write!(f,"{}",numval),
        Self::StringLit { stringval }=>write!(f,"{}",stringval),
        Self::BooleanLit { boolval }=>write!(f,"{}",boolval),
        Self::Nil =>write!(f,"NULL or NIL TYPE"),
        Self::Callable (x)=>write!(f,"Callable name: {}",x),
       }
    }
}

#[derive(Debug,Clone,PartialEq)]
   pub struct Token{
    pub  token_type:TokenType,
    pub  lexeme:String,
    pub  literal:Literals,
    pub   line:usize,
        
    }

   