use std::io::{self,Write,Read};
use std::path::Path;
use std::fs::File;

fn read_lines<P>(filename: P) -> io::Result<io::BufReader<File>>
where P: AsRef<Path>, {
    
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}
pub fn run_file(path:&String) {
    
    if !Path::new(path).exists(){
        println!("The Path {} is not valid",path);
    }
    let mut text = String::new();
    if let Ok(mut lines) = read_lines(&path) {
                
            lines.read_to_string(&mut text).expect("cannot read string");
            println!("{}", text);
        
    }
}
pub fn run_prompt()  {
    println!("Running the Lox Interpreter");    
    print!("> ");
        io::stdout().flush().expect("Flush failed.");
    loop {        
        let mut buffer= String::new();
        io::stdin().read_line(&mut buffer).expect("error: unable to read user input");
        if buffer.is_empty(){break;}
        run(&buffer);
        print!("> ");
        io::stdout().flush().expect("Flush failed.");
    }
}


fn run(_line:&String){
    
}
