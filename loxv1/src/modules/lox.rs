use std::io::{self,Write,Read};
use std::path::Path;
use std::fs::File;
use crate::modules::lexer;
use crate::modules::token;

pub struct Lox{
   pub had_error:bool,
}


fn read_lines<P>(filename: P) -> io::Result<io::BufReader<File>>
where P: AsRef<Path>, {
    
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}
impl Lox {

    pub  fn new() -> Self {
        Lox { had_error: false, }
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
                None=> println!("{} {} {} {}",i.lexeme,i.line,i.token_type,"NONE"),
            }
           
        }     
    
    }
   
pub fn error(&mut self,  line: usize,message:String){
    self.report(line,message);
}
fn report (&mut self, line: usize,message:String){
    eprintln!("[line {}] Error : {}",line,message);
    self.had_error=true;
    
}

  
    
}
