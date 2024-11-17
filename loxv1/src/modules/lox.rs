use std::io::{self,Write,Read};
use std::path::Path;
use std::fs::File;
use crate::modules::lexer;
use crate::modules::token;
// use crate::modules::astprinter;
use crate::modules::interpreter;
use super::{parser,stmt};

pub struct Lox{
   pub had_error:bool,
   pub had_runtime_error:bool,
   pub allstatements:Option<Vec<stmt::Stmt>>,
}


fn read_lines<P>(filename: P) -> io::Result<io::BufReader<File>>
where P: AsRef<Path>, {
    
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}
impl Lox {

    pub  fn new() -> Self {
        Lox { had_error: false,had_runtime_error:false,allstatements:None }
    }

    pub fn run_file(&mut self,path:&String) {
    
        if !Path::new(path).exists(){
            println!("The Path {} is not valid",path);
        }
        let mut text = String::new();
        if let Ok(mut lines) = read_lines(&path) {
                    
                lines.read_to_string(&mut text).expect("cannot read string");
                println!("{}", text);
            
        }
        self.run(text);
        if self.had_error{std::process::exit(1);}
        if self.had_runtime_error{println!("Runtime Error\n");std::process::exit(1);}
    }
    pub fn run_prompt(&mut self)  {
        println!("Running the Lox Interpreter");    
        print!("> ");
            io::stdout().flush().expect("Flush failed.");
        loop {        
            let mut buffer= String::new();
            io::stdin().read_line(&mut buffer).expect("error: unable to read user input");
            if buffer.is_empty(){break;}
            self.run(buffer);
            self.had_error = false;
            print!("> ");
            io::stdout().flush().expect("Flush failed.");
        }
    }
    
    
    fn run(&mut self,line:String){
    
        let mut scanner=lexer::Scanner::new(line); 
        scanner.scan_tokens(self);   
        
        for i in scanner.tokens.iter(){
            
            match  &i.literal{
                Some(token::Literals::NumLit{numval})=> { 
                    println!("{} {} {} {}",i.lexeme,i.line,i.token_type,numval)
                },
                Some(token::Literals::StringLit{stringval})=> { 
                    println!("{} {} {} {}",i.lexeme,i.line,i.token_type,stringval)
                },
                Some(token::Literals::BooleanLit{boolval})=> { 
                    println!("{} {} {} {}",i.lexeme,i.line,i.token_type,boolval)
                },
                Some(token::Literals::Nil)=>{
                    println!("{} {} {} {}",i.lexeme,i.line,i.token_type,"NULL or NIL TYPE")
                },
                Some(token::Literals::Function { funcval })=>{
                    println!("{} {} {} {} {}",i.lexeme,i.line,i.token_type,"Function Type : {}",funcval);
                },
                None=> println!("{} {} {} {}",i.lexeme,i.line,i.token_type,"NONE"),
            }
           
        }    
        //let expression=expr::Expr::Literal(Box::new(expr::Literal { value: token::Literals::NumLit{numval:45.67} }));
        let mut parser=parser::Parser::new(scanner.tokens);
        // let expression=parser.parse(self);
        
        let statements=parser.parse_new(self);
        
        if self.had_error{
            return;
        }
        
        // let mut debug_printer = astprinter::AstPrinter{};
        // let e=expression.unwrap();
        // debug_printer.print(&e);
       
        if self.had_runtime_error{
            return;
        }                
       
        let mut interpreter=interpreter::Interpreter::new();
        match &mut self.allstatements {
            // Some(x)=>{
            //     x.append(&mut statements);
            //     }
            // None=>{
            //  self.allstatements=Some(statements);
            // }  
            _=>{self.allstatements=Some(statements);}          
        }
        interpreter.interpret_new(self);
    
    
    }
   
pub fn error(&mut self,  line: usize,message:String){
    self.report(line,message);
}
fn report (&mut self, line: usize,message:String){
    eprintln!("[line {}] Error : {}",line,message);
    self.had_error=true;
    
}

pub fn errorp(&mut self,tok:token::Token,message:String){

    if tok.token_type == token::TokenType::Eof{
         
        self.report(tok.line," at end ".to_string() +&message);
    }
    else{
        self.report(tok.line," at '".to_string()+&tok.lexeme+&"'".to_string()+&message);
    }

}

pub fn runtimeerror(&mut self,err:interpreter::RuntimeError){

    println!("{}\n[line {} ]",err.mess,err.tok.line);
    self.had_runtime_error=true;
}

  
    
}
