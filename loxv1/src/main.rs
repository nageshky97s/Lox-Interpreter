mod modules;
use crate::modules::lox;
use std::env;
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut interpreter=lox::Lox::new();
    if args.len() > 2 {
       
    } else if args.len() == 2 {
        if &args[0]!="lox"{
            println!("Usage: lox [FilePath]");
            std::process::exit(1);
        }
        interpreter.run_file(&args[1]);
    } else {
        
        interpreter.run_prompt();
    }
    
}
