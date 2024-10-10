use std::env;
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() > 1 {
        println!("Usage: lox [script]");
        std::process::exit(1);
    } else if args.len() == 1 {
        println!("File");
    } else {
        println!("Running");
    }
}
