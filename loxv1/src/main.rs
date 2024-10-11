mod modules;
use crate::modules::core;
use std::env;
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() > 2 {
        println!("Usage: lox [FilePath]");
        std::process::exit(1);
    } else if args.len() == 2 {
        core::run_file(&args[1]);
    } else {
        
        core::run_prompt();
    }
    
}
