use std::any::Any;
#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    // Literals.
    Identifier, String, Number,
    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,
    Eof
    }

   pub struct Token{
    pub  token_type:TokenType,
    pub  lexeme:String,
    pub  literal:Option<Box<dyn Any>>,
    pub   line:usize,
        
    }

    impl Token {
        pub fn new(token_type_:TokenType,lexeme_:String,literal_:Option<Box<dyn Any>>,line_:usize)->Token{
            Token { token_type: token_type_, lexeme: lexeme_, literal: literal_, line: line_ }
        }
        // pub fn to_string(self) ->String{
        //     //self.ty
        // }
    }